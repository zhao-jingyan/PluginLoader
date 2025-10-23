use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

/// 音频电平表（线程安全）
#[derive(Clone)]
pub struct LevelMeter {
    peak_left: Arc<AtomicU32>,
    peak_right: Arc<AtomicU32>,
}

impl LevelMeter {
    pub fn new() -> Self {
        Self {
            peak_left: Arc::new(AtomicU32::new(0)),
            peak_right: Arc::new(AtomicU32::new(0)),
        }
    }
    
    /// 更新电平（在音频线程中调用）
    pub fn update(&self, left: f32, right: f32) {
        let left_bits = left.abs().to_bits();
        let right_bits = right.abs().to_bits();
        
        // 使用原子操作更新峰值
        self.peak_left.fetch_max(left_bits, Ordering::Relaxed);
        self.peak_right.fetch_max(right_bits, Ordering::Relaxed);
    }
    
    /// 处理音频缓冲区并更新电平
    pub fn process_buffer(&self, buffer: &[f32]) {
        // 假设立体声交错格式: L, R, L, R, ...
        for chunk in buffer.chunks_exact(2) {
            if chunk.len() >= 2 {
                self.update(chunk[0], chunk[1]);
            }
        }
    }
    
    /// 获取当前峰值电平（在 UI 线程中调用）
    pub fn get_peak_db(&self) -> (f32, f32) {
        let left_bits = self.peak_left.swap(0, Ordering::Relaxed);
        let right_bits = self.peak_right.swap(0, Ordering::Relaxed);
        
        let left = f32::from_bits(left_bits);
        let right = f32::from_bits(right_bits);
        
        (amplitude_to_db(left), amplitude_to_db(right))
    }
    
    /// 获取当前峰值幅度（0.0 - 1.0）
    #[allow(dead_code)]
    pub fn get_peak_amplitude(&self) -> (f32, f32) {
        let left_bits = self.peak_left.swap(0, Ordering::Relaxed);
        let right_bits = self.peak_right.swap(0, Ordering::Relaxed);
        
        let left = f32::from_bits(left_bits);
        let right = f32::from_bits(right_bits);
        
        (left, right)
    }
}

impl Default for LevelMeter {
    fn default() -> Self {
        Self::new()
    }
}

/// 将幅度转换为 dB
fn amplitude_to_db(amplitude: f32) -> f32 {
    if amplitude > 0.0 {
        20.0 * amplitude.log10()
    } else {
        -100.0 // 静音
    }
}

/// 格式化 dB 值显示
pub fn format_db(db: f32) -> String {
    if db <= -99.0 {
        "-∞ dB".to_string()
    } else {
        format!("{:.1} dB", db)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_level_meter() {
        let meter = LevelMeter::new();
        
        // 更新电平
        meter.update(0.5, 0.8);
        
        let (left, right) = meter.get_peak_amplitude();
        assert!((left - 0.5).abs() < 0.001);
        assert!((right - 0.8).abs() < 0.001);
        
        // 第二次读取应该归零
        let (left, right) = meter.get_peak_amplitude();
        assert_eq!(left, 0.0);
        assert_eq!(right, 0.0);
    }
    
    #[test]
    fn test_amplitude_to_db() {
        assert!((amplitude_to_db(1.0) - 0.0).abs() < 0.001);
        assert!((amplitude_to_db(0.5) + 6.02).abs() < 0.1);
        assert!(amplitude_to_db(0.0) < -90.0);
    }
}

