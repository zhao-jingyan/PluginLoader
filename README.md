# Plugin Loader - 轻量化吉他效果器插件宿主

一个专为吉他手打造的轻量化 VST3 插件加载器，让你无需启动 DAW 就能使用专业效果器插件练琴。

## ✨ 特性

- 🎸 **低延迟音频处理** - 基于 CoreAudio，延迟 < 10ms
- 🔌 **Audio Unit 插件支持** - 加载和串联多个专业插件（macOS 原生格式）
- 🎚️ **简洁界面** - 专注于练琴，去除 DAW 的复杂功能
- 🚀 **轻量快速** - 启动时间 < 3 秒
- 💾 **工程管理** - 保存和加载插件链配置

## 📋 系统要求

- macOS 12.0+ (目前仅支持 macOS)
- 音频接口（推荐）
- Rust 1.70+ (开发)

## 🚀 快速开始

### 安装依赖

```bash
# 安装 Rust (如果尚未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 编译运行

```bash
# 克隆仓库
git clone <repository-url>
cd PluginLoader

# 运行项目（开发模式）
cargo run

# 编译发布版本
cargo build --release
```

### Phase 1: 基础音频引擎测试

当前版本实现了完全 bypass 的音频信号链，用于测试基础架构：

```bash
cargo run
```

**⚠️ 重要**：首次运行需要授予麦克风权限：
1. 打开 **系统设置 > 隐私与安全性 > 麦克风**
2. 找到 **Terminal** 或 **Cursor** 并勾选
3. 重新运行程序

程序会：
1. 列出所有音频设备
2. 自动选择默认输入/输出设备
3. 建立低延迟音频流
4. 实时显示输入电平表
5. 按 Ctrl+C 退出

如遇问题，请查看 [TROUBLESHOOTING.md](TROUBLESHOOTING.md)

## 📁 项目结构

```
PluginLoader/
├── src/
│   ├── main.rs           # 程序入口
│   └── audio/            # 音频引擎模块
│       ├── mod.rs        # 模块定义
│       ├── engine.rs     # 音频引擎核心
│       ├── device.rs     # 设备管理
│       └── level_meter.rs # 电平表
├── Cargo.toml            # 项目配置
├── PROJECT_PLAN.md       # 详细计划书
└── README.md             # 本文件
```

## 🛣️ 开发路线

- [x] **Phase 1** - 基础音频引擎（当前）
  - [x] 设备枚举
  - [x] Bypass 信号链
  - [x] 电平表显示
  
- [x] **Phase 2** - Audio Unit 插件系统（已完成）
  - [x] 插件扫描器
  - [x] 插件加载器（AU 框架集成）
  - [x] 串联信号链
  - [x] 音频处理引擎
  - [x] 工程文件管理
  
- [ ] **Phase 3** - 图形界面
  - [ ] egui 主窗口
  - [ ] 插件 UI 调用
  - [ ] 工程文件管理
  
- [ ] **Phase 4** - 优化与发布
  - [ ] 性能优化
  - [ ] macOS 打包

详细计划请查看 [PROJECT_PLAN.md](PROJECT_PLAN.md)

## 🧪 测试

```bash
# 运行测试
cargo test

# 运行特定模块测试
cargo test level_meter
```

## 📝 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

**注意**: 本项目目前处于早期开发阶段 (Phase 1)，功能持续完善中。