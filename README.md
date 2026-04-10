# myip – Minimal public IP fetcher

A tiny, safe, and robust Rust tool that shows your public IPv4 address using [ipify.org](https://www.ipify.org).

- **Single binary** – no runtime dependencies
- **Memory safe** – pure Rust, no external crates
- **Timeout‑aware** – won't hang on slow networks
- **Validates output** – guarantees a real IP address

## Quick start

### Linux
```bash
$ wget -q https://github.com/shashwot/myip/releases/download/v1.0.1/myip-linux-x86_64 -O myip && chmod +x myip && sudo mv myip /usr/bin/myip && myip
```

### MACOS Apple Silicon
```bash
$ curl -sL https://github.com/shashwot/myip/releases/download/v1.0.1/myip-macos-aarch64 -o myip && chmod +x myip && sudo mv myip /usr/local/bin/myip && myip
```

### MACOS Intel (x86_64)
```bash
$ curl -sL https://github.com/shashwot/myip/releases/download/v1.0.1/myip-macos-x86_64 -o myip && chmod +x myip && sudo mv myip /usr/local/bin/myip && myip
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
