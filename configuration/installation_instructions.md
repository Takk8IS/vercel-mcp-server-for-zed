To use this extension, you need a Vercel MCP server deployed and accessible via a URL.

1.  **Deploy your Vercel MCP server**: Ensure you have a Vercel MCP server already deployed. You can find instructions on how to create one in the [official Vercel MCP documentation](https://vercel.com/docs/mcp).
2.  **Copy your server URL**: Obtain the URL of your deployed Vercel MCP server (e.g., `https://your-mcp-server.vercel.app/api/mcp`).
3.  **Configure in Zed**: Open your Zed settings (via Command Palette: `settings: open your settings`).
4.  **Add the configuration**: Insert the following JSON snippet into your Zed settings, replacing `<YOUR_VERCEL_MCP_SERVER_URL>` with the URL you copied:

```json
"context_servers": {
  "mcp-server-vercel": {
    "settings": {
      "url": "<YOUR_VERCEL_MCP_SERVER_URL>"
    }
  }
}
```
