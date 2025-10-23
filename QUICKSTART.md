# 快速入门指南

## Phase 1: 基础音频引擎测试

### 1. 确保已安装 Rust

```bash
# 检查 Rust 版本
rustc --version

# 如果未安装，执行：
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. 编译并运行

```bash
cd /Users/zhaojingyan/Code/PluginLoader

# 首次运行会下载依赖并编译（可能需要几分钟）
cargo run

# 或者先编译，再运行
cargo build
./target/debug/plugin-loader
```

### 3. 预期行为

程序启动后，你应该看到：

```
Plugin Loader 启动中...
版本: 0.1.0

=== 音频设备列表 ===
📥 默认输入设备: MacBook Pro Microphone
  支持的配置:
    - 采样率: 44100 - 48000 Hz, 通道数: 1
  默认配置: 48000 Hz, 1 通道, F32

📤 默认输出设备: MacBook Pro Speakers
  支持的配置:
    - 采样率: 44100 - 48000 Hz, 通道数: 2
  默认配置: 48000 Hz, 2 通道, F32

=== 启动音频引擎 ===
输入设备: MacBook Pro Microphone
输出设备: MacBook Pro Speakers
音频配置: 48000 Hz, 2 通道, 缓冲区大小: 256 samples
理论延迟: 5.33 ms
✅ 音频引擎启动成功！
提示: 按 Ctrl+C 停止

🎸 电平: L: ████░░░░░░░░░░░░░░░░ -18.2 dB | R: ██░░░░░░░░░░░░░░░░░░ -24.5 dB
```

### 4. 测试音频 Bypass

1. **连接音频接口**（推荐）或使用内置麦克风
2. **插入吉他/乐器**到音频接口
3. 运行程序：`cargo run`
4. **演奏**，观察实时电平表
5. **监听输出**，确认音频无延迟通过

### 5. 停止程序

按 `Ctrl+C` 优雅退出。

## 常见问题

### Q: 听不到声音？
A: 
- 检查系统音量设置
- 确认音频接口已连接并被识别
- 检查输入/输出设备是否正确选择

### Q: 延迟太高？
A: 
- 在 `src/audio/engine.rs` 中调整缓冲区大小：
  ```rust
  config.buffer_size = cpal::BufferSize::Fixed(128); // 改为 128
  ```
- 使用专业音频接口（如 Focusrite Scarlett）

### Q: 编译错误？
A: 
- 确保 Rust 版本 >= 1.70: `rustup update`
- 清理并重新编译: `cargo clean && cargo build`

## 下一步

Phase 1 测试完成后，将进入 Phase 2：VST3 插件系统开发。

敬请期待！🎸

