mod audio;

use anyhow::Result;
use log::{info, error};

fn main() -> Result<()> {
    // 初始化日志系统
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Plugin Loader 启动中...");
    info!("版本: {}", env!("CARGO_PKG_VERSION"));

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

