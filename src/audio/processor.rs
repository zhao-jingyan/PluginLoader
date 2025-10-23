use std::sync::{Arc, Mutex};
use log::debug;

use crate::plugin::PluginChain;

/// 音频处理器 - 管理插件链和音频处理
pub struct AudioProcessorEngine {
    plugin_chain: Arc<Mutex<PluginChain>>,
    bypass: bool,
}

impl AudioProcessorEngine {
    pub fn new() -> Self {
        Self {
            plugin_chain: Arc::new(Mutex::new(PluginChain::new())),
            bypass: false,
        }
    }
    
    /// 获取插件链的引用
    pub fn get_plugin_chain(&self) -> Arc<Mutex<PluginChain>> {
        self.plugin_chain.clone()
    }
    
    /// 设置bypass状态
    pub fn set_bypass(&mut self, bypass: bool) {
        self.bypass = bypass;
    }
    
    /// 处理音频缓冲区
    pub fn process_audio(&self, buffer: &mut [f32]) {
        if self.bypass {
            // bypass 模式 - 直通
            return;
        }
        
        // 尝试获取插件链的锁
        if let Ok(mut chain) = self.plugin_chain.try_lock() {
            if !chain.is_empty() {
                // 应用插件链处理
                chain.process(buffer);
            }
        } else {
            // 无法获取锁，直通（避免阻塞音频线程）
            debug!("无法获取插件链锁，跳过处理");
        }
    }
}

impl Default for AudioProcessorEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for AudioProcessorEngine {
    fn clone(&self) -> Self {
        Self {
            plugin_chain: self.plugin_chain.clone(),
            bypass: self.bypass,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_processor_creation() {
        let processor = AudioProcessorEngine::new();
        assert!(!processor.bypass);
    }
    
    #[test]
    fn test_bypass() {
        let mut processor = AudioProcessorEngine::new();
        processor.set_bypass(true);
        assert!(processor.bypass);
        
        // bypass 模式下处理音频应该不修改数据
        let mut buffer = vec![0.5, 0.3, 0.7, 0.2];
        let original = buffer.clone();
        processor.process_audio(&mut buffer);
        assert_eq!(buffer, original);
    }
    
    #[test]
    fn test_empty_chain() {
        let processor = AudioProcessorEngine::new();
        let mut buffer = vec![0.5, 0.3, 0.7, 0.2];
        let original = buffer.clone();
        
        // 空插件链应该不修改数据
        processor.process_audio(&mut buffer);
        assert_eq!(buffer, original);
    }
}

