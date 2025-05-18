# DiscordWebhookProxy
[![Rust-Proxy-Banner.png](https://i.postimg.cc/Pq6cbg7T/Rust-Proxy-Banner.png)](https://postimg.cc/FfLDNBmB)
<p align="center">
	WIP
</p>

---

## Description
DiscordWebhookProxy is a powerful Discord proxy service designed for Roblox, built to prevent abuse and provide secure relaying. It offers complete server management, allowing users to set hardware usage caps and ban abusive users via an intuitive dashboard. Easily deployed with one-click options for Docker, Nix, or Vercel.

---
## Commands & Usage
### Commands
#### Deploy with Docker
Build the container
```bash
docker build -t discord-webhook-proxy .
```
Deploy the container
```bash
docker run -p 8000:8000 discord-webhook-proxy
```

#### Deploy with Nix
```bash
nix build .#dockerImage
```

### WIP ( TBD )