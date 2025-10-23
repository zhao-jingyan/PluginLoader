use anyhow::Result;
use log::info;
use std::path::Path;

use super::types::{PluginMetadata, AudioProcessor};
use super::au_wrapper::AudioUnitPlugin;
use super::scanner::PluginInfo;

/// Audio Unit 插件加载器
pub struct PluginLoader {
    // 缓存已加载的插件信息
    loaded_plugins: Vec<String>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            loaded_plugins: Vec::new(),
        }
    }
    
    /// 从路径加载插件
    pub fn load_plugin(&mut self, path: &Path) -> Result<Box<dyn AudioProcessor>> {
        info!("加载插件: {:?}", path);
        
        // 使用 AudioUnitPlugin 包装器加载
        let plugin = AudioUnitPlugin::load(path)?;
        
        // 记录已加载
        self.loaded_plugins.push(plugin.get_info().id.clone());
        
        Ok(Box::new(plugin))
    }
    
    /// 从插件信息加载
    pub fn load_from_info(&mut self, info: &PluginInfo) -> Result<Box<dyn AudioProcessor>> {
        if !info.valid {
            return Err(anyhow::anyhow!("插件无效: {}", info.error.as_ref().unwrap_or(&"未知错误".to_string())));
        }
        
        self.load_plugin(&info.metadata.path)
    }
    
    /// 从元数据加载
    pub fn load_from_metadata(&mut self, metadata: &PluginMetadata) -> Result<Box<dyn AudioProcessor>> {
        self.load_plugin(&metadata.path)
    }
    
    /// 卸载插件
    pub fn unload_plugin(&mut self, plugin: Box<dyn AudioProcessor>) {
        let id = plugin.get_info().id.clone();
        info!("卸载插件: {}", id);
        
        // 从已加载列表中移除
        if let Some(pos) = self.loaded_plugins.iter().position(|x| x == &id) {
            self.loaded_plugins.remove(pos);
        }
        
        // plugin 会在这里自动 drop，触发清理
        drop(plugin);
    }
    
    /// 获取已加载插件列表
    pub fn get_loaded_plugins(&self) -> &[String] {
        &self.loaded_plugins
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// 模拟插件（用于测试）
pub struct DummyPlugin {
    metadata: PluginMetadata,
}

impl DummyPlugin {
    pub fn new(metadata: PluginMetadata) -> Self {
        Self { metadata }
    }
}

impl AudioProcessor for DummyPlugin {
    fn process(&mut self, _buffer: &mut [f32]) {
        // Bypass - 不做任何处理
    }
    
    fn get_info(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn set_parameter(&mut self, _id: u32, _value: f64) {
        // 空实现
    }
    
    fn get_parameter(&self, _id: u32) -> Option<f64> {
        None
    }
    
    fn get_all_parameters(&self) -> Vec<super::types::PluginParameter> {
        Vec::new()
    }
    
    fn save_state(&self) -> super::types::PluginState {
        super::types::PluginState {
            plugin_id: self.metadata.id.clone(),
            parameters: Vec::new(),
            state_data: String::new(),
        }
    }
    
    fn load_state(&mut self, _state: &super::types::PluginState) {
        // 空实现
    }
}

