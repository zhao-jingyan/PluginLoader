# 开发指南

本文档面向想要参与 Plugin Loader 开发的开发者。

## 🏗️ 项目架构

### 模块结构

```
src/
├── main.rs              # 应用入口点
├── audio/               # 音频处理模块
│   ├── mod.rs          # 模块导出
│   ├── engine.rs       # 音频引擎核心
│   ├── device.rs       # 音频设备管理
│   └── level_meter.rs  # 电平表实现
├── plugin/             # (Phase 2) 插件系统
└── ui/                 # (Phase 3) 用户界面
```

### 数据流

```
音频输入设备 (CoreAudio)
    ↓
Input Stream Callback
    ↓
环形缓冲区 (RingBuffer)
    ↓
Output Stream Callback  
    ↓
音频输出设备 (CoreAudio)

同时 ↓
电平表更新 (原子操作)
    ↓
UI 线程读取 (定期轮询)
```

## 🔧 开发环境设置

### 必需工具

```bash
# 安装 Rust (最新稳定版)
rustup update stable

# 安装开发工具
cargo install cargo-watch    # 文件监听自动编译
cargo install cargo-edit     # 管理依赖
cargo install cargo-tree     # 查看依赖树
```

### 推荐 IDE 配置

**VS Code / Cursor:**
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true
}
```

## 🔨 常用命令

```bash
# 开发模式运行（带日志）
RUST_LOG=debug cargo run

# 自动重新编译
cargo watch -x run

# 代码检查
cargo clippy

# 格式化代码
cargo fmt

# 运行测试
cargo test

# 性能分析（macOS）
cargo build --release
instruments -t "Time Profiler" ./target/release/plugin-loader
```

## 📝 代码规范

### Rust 风格

- 遵循 [Rust 官方风格指南](https://doc.rust-lang.org/nightly/style-guide/)
- 使用 `cargo fmt` 自动格式化
- 所有 public API 必须有文档注释
- 使用 `clippy` 检查代码质量

### 命名约定

- 函数：`snake_case`
- 类型/结构体：`PascalCase`
- 常量：`SCREAMING_SNAKE_CASE`
- 模块：`snake_case`

### 文档注释示例

```rust
/// 音频电平表，用于实时显示输入/输出电平
/// 
/// # 线程安全
/// 
/// 该结构体使用原子操作，可以安全地在音频线程和 UI 线程之间共享。
/// 
/// # 示例
/// 
/// ```
/// let meter = LevelMeter::new();
/// meter.update(0.5, 0.8);
/// let (left, right) = meter.get_peak_db();
/// ```
pub struct LevelMeter {
    // ...
}
```

## 🎯 Phase 2: VST3 插件系统开发指南

### 准备工作

1. **学习 VST3 规范**
   - [VST3 官方文档](https://steinbergmedia.github.io/vst3_doc/)
   - 参考项目：[vst-rs](https://github.com/RustAudio/vst-rs)

2. **设置测试插件**
   ```bash
   # 下载免费 VST3 插件用于测试
   # 推荐：Amplitube Free, Guitar Rig Player
   ```

### 实现步骤

#### 1. 插件扫描器

```rust
// src/plugin/scanner.rs
pub struct PluginScanner {
    // 扫描 VST3 目录
    // 解析 .vst3 bundle
    // 缓存插件信息
}
```

#### 2. 插件加载器

```rust
// src/plugin/loader.rs
pub struct PluginLoader {
    // 动态库加载 (libloading)
    // VST3 初始化
    // 实例管理
}
```

#### 3. 插件链

```rust
// src/plugin/chain.rs
pub struct PluginChain {
    plugins: Vec<Box<dyn Plugin>>,
    
    pub fn process(&mut self, buffer: &mut [f32]) {
        for plugin in &mut self.plugins {
            plugin.process(buffer);
        }
    }
}
```

### 集成到音频引擎

修改 `src/audio/engine.rs`：

```rust
// 在音频回调中：
let mut buffer = /* 从 input 获取 */;

// 应用插件链
plugin_chain.process(&mut buffer);

// 输出到 output
```

## 🧪 测试策略

### 单元测试

```bash
# 运行所有测试
cargo test

# 运行特定模块
cargo test audio::level_meter

# 显示输出
cargo test -- --nocapture
```

### 集成测试

```bash
# 测试完整音频流
cargo test --test audio_integration
```

### 性能测试

```rust
#[cfg(test)]
mod benches {
    use test::Bencher;
    
    #[bench]
    fn bench_audio_process(b: &mut Bencher) {
        // 测试音频处理性能
    }
}
```

## 🐛 调试技巧

### 音频问题调试

```rust
// 在音频回调中使用原子标志
static DEBUG_FLAG: AtomicBool = AtomicBool::new(false);

if DEBUG_FLAG.load(Ordering::Relaxed) {
    // 不要在音频线程使用 println!
    // 使用原子计数器或写入环形缓冲区
}
```

### 使用 Instruments (macOS)

```bash
# CPU 性能分析
instruments -t "Time Profiler" ./target/release/plugin-loader

# 内存泄漏检测
instruments -t "Leaks" ./target/release/plugin-loader

# 线程分析
instruments -t "System Trace" ./target/release/plugin-loader
```

## 📚 推荐阅读

### Rust 音频开发

- [RustAudio GitHub](https://github.com/RustAudio)
- [CPAL 文档](https://docs.rs/cpal/)
- [Real-time audio in Rust](https://www.youtube.com/watch?v=Yom9E-67bdI)

### VST3 开发

- [VST3 SDK](https://github.com/steinbergmedia/vst3sdk)
- [vst3-sys crate](https://docs.rs/vst3-sys/)

### 音频处理基础

- [The Audio Programmer YouTube](https://www.youtube.com/c/TheAudioProgrammer)
- [Digital Signal Processing - Smith](https://www.dspguide.com/)

## 🤝 贡献流程

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### PR 检查清单

- [ ] 代码通过 `cargo fmt` 和 `cargo clippy`
- [ ] 添加了必要的测试
- [ ] 更新了文档
- [ ] CHANGELOG.md 已更新

## ❓ 常见问题

### Q: 如何减少音频延迟？

A: 调整 `src/audio/engine.rs` 中的缓冲区大小：
```rust
config.buffer_size = cpal::BufferSize::Fixed(128); // 或更小
```

### Q: 如何添加日志？

A: 使用 `log` 宏：
```rust
use log::{info, warn, error, debug};

debug!("详细调试信息");
info!("一般信息");
warn!("警告");
error!("错误");
```

运行时设置日志级别：
```bash
RUST_LOG=debug cargo run
```

### Q: 音频回调中可以做什么？

A: **可以**:
- 处理音频缓冲区
- 使用原子操作
- 访问无锁数据结构

**不可以**:
- 分配内存 (malloc/Vec::push)
- 使用 Mutex (可能阻塞)
- 调用系统 I/O
- 使用 println!

---

有问题？欢迎提 Issue 或联系维护者！🎸


