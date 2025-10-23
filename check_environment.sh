#!/bin/bash

# 环境检查脚本

echo "=== Plugin Loader 环境检查 ==="
echo ""

# 检查 Rust
echo "1. 检查 Rust 安装..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo "   ✅ $RUST_VERSION"
else
    echo "   ❌ 未安装 Rust"
    echo "   请运行: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# 检查 Cargo
echo ""
echo "2. 检查 Cargo..."
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo "   ✅ $CARGO_VERSION"
else
    echo "   ❌ 未安装 Cargo"
    exit 1
fi

# 检查 macOS 版本
echo ""
echo "3. 检查操作系统..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    MACOS_VERSION=$(sw_vers -productVersion)
    echo "   ✅ macOS $MACOS_VERSION"
else
    echo "   ⚠️  当前系统不是 macOS，可能需要额外配置"
fi

# 检查音频设备
echo ""
echo "4. 检查音频设备..."
if command -v system_profiler &> /dev/null; then
    AUDIO_DEVICES=$(system_profiler SPAudioDataType 2>/dev/null | grep -c "Audio ID")
    if [ "$AUDIO_DEVICES" -gt 0 ]; then
        echo "   ✅ 检测到音频设备"
    else
        echo "   ⚠️  未检测到音频设备"
    fi
fi

# 检查项目文件
echo ""
echo "5. 检查项目文件..."
if [ -f "Cargo.toml" ]; then
    echo "   ✅ Cargo.toml 存在"
else
    echo "   ❌ Cargo.toml 不存在，请确认在项目根目录"
    exit 1
fi

if [ -d "src" ]; then
    echo "   ✅ src 目录存在"
else
    echo "   ❌ src 目录不存在"
    exit 1
fi

echo ""
echo "=== 环境检查完成 ==="
echo ""
echo "可以运行项目了："
echo "  cargo run"
echo ""

