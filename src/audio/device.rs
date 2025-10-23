use anyhow::{Result, Context};
use cpal::traits::{DeviceTrait, HostTrait};
use log::{info, warn};

/// 音频设备配置
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AudioDeviceConfig {
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub channels: u16,
}

impl Default for AudioDeviceConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            buffer_size: 256,
            channels: 2,
        }
    }
}

/// 列出所有可用的音频设备
pub fn list_audio_devices() -> Result<()> {
    let host = cpal::default_host();
    
    info!("=== 音频设备列表 ===");
    
    // 默认输入设备
    match host.default_input_device() {
        Some(device) => {
            match device.name() {
                Ok(name) => {
                    info!("📥 默认输入设备: {}", name);
                    let _ = print_device_info(&device); // 忽略设备信息错误
                }
                Err(e) => warn!("无法获取输入设备名称: {}", e),
            }
        }
        None => warn!("未找到默认输入设备"),
    }
    
    println!();
    
    // 默认输出设备
    match host.default_output_device() {
        Some(device) => {
            match device.name() {
                Ok(name) => {
                    info!("📤 默认输出设备: {}", name);
                    let _ = print_device_info(&device); // 忽略设备信息错误
                }
                Err(e) => warn!("无法获取输出设备名称: {}", e),
            }
        }
        None => warn!("未找到默认输出设备"),
    }
    
    println!();
    
    // 列出所有输入设备
    info!("所有输入设备:");
    match host.input_devices() {
        Ok(devices) => {
            for (idx, device) in devices.enumerate() {
                let name = device.name().unwrap_or_else(|_| "未知设备".to_string());
                info!("  {}. {}", idx + 1, name);
            }
        }
        Err(e) => warn!("无法枚举输入设备: {}", e),
    }
    
    println!();
    
    // 列出所有输出设备
    info!("所有输出设备:");
    match host.output_devices() {
        Ok(devices) => {
            for (idx, device) in devices.enumerate() {
                let name = device.name().unwrap_or_else(|_| "未知设备".to_string());
                info!("  {}. {}", idx + 1, name);
            }
        }
        Err(e) => warn!("无法枚举输出设备: {}", e),
    }
    
    Ok(())
}

/// 打印设备详细信息
fn print_device_info(device: &cpal::Device) -> Result<()> {
    // 支持的配置（输入）
    match device.supported_input_configs() {
        Ok(configs) => {
            info!("  支持的配置:");
            for config in configs.take(3) {
                info!("    - 采样率: {} - {} Hz, 通道数: {}", 
                    config.min_sample_rate().0,
                    config.max_sample_rate().0,
                    config.channels()
                );
            }
        }
        Err(_) => {
            // 可能是输出设备，尝试输出配置
            if let Ok(configs) = device.supported_output_configs() {
                info!("  支持的配置:");
                for config in configs.take(3) {
                    info!("    - 采样率: {} - {} Hz, 通道数: {}", 
                        config.min_sample_rate().0,
                        config.max_sample_rate().0,
                        config.channels()
                    );
                }
            }
        }
    }
    
    // 默认配置（尝试输入，失败则尝试输出）
    if let Ok(config) = device.default_input_config() {
        info!("  默认配置: {} Hz, {} 通道, {:?}", 
            config.sample_rate().0,
            config.channels(),
            config.sample_format()
        );
    } else if let Ok(config) = device.default_output_config() {
        info!("  默认配置: {} Hz, {} 通道, {:?}", 
            config.sample_rate().0,
            config.channels(),
            config.sample_format()
        );
    }
    
    Ok(())
}

/// 获取默认输入设备
pub fn get_default_input_device() -> Result<cpal::Device> {
    let host = cpal::default_host();
    host.default_input_device()
        .context("未找到默认输入设备")
}

/// 获取默认输出设备
pub fn get_default_output_device() -> Result<cpal::Device> {
    let host = cpal::default_host();
    host.default_output_device()
        .context("未找到默认输出设备")
}

