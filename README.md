# 💾 Vercel MCP Server for Zed

This extension acts as a remote client for a [Vercel MCP](https://vercel.com/docs/mcp) server, allowing you to use it as a context server for [Zed's](https://zed.dev) [Agent Panel.](https://zed.dev/docs/ai/overview)

It uses the `mcp-remote` tool to create a local stdio proxy to your remote, HTTP-based Vercel MCP server, enabling seamless integration with the Zed editor.

## 🤖 Overview

Instead of running a local MCP server, this extension connects to a server you have deployed on Vercel. This allows you to leverage powerful, cloud-based tools within Zed's AI assistant.

### 🖥️ Screenshot

![Vercel MCP Server for Zed](https://github.com/Takk8IS/vercel-mcp-server-for-zed/blob/main/assets/screenshot-01.png?raw=true)

### 💾 Installation

**1. Prerequisites**

Before using this extension, you must have a Vercel MCP server deployed and accessible via a public URL. You can learn how to create one by following the [official Vercel MCP documentation](https://vercel.com/docs/mcp).

**2. Install from Zed Marketplace**

- **Open**: Open the Command Palette in Zed (`cmd-shift-p`).
- **Type**: Type `zed: extensions`.
- **Search**: Search for "Vercel MCP Server" and install it directly from the extension marketplace.

**3. Configuration**

- **Open**: Open your Zed `settings.json` file.
- **Add**: Configure the URL of your deployed Vercel MCP server:

```json
"context_servers": {
  "mcp-server-vercel": {
    "settings": {
      "url": "<YOUR_VERCEL_MCP_SERVER_URL>"
    }
  }
}
```

Replace `<YOUR_VERCEL_MCP_SERVER_URL>` with the actual URL of your server (e.g., `https://my-mcp-server.vercel.app/api/mcp`).

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

## 💡 Donations

If this project has been helpful, consider making a donation:

**USDT (TRC-20)**: `TP6zpvjt2ZNGfWKPevfp65ZrcbKMWSQXDi`

Your support helps us continue to develop innovative tools.

## 🔧 Support

Experience the power of Zed by visiting their [official website](https://zed.dev/).

To contribute to public and social projects focused on research and artificial intelligence, feel free to support with any amount you prefer.

## 👥 About the Author

### 🧠 Takk™ Innovate Studio

- **Author**: David C Cavalcante
- **LinkedIn**: [David C Cavalcante](https://www.linkedin.com/in/hellodav/)
- **Medium**: [David C Cavalcante](https://medium.com/@davcavalcante/)
- **Positive results, rapid innovation**
- **Leading the Digital Revolution as the Pioneering 100% Artificial Intelligence Team**
- **URL**: [Takk](https://takk.ag/)
- **Twitter**: [Takk](https://twitter.com/takk8is/)
- **Medium**: [Takk](https://takk8is.medium.com/)

Enjoy coding with the power of Vercel MCP in your Zed editor.
