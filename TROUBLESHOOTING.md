# 故障排除指南

## macOS 权限问题

### 错误："backend-specific error" 或 "An unknown error"

**原因**：macOS 需要明确授予麦克风访问权限。

**解决方案**：

1. **打开系统设置 > 隐私与安全性 > 麦克风**
2. 找到 `Terminal` 或 `Cursor`（取决于你从哪里运行）
3. 勾选启用麦克风访问权限
4. 重新运行程序

### 快速测试（无需麦克风）

如果暂时无法授予权限，可以运行仅输出测试：

```bash
# TODO: 实现 output-only 模式
cargo run --features output-only
```

## 其他常见问题

### Q: 听不到声音？

**检查项**：
- 确认系统音量不是静音
- 检查是否选择了正确的音频设备
- 尝试插拔音频接口

### Q: 延迟很高？

**解决方案**：
- 使用专业音频接口（ASIO/CoreAudio 驱动）
- 减小缓冲区大小（编辑 `src/audio/engine.rs`）

### Q: 音频爆音/卡顿？

**解决方案**：
- 增大缓冲区大小
- 关闭其他占用 CPU 的程序
- 检查系统后台进程

### Q: 找不到音频设备？

**检查项**：
- 音频接口已连接并打开电源
- 在系统设置中可以看到设备
- 重启音频接口

## 获取帮助

如果问题仍未解决，请提供以下信息：

1. macOS 版本：`sw_vers`
2. Rust 版本：`rustc --version`
3. 错误日志：运行 `RUST_LOG=debug cargo run`
4. 音频设备信息

提交 Issue：<repository-url>/issues


