# 💾 AWS MCP Server for Zed

This extension provides an AWS Model Context Protocol (MCP) server for the Zed editor, allowing you to use AWS services as a context provider for Zed's AI Assistant.

It enables Zed to communicate with AWS services, such as retrieving information from S3, querying DynamoDB, or invoking Lambda functions, directly within the editor's context.

## 🤖 Overview

This extension starts a local MCP server that is configured to interact with your AWS account. By providing your AWS credentials, the server can access your AWS resources and provide relevant information to Zed's AI assistant.

### 🖥️ Screenshot

![AWS MCP Server for Zed](https://github.com/Takk8IS/aws-mcp-server-for-zed/blob/main/assets/screenshot-01.png?raw=true)

### 💾 Installation

**1. Prerequisites**

Before using this extension, you must have an AWS account and have your AWS Access Key ID and Secret Access Key available.

**2. Install from Zed Marketplace**

- **Open**: Open the Command Palette in Zed (`cmd-shift-p`).
- **Type**: Type `zed: extensions`.
- **Search**: Search for "AWS MCP Server" and install it directly from the extension marketplace.

**3. Configuration**

- **Open**: Open your Zed `settings.json` file.
- **Add**: Configure your AWS credentials:

```json
"context_servers": {
  "aws-mcp-server-for-zed": {
    "settings": {
      "aws_access_key_id": "<YOUR_AWS_ACCESS_KEY_ID>",
      "aws_secret_access_key": "<YOUR_AWS_SECRET_ACCESS_KEY>",
      "aws_region": "<YOUR_AWS_REGION>"
    }
  }
}
```

Replace the placeholders with your actual AWS credentials and desired region.

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

Enjoy coding with the power of AWS MCP in your Zed editor.