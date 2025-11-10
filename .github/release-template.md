### Installation

**Homebrew (macOS/Linux)**:
```bash
brew install billxc/tap/gfv
```

**Scoop (Windows)**:
```powershell
scoop bucket add xc-scoop https://github.com/billxc/xc-scoop
scoop install gfv
```

### Manual Installation

**macOS**:
```bash
curl -L -O https://github.com/{{REPOSITORY}}/releases/download/{{TAG_NAME}}/gfv-macos-arm64
chmod +x gfv-macos-arm64
sudo mv gfv-macos-arm64 /usr/local/bin/gfv
```

**Linux**:
```bash
curl -L -O https://github.com/{{REPOSITORY}}/releases/download/{{TAG_NAME}}/gfv-linux-x86_64
chmod +x gfv-linux-x86_64
sudo mv gfv-linux-x86_64 /usr/local/bin/gfv
```

**Windows**:
Download `gfv-windows-x86_64.exe` and add to your PATH.

### Available Downloads
- **macOS Apple Silicon (ARM64)**: `gfv-macos-arm64`
- **macOS Intel (x86_64)**: `gfv-macos-x86_64`
- **Linux x86_64**: `gfv-linux-x86_64`
- **Linux ARM64**: `gfv-linux-arm64`
- **Windows x86_64**: `gfv-windows-x86_64.exe`
- **Windows ARM64**: `gfv-windows-arm64.exe`
