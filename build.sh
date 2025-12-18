#!/bin/bash

echo "🛠️ Building System Monitor..."

# Проверяем Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first."
    exit 1
fi

# Определяем ОС
OS=$(uname -s)
echo "🔧 Detected OS: $OS"

# Сборка
echo "📦 Building for $OS..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Binary location: ./target/release/system-monitor"
    
    # Копируем в удобное место для Linux/macOS
    if [ "$OS" = "Linux" ]; then
        echo "🔧 Linux detected - creating symlink in /usr/local/bin"
        sudo cp ./target/release/system-monitor /usr/local/bin/system-monitor 2>/dev/null || true
        echo "🎉 Try: system-monitor --help"
    elif [ "$OS" = "Darwin" ]; then
        echo "🍎 macOS detected"
        echo "🎉 Try: ./target/release/system-monitor --help"
    fi
else
    echo "❌ Build failed!"
    exit 1
fi