use zed_extension_api::{self as zed, settings::LspSettings, Result};

struct OnyxExtension;

struct OnyxBinary {
    path: String,
    args: Option<Vec<String>>,
    environment: Option<Vec<(String, String)>>,
}

impl OnyxExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<OnyxBinary> {
        let mut args: Option<Vec<String>> = Some(vec!["lsp".to_string()]);
        let (platform, _arch) = zed::current_platform();
        let environment = match platform {
            zed::Os::Mac | zed::Os::Linux => Some(worktree.shell_env()),
            zed::Os::Windows => None,
        };
        if let Ok(lsp_settings) = LspSettings::for_worktree("onyx", worktree) {
            if let Some(binary) = lsp_settings.binary {
                args = binary.arguments;
                if let Some(path) = binary.path {
                    return Ok(OnyxBinary {
                        path: path.clone(),
                        args,
                        environment,
                    });
                }
            }
        }
        if let Some(path) = worktree.which("onyx") {
            return Ok(OnyxBinary {path,args,environment});
        }
        let err_message = "No language server!";
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Failed(err_message.to_string()),
        );
        Err(err_message.to_string())
    }
}

impl zed::Extension for OnyxExtension {
    fn new() -> Self {
        Self
    }
    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary.path,
            args: binary.args.unwrap_or_default(),
            env: binary.environment.unwrap_or_default(),
        })
    }
}

zed::register_extension!(OnyxExtension);
