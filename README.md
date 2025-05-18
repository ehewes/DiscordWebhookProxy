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
### Deployment Commands
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
```bash
nix run .#dockerImage
```

### Usage
To utilise a domain for your webhook you'd have to point your domain to the server IP and port 8000. You can do this by creating a record in your DNS settings.

``
http://localhost:8000/webhook/WEBHOOK_ID/WEBHOOK_TOKEN
``

#### Curl Example
```
curl -X POST \
  http://localhost:8000/webhook/WEBHOOK_ID/WEBHOOK_TOKEN \
  -H 'Content-Type: application/json' \
  -d '{
    "content": "Hello, world!"
}'
```