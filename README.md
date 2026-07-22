# DiscordWebhookProxy

[![Rust-Proxy-Banner.png](https://i.postimg.cc/Pq6cbg7T/Rust-Proxy-Banner.png)](https://postimg.cc/FfLDNBmB)

A lightweight Discord webhook proxy with **async forwarding**, **automatic rate-limit queueing**, **persistent crash recovery**, and full Discord API compatibility.

---

## Features

- ⚡ **Async, non-blocking** — built on tokio + reqwest, won't stall under load
- 📦 **Automatic rate-limit queue** — when Discord responds 429, requests are queued and retried with configurable concurrency
- 💾 **Persistent queue storage** — backed by Sled (embedded DB); survives restarts and crashes
- 🔄 **Crash recovery** — on startup, any unfinished queued webhooks are re-processed automatically
- 🔁 **Drop-in replacement** — same payload format, same response codes (GET + POST)
- 🔧 **Configurable via environment variables**

---

## Quick Start

### Docker

```bash
docker build -t discord-webhook-proxy .
docker run -p 8000:8000 discord-webhook-proxy
```

### From source

```bash
cargo run --release
```

---

## Usage

### GET — fetch webhook info

```bash
curl http://localhost:8000/webhook/WEBHOOK_ID/WEBHOOK_TOKEN
```

### POST — send a webhook message

```bash
curl -X POST http://localhost:8000/webhook/WEBHOOK_ID/WEBHOOK_TOKEN \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Hello, world!",
    "username": "My Bot",
    "embeds": [{
      "title": "Embed Title",
      "description": "Embed description",
      "color": 3447003
    }]
  }'
```

### Environment Variables

| Variable | Default | Description |
|---|---|---|
| `QUEUE_SIZE` | `50000` | Max queued requests before blocking |
| `CONCURRENCY_LIMIT` | `20` | Max concurrent outbound requests to Discord |
| `FALLBACK_COOLDOWN_SECS` | `10` | Seconds to wait before retry when Retry-After header is missing |
