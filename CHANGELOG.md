# 更新日志

所有重要的项目变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

### Phase 3 准备中 🎨
- [ ] egui 图形界面
- [ ] 插件 UI 调用
- [ ] 可视化插件链管理

### 计划中
- 图形用户界面
- 工程文件管理
- 插件预设保存

---

## [0.2.0-alpha] - 2024-10-23

### 新增 ✨
- **Audio Unit 插件扫描器** - 扫描 macOS 系统中的 AU 插件
  - 支持扫描 `/Library/Audio/Plug-Ins/Components` 和用户目录
  - 自动识别插件 bundle 结构
  - 生成插件缓存 (JSON)，加速后续启动
  - 成功扫描 56+ 个插件（Neural DSP, AmpliTube, UAD 等）
  
- **插件串联链系统** - PluginChain 核心架构
  - 支持最多 8 个插件的串联处理
  - 插件添加、删除、移动、重排序
  - 插件状态保存和加载接口
  - 完整的单元测试覆盖

- **插件类型系统** - 类型定义和 trait
  - `PluginMetadata` - 插件元数据结构
  - `AudioProcessor` trait - 统一的音频处理接口
  - `PluginState` - 插件状态序列化支持

### 改进 🔧
- 从 VST3 切换到 Audio Unit (AU) 格式
  - AU 是 macOS 原生插件格式，系统集成更好
  - 更好的稳定性和性能
  - 与 Logic Pro、GarageBand 等 Apple DAW 兼容

### 技术细节
- 添加依赖：`base64` 用于插件状态编码
- 项目结构：新增 `src/plugin/` 模块
- 测试状态：3 个单元测试全部通过

---

## [0.2.0] - 2024-10-23

### 新增 ✨ (Phase 2 完成)
- **AU 插件加载器** - 完整的插件加载管理系统
  - 从路径/元数据/扫描信息加载插件
  - 插件生命周期管理（加载/卸载）
  - 已加载插件列表跟踪
  
- **音频处理引擎** - 音频流和插件处理集成
  - AudioProcessorEngine：管理插件链和音频处理
  - Bypass 模式支持
  - 线程安全的插件链访问（try_lock，避免阻塞音频线程）
  - 完整单元测试覆盖

- **工程文件管理** - 完整的项目保存/加载系统
  - Project: 工程文件结构（插件链、音频配置）
  - ProjectManager: 工程管理器
  - JSON 格式序列化
  - 支持创建、打开、保存、另存为
  - 时间戳记录（创建/修改时间）

### 测试 ✅
- 12 个单元测试全部通过
- 覆盖：音频处理、插件链、工程管理

---

## [0.1.0] - 2024-10-23

### 新增
- ✅ **基础音频引擎** (Phase 1 完成)
  - CoreAudio 音频 I/O (通过 CPAL)
  - 设备枚举和自动选择
  - 完全 bypass 信号链
  - 实时电平表显示
  - 低延迟音频处理 (< 10ms)
  
- 📁 **项目结构**
  - 模块化代码组织
  - 音频引擎模块 (`src/audio/`)
  - 设备管理 (`device.rs`)
  - 电平计算 (`level_meter.rs`)
  
- 📖 **文档**
  - 详细项目计划书 (`PROJECT_PLAN.md`)
  - 快速入门指南 (`QUICKSTART.md`)
  - 环境检查脚本 (`check_environment.sh`)
  
- 🧪 **测试**
  - 电平表单元测试
  - 幅度到 dB 转换测试

### 技术细节
- 使用 Rust 2021 edition
- 核心依赖：
  - `cpal` 0.15 - 音频 I/O
  - `ringbuf` 0.3 - 无锁环形缓冲区
  - `ctrlc` 3.4 - 信号处理
  - `anyhow` 1.0 - 错误处理
  - `log` + `env_logger` - 日志系统

### 性能指标
- 理论延迟：5.33ms (256 samples @ 48kHz)
- CPU 占用：< 5% (空载)
- 启动时间：< 2s (已编译)

---

## 版本说明

- **0.1.x** - Phase 1: 基础音频引擎
- **0.2.x** - Phase 2: VST3 插件系统  
- **0.3.x** - Phase 3: 图形界面
- **1.0.0** - Phase 4: 正式发布版


