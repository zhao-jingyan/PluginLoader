use anyhow::Result;
use log::{info, debug};

use super::types::{AudioProcessor, PluginState};

/// 插件串联链
pub struct PluginChain {
    plugins: Vec<Box<dyn AudioProcessor>>,
    max_plugins: usize,
}

impl PluginChain {
    /// 创建新的插件链
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            max_plugins: 8, // 最多支持 8 个插件
        }
    }
    
    /// 添加插件到链的末尾
    pub fn add_plugin(&mut self, plugin: Box<dyn AudioProcessor>) -> Result<()> {
        if self.plugins.len() >= self.max_plugins {
            return Err(anyhow::anyhow!("插件链已满（最多 {} 个）", self.max_plugins));
        }
        
        let name = plugin.get_info().name.clone();
        self.plugins.push(plugin);
        info!("添加插件到链: {}", name);
        
        Ok(())
    }
    
    /// 在指定位置插入插件
    pub fn insert_plugin(&mut self, index: usize, plugin: Box<dyn AudioProcessor>) -> Result<()> {
        if self.plugins.len() >= self.max_plugins {
            return Err(anyhow::anyhow!("插件链已满（最多 {} 个）", self.max_plugins));
        }
        
        if index > self.plugins.len() {
            return Err(anyhow::anyhow!("索引超出范围"));
        }
        
        let name = plugin.get_info().name.clone();
        self.plugins.insert(index, plugin);
        info!("在位置 {} 插入插件: {}", index, name);
        
        Ok(())
    }
    
    /// 移除指定位置的插件
    pub fn remove_plugin(&mut self, index: usize) -> Result<Box<dyn AudioProcessor>> {
        if index >= self.plugins.len() {
            return Err(anyhow::anyhow!("索引超出范围"));
        }
        
        let plugin = self.plugins.remove(index);
        info!("移除插件: {}", plugin.get_info().name);
        
        Ok(plugin)
    }
    
    /// 移动插件到新位置
    pub fn move_plugin(&mut self, from: usize, to: usize) -> Result<()> {
        if from >= self.plugins.len() || to >= self.plugins.len() {
            return Err(anyhow::anyhow!("索引超出范围"));
        }
        
        let plugin = self.plugins.remove(from);
        self.plugins.insert(to, plugin);
        info!("移动插件从 {} 到 {}", from, to);
        
        Ok(())
    }
    
    /// 清空插件链
    pub fn clear(&mut self) {
        self.plugins.clear();
        info!("清空插件链");
    }
    
    /// 获取插件数量
    pub fn len(&self) -> usize {
        self.plugins.len()
    }
    
    /// 判断插件链是否为空
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
    
    /// 处理音频（串联所有插件）
    pub fn process(&mut self, buffer: &mut [f32]) {
        if self.plugins.is_empty() {
            // 没有插件，直接 bypass
            return;
        }
        
        // 顺序处理每个插件
        for plugin in &mut self.plugins {
            plugin.process(buffer);
        }
    }
    
    /// 获取所有插件的名称
    pub fn get_plugin_names(&self) -> Vec<String> {
        self.plugins
            .iter()
            .map(|p| p.get_info().name.clone())
            .collect()
    }
    
    /// 保存插件链状态
    pub fn save_chain_state(&self) -> Vec<PluginState> {
        self.plugins
            .iter()
            .map(|p| p.save_state())
            .collect()
    }
    
    /// 加载插件链状态
    pub fn load_chain_state(&mut self, states: &[PluginState]) {
        if states.len() != self.plugins.len() {
            log::warn!("状态数量与插件数量不匹配");
            return;
        }
        
        for (plugin, state) in self.plugins.iter_mut().zip(states.iter()) {
            plugin.load_state(state);
        }
        
        info!("加载了 {} 个插件的状态", states.len());
    }
}

impl Default for PluginChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::loader::DummyPlugin;
    use crate::plugin::types::{PluginMetadata, PluginFormat};
    use std::path::PathBuf;
    
    fn create_dummy_plugin(name: &str) -> Box<dyn AudioProcessor> {
        Box::new(DummyPlugin::new(PluginMetadata {
            id: name.to_string(),
            name: name.to_string(),
            vendor: "Test".to_string(),
            version: "1.0".to_string(),
            path: PathBuf::from("/test"),
            format: PluginFormat::AudioUnit,
            num_inputs: 2,
            num_outputs: 2,
        }))
    }
    
    #[test]
    fn test_add_plugin() {
        let mut chain = PluginChain::new();
        assert_eq!(chain.len(), 0);
        
        chain.add_plugin(create_dummy_plugin("Plugin1")).unwrap();
        assert_eq!(chain.len(), 1);
        
        chain.add_plugin(create_dummy_plugin("Plugin2")).unwrap();
        assert_eq!(chain.len(), 2);
    }
    
    #[test]
    fn test_max_plugins() {
        let mut chain = PluginChain::new();
        
        // 添加 8 个插件（最大值）
        for i in 0..8 {
            chain.add_plugin(create_dummy_plugin(&format!("Plugin{}", i))).unwrap();
        }
        
        // 第 9 个应该失败
        assert!(chain.add_plugin(create_dummy_plugin("Plugin9")).is_err());
    }
    
    #[test]
    fn test_remove_plugin() {
        let mut chain = PluginChain::new();
        chain.add_plugin(create_dummy_plugin("Plugin1")).unwrap();
        chain.add_plugin(create_dummy_plugin("Plugin2")).unwrap();
        
        let removed = chain.remove_plugin(0).unwrap();
        assert_eq!(removed.get_info().name, "Plugin1");
        assert_eq!(chain.len(), 1);
    }
}

