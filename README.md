# 🧀 feed-the-rat

Tiny helper that listens to RSS feeds and POSTs to Discord via webhooks.

## Configuration

Copy `config.example.toml` to `config.toml` and setup the webhook URL and your feeds:

```toml
[discord]
webhook_url = "<put your discord webhook url here>"

[feeds.terminal_trove_blog]
url = "https://terminaltrove.com/blog.xml"
interval = 600

[feeds.terminal_trove_totw]
url = "https://terminaltrove.com/totw.xml"
interval = 600

[feeds.ratatui]
url = "https://fosstodon.org/@ratatui_rs.rss"
interval = 600
```
