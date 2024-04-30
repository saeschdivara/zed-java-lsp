use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct JavaExtension;

impl zed::Extension for JavaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("java-language-server")
            .ok_or_else(|| "java-language-server must be installed and available on your $PATH".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(JavaExtension);