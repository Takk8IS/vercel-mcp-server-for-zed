use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

// TODO: Define the actual AWS MCP server package name or binary path
const AWS_MCP_SERVER_BINARY: &str = "aws-mcp-server";

#[derive(Debug, Deserialize, JsonSchema)]
struct AwsContextServerSettings {
    aws_access_key_id: String,
    aws_secret_access_key: String,
    aws_region: String,
    // Add other AWS-specific settings here as needed
}

struct AwsMcpExtension;

impl zed::Extension for AwsMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        // TODO: Implement logic to find and run the AWS MCP server binary.
        // This might involve checking if it's installed, downloading it,
        // or building it from source.

        let settings = ContextServerSettings::for_project("aws-mcp-server-for-zed", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing settings".into());
        };
        let settings: AwsContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: AWS_MCP_SERVER_BINARY.to_string(),
            args: vec![
                "--access-key-id".to_string(),
                settings.aws_access_key_id,
                "--secret-access-key".to_string(),
                settings.aws_secret_access_key,
                "--region".to_string(),
                settings.aws_region,
            ],
            env: vec![],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        // TODO: Update installation_instructions.md and default_settings.jsonc
        // for the AWS MCP server.
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(AwsContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(AwsMcpExtension);
