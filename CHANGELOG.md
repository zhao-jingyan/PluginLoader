# 更新日志

所有重要的项目变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

### 计划中
- VST3 插件扫描和加载
- 图形用户界面
- 工程文件管理
- 插件预设保存

## [0.1.0] - 2025-10-23

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


