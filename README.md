# myip – Minimal public IP fetcher

A tiny, safe, and robust Rust tool that shows your public IPv4 address using [ipify.org](https://www.ipify.org).

- **Single binary** – no runtime dependencies
- **Memory safe** – pure Rust, no external crates
- **Timeout‑aware** – won't hang on slow networks
- **Validates output** – guarantees a real IP address

## Quick start

### MACOS
```bash
# Download the latest macOS binary (replace URL with actual release link). In case of x86_64, use myip-macos-x86_64
$ curl -LO https://github.com/shashwot/myip/releases/download/v1.0.1/myip-macos-aarch64

# Make it executable
$ chmod +x myip-macos-aarch64

# Move to a directory in your PATH, e.g., /usr/local/bin
$ sudo mv myip-macos-aarch64 /usr/local/bin/myip

# Verify installation
$ myip
```

### Linux
```bash
# Download the latest Linux binary (replace URL with actual release link)
$ wget https://github.com/shashwot/myip/releases/download/v1.0.1/myip-linux-x86_64

# Make it executable
$ chmod +x myip-linux-x86_64

# Move to a directory in your PATH, e.g., /usr/bin
$ sudo mv myip-linux-x86_64 /usr/bin/myip

# Verify installation
$ myip
```

### Windows
Download the latest Windows binary (myip-windows-x86_64.exe) from the [release page](https://github.com/shashwot/myip/releases/download/v1.0.1/myip-windows-x86_64.exe).
```powershell
C:\> myip-windows-x86_64.exe
```

### Output 
```
23.250.26.229, FE80:0000:0000:0000:0202:B3FF:FE1E:8329
```