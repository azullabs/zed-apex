use std::fs;
use zed_extension_api::{
    self as zed, serde_json, settings::LspSettings, Command, DownloadedFileType, Extension,
    LanguageServerId, LanguageServerInstallationStatus, Result, Worktree,
};

const SERVER_JAR: &str = "apex-jorje-lsp.jar";
const DOWNLOAD_URL: &str = "https://raw.githubusercontent.com/forcedotcom/salesforcedx-vscode/develop/packages/salesforcedx-vscode-apex/jars/apex-jorje-lsp.jar";

struct ApexExtension {
    cached_jar_path: Option<String>,
}

impl Extension for ApexExtension {
    fn new() -> Self {
        Self {
            cached_jar_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let jar_path = self.ensure_jar_downloaded(language_server_id)?;
        let java_path = self.find_java(worktree)?;

        Ok(Command {
            command: java_path,
            args: vec![
                "-cp".to_string(),
                jar_path,
                "-Ddebug.internal.errors=true".to_string(),
                "-Ddebug.semantic.errors=false".to_string(),
                "-Ddebug.completion.statistics=false".to_string(),
                "-Dlwc.typegeneration.disabled=true".to_string(),
                "apex.jorje.lsp.ApexLanguageServerLauncher".to_string(),
            ],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|s| s.initialization_options);

        Ok(settings.or_else(|| {
            Some(serde_json::json!({
                "enableEmbeddedSoqlCompletion": true
            }))
        }))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|s| s.settings))
    }
}

impl ApexExtension {
    fn ensure_jar_downloaded(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        if let Some(path) = &self.cached_jar_path {
            if fs::metadata(path).map_or(false, |m| m.is_file()) {
                return Ok(path.clone());
            }
        }

        let jar_path = SERVER_JAR.to_string();
        if fs::metadata(&jar_path).map_or(false, |m| m.is_file()) {
            self.cached_jar_path = Some(jar_path.clone());
            return Ok(jar_path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        zed::download_file(DOWNLOAD_URL, &jar_path, DownloadedFileType::Uncompressed)
            .map_err(|e| format!("Failed to download {SERVER_JAR}: {e}"))?;

        self.cached_jar_path = Some(jar_path.clone());
        Ok(jar_path)
    }

    fn find_java(&self, worktree: &Worktree) -> Result<String> {
        worktree.which("java").ok_or_else(|| {
            "Java not found. Install Java 11 or later and ensure `java` is on your PATH."
                .to_string()
        })
    }
}

zed::register_extension!(ApexExtension);
