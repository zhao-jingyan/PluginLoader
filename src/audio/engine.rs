use anyhow::{Result, Context};
use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Stream, StreamConfig, SampleFormat};
use log::{info, error};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::{self, Write};
use std::time::Duration;

use super::device::{get_default_input_device, get_default_output_device, list_audio_devices};
use super::level_meter::{LevelMeter, format_db};

/// 音频引擎主函数
pub fn run_audio_engine() -> Result<()> {
    // 1. 列出所有音频设备
    list_audio_devices()?;
    
    println!();
    info!("=== 启动音频引擎 ===");
    
    // 2. 获取默认设备
    let input_device = get_default_input_device()?;
    let output_device = get_default_output_device()?;
    
    info!("输入设备: {}", input_device.name()?);
    info!("输出设备: {}", output_device.name()?);
    
    // 3. 配置音频流
    let (config, sample_format) = get_audio_config(&input_device)?;
    
    // 获取实际缓冲区大小（用于显示）
    let buffer_size_str = match config.buffer_size {
        cpal::BufferSize::Fixed(size) => format!("{} samples", size),
        cpal::BufferSize::Default => "设备默认".to_string(),
    };
    
    info!("音频配置: {} Hz, {} 通道, 缓冲区: {}",
        config.sample_rate.0,
        config.channels,
        buffer_size_str
    );
    
    // 如果是固定缓冲区，计算理论延迟
    if let cpal::BufferSize::Fixed(size) = config.buffer_size {
        let latency_ms = (size as f32 / config.sample_rate.0 as f32) * 1000.0;
        info!("理论延迟: {:.2} ms", latency_ms);
    }
    
    // 4. 创建电平表
    let level_meter = LevelMeter::new();
    let level_meter_clone = level_meter.clone();
    
    // 5. 创建停止标志
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    // 设置 Ctrl+C 处理
    ctrlc::set_handler(move || {
        info!("收到停止信号...");
        running_clone.store(false, Ordering::Relaxed);
    }).context("设置 Ctrl+C 处理器失败")?;
    
    // 6. 构建音频流
    let streams = build_audio_streams(
        &input_device,
        &output_device,
        &config,
        sample_format,
        level_meter_clone,
    )?;
    
    info!("✅ 音频引擎启动成功！");
    info!("提示: 按 Ctrl+C 停止");
    println!();
    
    // 7. 主循环 - 显示电平表
    run_level_meter_display(&level_meter, &running)?;
    
    // 8. 停止音频流
    drop(streams);
    info!("音频引擎已停止");
    
    Ok(())
}

/// 获取音频配置
fn get_audio_config(device: &cpal::Device) -> Result<(StreamConfig, SampleFormat)> {
    let default_config = device.default_input_config()
        .context("获取默认音频配置失败")?;
    
    let sample_format = default_config.sample_format();
    let config: StreamConfig = default_config.into();
    
    // 使用设备默认的缓冲区大小，不强制设置
    // 这样可以确保兼容性
    
    Ok((config, sample_format))
}

/// 构建输入和输出音频流
fn build_audio_streams(
    input_device: &cpal::Device,
    output_device: &cpal::Device,
    config: &StreamConfig,
    sample_format: SampleFormat,
    level_meter: LevelMeter,
) -> Result<(Stream, Stream)> {
    
    // 使用环形缓冲区在输入和输出之间传递音频数据
    let ring_buffer = ringbuf::HeapRb::<f32>::new(
        config.sample_rate.0 as usize * 2 // 2 秒的缓冲
    );
    let (producer, mut consumer) = ring_buffer.split();
    
    let producer = Arc::new(std::sync::Mutex::new(producer));
    
    // 构建输入流
    let input_producer = producer.clone();
    let input_level_meter = level_meter.clone();
    
    let input_stream = match sample_format {
        SampleFormat::F32 => {
            input_device.build_input_stream(
                config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    // 更新电平表
                    input_level_meter.process_buffer(data);
                    
                    // 写入环形缓冲区
                    if let Ok(mut prod) = input_producer.lock() {
                        for &sample in data {
                            let _ = prod.push(sample);
                        }
                    }
                },
                |err| error!("输入流错误: {}", err),
                None,
            )?
        }
        SampleFormat::I16 => {
            input_device.build_input_stream(
                config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut prod) = input_producer.lock() {
                        for &sample in data {
                            let normalized = sample as f32 / i16::MAX as f32;
                            let _ = prod.push(normalized);
                        }
                    }
                },
                |err| error!("输入流错误: {}", err),
                None,
            )?
        }
        SampleFormat::U16 => {
            input_device.build_input_stream(
                config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut prod) = input_producer.lock() {
                        for &sample in data {
                            let normalized = (sample as f32 / u16::MAX as f32) * 2.0 - 1.0;
                            let _ = prod.push(normalized);
                        }
                    }
                },
                |err| error!("输入流错误: {}", err),
                None,
            )?
        }
        _ => return Err(anyhow::anyhow!("不支持的音频格式")),
    };
    
    // 构建输出流
    let output_stream = match sample_format {
        SampleFormat::F32 => {
            output_device.build_output_stream(
                config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for sample in data.iter_mut() {
                        *sample = consumer.pop().unwrap_or(0.0);
                    }
                },
                |err| error!("输出流错误: {}", err),
                None,
            )?
        }
        SampleFormat::I16 => {
            output_device.build_output_stream(
                config,
                move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                    for sample in data.iter_mut() {
                        let value = consumer.pop().unwrap_or(0.0);
                        *sample = (value * i16::MAX as f32) as i16;
                    }
                },
                |err| error!("输出流错误: {}", err),
                None,
            )?
        }
        SampleFormat::U16 => {
            output_device.build_output_stream(
                config,
                move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                    for sample in data.iter_mut() {
                        let value = consumer.pop().unwrap_or(0.0);
                        *sample = ((value + 1.0) / 2.0 * u16::MAX as f32) as u16;
                    }
                },
                |err| error!("输出流错误: {}", err),
                None,
            )?
        }
        _ => return Err(anyhow::anyhow!("不支持的音频格式")),
    };
    
    // 启动音频流
    input_stream.play()?;
    output_stream.play()?;
    
    Ok((input_stream, output_stream))
}

/// 运行电平表显示
fn run_level_meter_display(level_meter: &LevelMeter, running: &Arc<AtomicBool>) -> Result<()> {
    let mut update_counter = 0;
    
    while running.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_millis(100));
        
        let (left_db, right_db) = level_meter.get_peak_db();
        
        // 每秒更新 10 次
        if update_counter % 1 == 0 {
            print!("\r🎸 电平: L: {} | R: {}    ",
                format_db_bar(left_db),
                format_db_bar(right_db)
            );
            io::stdout().flush()?;
        }
        
        update_counter += 1;
    }
    
    println!(); // 换行
    Ok(())
}

/// 格式化电平为可视化条形图
fn format_db_bar(db: f32) -> String {
    let normalized = if db <= -60.0 {
        0.0
    } else if db >= 0.0 {
        1.0
    } else {
        (db + 60.0) / 60.0
    };
    
    let bar_length = (normalized * 20.0) as usize;
    let bar = "█".repeat(bar_length);
    let empty = "░".repeat(20 - bar_length);
    
    format!("{}{} {}", bar, empty, format_db(db))
}

