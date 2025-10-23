use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// 插件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginFormat {
    AudioUnit, // macOS 原生格式
    // 未来可以扩展: VST3, VST2, etc.
}

/// 插件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// 插件 ID（唯一标识符）
    pub id: String,
    
    /// 插件名称
    pub name: String,
    
    /// 厂商名称
    pub vendor: String,
    
    /// 版本号
    pub version: String,
    
    /// 插件路径
    pub path: PathBuf,
    
    /// 插件格式
    pub format: PluginFormat,
    
    /// 输入通道数
    pub num_inputs: u32,
    
    /// 输出通道数
    pub num_outputs: u32,
}

/// 插件参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginParameter {
    /// 参数 ID
    pub id: u32,
    
    /// 参数名称
    pub name: String,
    
    /// 参数值 (0.0 - 1.0 归一化)
    pub value: f64,
    
    /// 参数显示文本
    pub display: String,
}

/// 插件状态（用于保存/加载）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginState {
    /// 插件 ID
    pub plugin_id: String,
    
    /// 参数列表
    pub parameters: Vec<PluginParameter>,
    
    /// 原始状态数据（二进制，base64 编码）
    #[serde(default)]
    pub state_data: String,
}

/// 音频处理的 Trait
pub trait AudioProcessor: Send {
    /// 处理音频缓冲区
    /// buffer: 交错的立体声音频数据 [L, R, L, R, ...]
    fn process(&mut self, buffer: &mut [f32]);
    
    /// 获取插件信息
    fn get_info(&self) -> &PluginMetadata;
    
    /// 设置参数
    fn set_parameter(&mut self, id: u32, value: f64);
    
    /// 获取参数
    fn get_parameter(&self, id: u32) -> Option<f64>;
    
    /// 获取所有参数
    fn get_all_parameters(&self) -> Vec<PluginParameter>;
    
    /// 保存状态
    fn save_state(&self) -> PluginState;
    
    /// 加载状态
    fn load_state(&mut self, state: &PluginState);
}
