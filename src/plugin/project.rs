// 工程文件管理
// 保存和加载插件链配置

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use log::info;

use super::types::PluginState;

/// 工程文件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// 工程名称
    pub name: String,
    
    /// 工程版本
    pub version: String,
    
    /// 创建时间
    pub created: String,
    
    /// 最后修改时间
    pub modified: String,
    
    /// 音频配置
    pub audio_config: AudioConfig,
    
    /// 插件链状态
    pub plugin_chain: Vec<PluginState>,
}

/// 音频配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// 采样率
    pub sample_rate: u32,
    
    /// 缓冲区大小
    pub buffer_size: u32,
    
    /// 通道数
    pub channels: u16,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            buffer_size: 256,
            channels: 2,
        }
    }
}

impl Project {
    /// 创建新工程
    pub fn new(name: String) -> Self {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        Self {
            name,
            version: "1.0".to_string(),
            created: now.clone(),
            modified: now,
            audio_config: AudioConfig::default(),
            plugin_chain: Vec::new(),
        }
    }
    
    /// 从文件加载工程
    pub fn load(path: &Path) -> Result<Self> {
        info!("加载工程文件: {:?}", path);
        
        let content = fs::read_to_string(path)
            .context(format!("读取工程文件失败: {:?}", path))?;
        
        let project: Project = serde_json::from_str(&content)
            .context("解析工程文件失败")?;
        
        info!("工程加载成功: {}", project.name);
        Ok(project)
    }
    
    /// 保存工程到文件
    pub fn save(&mut self, path: &Path) -> Result<()> {
        info!("保存工程文件: {:?}", path);
        
        // 更新修改时间
        self.modified = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let content = serde_json::to_string_pretty(self)
            .context("序列化工程文件失败")?;
        
        fs::write(path, content)
            .context(format!("写入工程文件失败: {:?}", path))?;
        
        info!("工程保存成功");
        Ok(())
    }
    
    /// 更新插件链状态
    pub fn update_plugin_chain(&mut self, plugin_chain: Vec<PluginState>) {
        self.plugin_chain = plugin_chain;
    }
    
    /// 获取插件链状态
    pub fn get_plugin_chain(&self) -> &[PluginState] {
        &self.plugin_chain
    }
}

/// 工程管理器
pub struct ProjectManager {
    current_project: Option<Project>,
    current_path: Option<PathBuf>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            current_project: None,
            current_path: None,
        }
    }
    
    /// 创建新工程
    pub fn new_project(&mut self, name: String) -> &mut Project {
        let project = Project::new(name);
        self.current_project = Some(project);
        self.current_path = None;
        self.current_project.as_mut().unwrap()
    }
    
    /// 打开工程
    pub fn open_project(&mut self, path: &Path) -> Result<&Project> {
        let project = Project::load(path)?;
        self.current_path = Some(path.to_path_buf());
        self.current_project = Some(project);
        Ok(self.current_project.as_ref().unwrap())
    }
    
    /// 保存当前工程
    pub fn save_current(&mut self) -> Result<()> {
        if self.current_project.is_none() {
            return Err(anyhow::anyhow!("没有打开的工程"));
        }
        
        if self.current_path.is_none() {
            return Err(anyhow::anyhow!("工程路径未设置，请使用 save_as"));
        }
        
        let path = self.current_path.as_ref().unwrap().clone();
        self.current_project.as_mut().unwrap().save(&path)
    }
    
    /// 另存为
    pub fn save_as(&mut self, path: &Path) -> Result<()> {
        if self.current_project.is_none() {
            return Err(anyhow::anyhow!("没有打开的工程"));
        }
        
        self.current_path = Some(path.to_path_buf());
        self.current_project.as_mut().unwrap().save(path)
    }
    
    /// 获取当前工程
    pub fn get_current_project(&self) -> Option<&Project> {
        self.current_project.as_ref()
    }
    
    /// 获取当前工程（可修改）
    pub fn get_current_project_mut(&mut self) -> Option<&mut Project> {
        self.current_project.as_mut()
    }
    
    /// 关闭当前工程
    pub fn close_project(&mut self) {
        self.current_project = None;
        self.current_path = None;
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_project_creation() {
        let project = Project::new("Test Project".to_string());
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.version, "1.0");
        assert!(project.plugin_chain.is_empty());
    }
    
    #[test]
    fn test_project_save_load() {
        let temp_dir = env::temp_dir();
        let test_path = temp_dir.join("test_project.json");
        
        // 创建并保存工程
        let mut project = Project::new("Save Test".to_string());
        project.save(&test_path).unwrap();
        
        // 加载工程
        let loaded = Project::load(&test_path).unwrap();
        assert_eq!(loaded.name, "Save Test");
        
        // 清理
        let _ = fs::remove_file(test_path);
    }
    
    #[test]
    fn test_project_manager() {
        let mut manager = ProjectManager::new();
        
        // 创建新工程
        manager.new_project("Manager Test".to_string());
        assert!(manager.get_current_project().is_some());
        assert_eq!(manager.get_current_project().unwrap().name, "Manager Test");
        
        // 关闭工程
        manager.close_project();
        assert!(manager.get_current_project().is_none());
    }
}

