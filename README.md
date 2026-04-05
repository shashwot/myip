# myip – Minimal public IP fetcher

A tiny, safe, and robust Rust tool that shows your public IPv4 address using [ipify.org](https://www.ipify.org).

- **Single binary** – no runtime dependencies
- **Memory safe** – pure Rust, no external crates
- **Timeout‑aware** – won't hang on slow networks
- **Validates output** – guarantees a real IP address

## Quick start

```bash
# Download the latest binary release (replace URL with actual release link)
$ wget https://github.com/shashwot/myip/releases/download/v1.0.1/myip-linux-x86_64

# Move the binary to /usr/bin (requires sudo)
$ sudo mv myip /usr/bin/myip
$ sudo chmod +x /usr/bin/myip

# Run it
$ myip
```

Output 
```
23.250.26.229, FE80:0000:0000:0000:0202:B3FF:FE1E:8329
```