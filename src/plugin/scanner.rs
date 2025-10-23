use anyhow::{Result, Context};
use log::{info, warn, debug};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

use super::types::{PluginMetadata, PluginFormat};

/// 插件信息（用于扫描结果）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub metadata: PluginMetadata,
    pub valid: bool,
    pub error: Option<String>,
}

/// Audio Unit 插件扫描器
pub struct PluginScanner {
    cache_file: PathBuf,
}

impl PluginScanner {
    /// 创建新的扫描器
    pub fn new() -> Self {
        Self {
            cache_file: PathBuf::from("plugin_cache.json"),
        }
    }
    
    /// 扫描所有 Audio Unit 插件
    pub fn scan_all(&self) -> Result<Vec<PluginInfo>> {
        info!("开始扫描 Audio Unit 插件...");
        
        let mut all_plugins = Vec::new();
        
        // macOS Audio Unit 标准目录
        let search_paths = vec![
            PathBuf::from("/Library/Audio/Plug-Ins/Components"),
            PathBuf::from(format!("{}/Library/Audio/Plug-Ins/Components", 
                std::env::var("HOME").unwrap_or_default())),
        ];
        
        for path in search_paths {
            if path.exists() {
                info!("扫描目录: {:?}", path);
                match self.scan_directory(&path) {
                    Ok(plugins) => {
                        info!("  找到 {} 个插件", plugins.len());
                        all_plugins.extend(plugins);
                    }
                    Err(e) => {
                        warn!("扫描目录失败 {:?}: {}", path, e);
                    }
                }
            } else {
                debug!("目录不存在: {:?}", path);
            }
        }
        
        info!("扫描完成，共找到 {} 个插件", all_plugins.len());
        
        // 保存到缓存
        self.save_cache(&all_plugins)?;
        
        Ok(all_plugins)
    }
    
    /// 扫描指定目录
    fn scan_directory(&self, path: &Path) -> Result<Vec<PluginInfo>> {
        let mut plugins = Vec::new();
        
        let entries = fs::read_dir(path)
            .context(format!("读取目录失败: {:?}", path))?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            // Audio Unit 插件是 .component bundle（macOS 上是目录）
            if path.extension().and_then(|s| s.to_str()) == Some("component") {
                debug!("发现插件: {:?}", path);
                
                match self.scan_plugin(&path) {
                    Ok(info) => plugins.push(info),
                    Err(e) => {
                        warn!("扫描插件失败 {:?}: {}", path, e);
                        plugins.push(PluginInfo {
                            metadata: PluginMetadata {
                                id: String::new(),
                                name: path.file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("Unknown")
                                    .to_string(),
                                vendor: String::new(),
                                version: String::new(),
                                path: path.clone(),
                                format: PluginFormat::AudioUnit,
                                num_inputs: 2,
                                num_outputs: 2,
                            },
                            valid: false,
                            error: Some(e.to_string()),
                        });
                    }
                }
            }
        }
        
        Ok(plugins)
    }
    
    /// 扫描单个插件
    fn scan_plugin(&self, path: &Path) -> Result<PluginInfo> {
        // 在 macOS 上，Audio Unit 是一个 bundle（目录结构）
        // 结构: PluginName.component/Contents/MacOS/PluginName
        
        let plugin_name = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("无效的插件名称"))?
            .to_string();
        
        // 检查 bundle 结构
        let binary_path = path.join("Contents/MacOS").join(&plugin_name);
        
        if !binary_path.exists() {
            return Err(anyhow::anyhow!("找不到插件二进制文件: {:?}", binary_path));
        }
        
        // TODO: 在 Phase 2 后期，我们会实际加载插件并读取其元数据
        // 现在只是创建基础信息
        
        let metadata = PluginMetadata {
            id: format!("au:{}", plugin_name),
            name: plugin_name.clone(),
            vendor: "Unknown".to_string(), // 需要从插件读取
            version: "0.0.0".to_string(),   // 需要从插件读取
            path: path.to_path_buf(),
            format: PluginFormat::AudioUnit,
            num_inputs: 2,  // 默认立体声
            num_outputs: 2, // 默认立体声
        };
        
        Ok(PluginInfo {
            metadata,
            valid: true,
            error: None,
        })
    }
    
    /// 保存插件缓存
    fn save_cache(&self, plugins: &[PluginInfo]) -> Result<()> {
        let json = serde_json::to_string_pretty(plugins)
            .context("序列化插件列表失败")?;
        
        fs::write(&self.cache_file, json)
            .context("写入缓存文件失败")?;
        
        info!("插件缓存已保存到: {:?}", self.cache_file);
        Ok(())
    }
    
    /// 从缓存加载插件列表
    pub fn load_cache(&self) -> Result<Vec<PluginInfo>> {
        if !self.cache_file.exists() {
            return Ok(Vec::new());
        }
        
        let json = fs::read_to_string(&self.cache_file)
            .context("读取缓存文件失败")?;
        
        let plugins: Vec<PluginInfo> = serde_json::from_str(&json)
            .context("解析缓存文件失败")?;
        
        info!("从缓存加载了 {} 个插件", plugins.len());
        Ok(plugins)
    }
}

impl Default for PluginScanner {
    fn default() -> Self {
        Self::new()
    }
}

