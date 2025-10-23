mod audio;
mod plugin;

use anyhow::Result;
use log::{info, error};

fn main() -> Result<()> {
    // 初始化日志系统
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Plugin Loader 启动中...");
    info!("版本: {}", env!("CARGO_PKG_VERSION"));
    
    // 测试插件扫描（Phase 2）
    test_plugin_scan();

    println!();
    info!("=== Phase 1: 音频引擎测试 ===");
    
    // Phase 1: 基础音频引擎测试
    match audio::run_audio_engine() {
        Ok(_) => {
            info!("音频引擎正常退出");
            Ok(())
        }
        Err(e) => {
            error!("音频引擎错误: {}", e);
            Err(e)
        }
    }
}
/// 测试插件扫描功能（Phase 2）
fn test_plugin_scan() {
    info!("=== Phase 2: 插件系统测试 ===");
    
    let scanner = plugin::PluginScanner::new();
    
    // 尝试从缓存加载
    match scanner.load_cache() {
        Ok(plugins) if !plugins.is_empty() => {
            info!("从缓存加载了 {} 个插件", plugins.len());
            for (idx, plugin_info) in plugins.iter().take(5).enumerate() {
                info!("  {}. {} ({})", 
                    idx + 1, 
                    plugin_info.metadata.name,
                    if plugin_info.valid { "✅" } else { "❌" }
                );
            }
            if plugins.len() > 5 {
                info!("  ... 还有 {} 个插件", plugins.len() - 5);
            }
        }
        _ => {
            info!("未找到插件缓存，开始扫描...");
            match scanner.scan_all() {
                Ok(plugins) => {
                    info!("扫描完成，找到 {} 个插件", plugins.len());
                    for (idx, plugin_info) in plugins.iter().take(5).enumerate() {
                        info!("  {}. {} ({})", 
                            idx + 1, 
                            plugin_info.metadata.name,
                            if plugin_info.valid { "✅" } else { "❌" }
                        );
                    }
                    if plugins.len() > 5 {
                        info!("  ... 还有 {} 个插件", plugins.len() - 5);
                    }
                }
                Err(e) => {
                    error!("插件扫描失败: {}", e);
                }
            }
        }
    }
}


