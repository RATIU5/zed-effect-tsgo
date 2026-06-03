// Originally based on the tsgo extension by Zed Industries, Inc.
// Modified by RATIU5 — see git history for changes.

use std::fs;
use std::path::{Path, PathBuf};

use zed_extension_api::{self as zed, LanguageServerId, Result, settings::LspSettings};

struct EffectTsgoExtension {
    cached_binary_path: Option<String>,
    cached_settings: Option<EffectTsgoSettings>,
}

const PACKAGE_NAME: &str = "@effect/tsgo";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct EffectTsgoSettings {
    binary_path: Option<String>,
    package_version: Option<String>,
}

impl EffectTsgoSettings {
    fn from_lsp_settings(settings: &LspSettings) -> Self {
        let binary_path = settings
            .binary
            .as_ref()
            .and_then(|binary| binary.path.as_deref())
            .map(|s| s.trim().to_string())
            .filter(|path| !path.is_empty());

        let package_version = settings
            .settings
            .as_ref()
            .and_then(|s| s.get("package_version"))
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        Self {
            binary_path,
            package_version,
        }
    }
}

impl EffectTsgoExtension {
    fn invalidate_cache_if_settings_changed(&mut self, settings: &EffectTsgoSettings) {
        if self.cached_settings.as_ref() != Some(settings) {
            self.cached_binary_path = None;
            self.cached_settings = Some(settings.clone());
        }
    }

    fn extension_working_directory() -> Result<PathBuf> {
        std::env::current_dir().map_err(|err| {
            format!(
                "Failed to determine the extension working directory: {}",
                err
            )
        })
    }

    fn get_platform_package_name() -> Result<String> {
        let (platform, arch) = zed::current_platform();

        let os = match platform {
            zed::Os::Mac => "darwin",
            zed::Os::Linux => "linux",
            zed::Os::Windows => "win32",
        };

        let arch = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X86 => {
                return Err(
                    "32-bit x86 architecture is not supported. Please use a 64-bit system."
                        .to_string(),
                );
            }
            zed::Architecture::X8664 => "x64",
        };

        Ok(format!("@effect/tsgo-{}-{}", os, arch))
    }

    fn get_native_binary_path() -> Result<PathBuf> {
        let platform_package = Self::get_platform_package_name()?;
        let extension_dir = Self::extension_working_directory()?;

        let package_path = extension_dir.join("node_modules").join(&platform_package);

        if !package_path.exists() {
            return Err(format!(
                "Platform package {} was not found at {}. The requested @effect/tsgo package may not provide a native binary for this platform, or the installation is incomplete.",
                platform_package,
                package_path.display()
            ));
        }

        let (platform, _) = zed::current_platform();
        let binary_name = match platform {
            zed::Os::Windows => "tsgo.exe",
            _ => "tsgo",
        };

        let binary_path = package_path.join("lib").join(binary_name);

        if !binary_path.exists() {
            return Err(format!(
                "Native binary for {} was not found at {}. The platform package may be missing or corrupted.",
                platform_package,
                binary_path.display()
            ));
        }

        Ok(binary_path)
    }

    fn ensure_binary_is_usable(path: &Path, source_description: &str) -> Result<()> {
        let metadata = fs::metadata(path).map_err(|err| {
            format!(
                "{} does not exist at {}: {}",
                source_description,
                path.display(),
                err
            )
        })?;

        if !metadata.is_file() {
            return Err(format!(
                "{} at {} is not a file.",
                source_description,
                path.display()
            ));
        }

        let path_str = path.to_str().ok_or_else(|| {
            format!(
                "{} at {} is not valid UTF-8, so it cannot be made executable.",
                source_description,
                path.display()
            )
        })?;

        zed::make_file_executable(path_str).map_err(|err| {
            format!(
                "{} at {} could not be made executable: {}",
                source_description,
                path.display(),
                err
            )
        })?;

        Ok(())
    }

    fn binary_exists(&self) -> bool {
        Self::get_native_binary_path().is_ok()
    }

    fn get_installed_version(&self) -> Option<String> {
        zed::npm_package_installed_version(PACKAGE_NAME)
            .ok()
            .flatten()
    }

    fn should_install_or_update(&self, target_version: &str) -> bool {
        if !self.binary_exists() {
            return true;
        }

        match self.get_installed_version() {
            Some(installed_version) => installed_version != target_version,
            None => true,
        }
    }

    fn install_package(
        &mut self,
        id: &LanguageServerId,
        custom_version: Option<&str>,
    ) -> Result<()> {
        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let target_version = match custom_version {
            Some(version) => version.to_string(),
            None => zed::npm_package_latest_version(PACKAGE_NAME)?,
        };

        if self.should_install_or_update(&target_version) {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let result = zed::npm_install_package(PACKAGE_NAME, &target_version);
            if let Err(error) = result {
                if !self.binary_exists() {
                    return Err(error);
                }
            }
        }

        let binary_path = Self::get_native_binary_path()
            .map_err(|e| format!("Failed to locate native binary after installation: {}", e))?;

        Self::ensure_binary_is_usable(&binary_path, "Installed @effect/tsgo binary")?;

        // Cache the successful installation
        self.cached_binary_path = Some(binary_path.to_string_lossy().to_string());

        Ok(())
    }

    fn resolve_configured_binary_path(path: &str) -> Result<String> {
        let configured_path = PathBuf::from(path);

        if !configured_path.is_absolute() {
            return Err(format!(
                "Configured effect-tsgo binary path must be absolute: {}",
                configured_path.display()
            ));
        }

        Self::ensure_binary_is_usable(&configured_path, "Configured effect-tsgo binary")?;

        Ok(configured_path.to_string_lossy().to_string())
    }

    fn installed_binary_path(
        &mut self,
        id: &LanguageServerId,
        package_version: Option<&str>,
    ) -> Result<String> {
        // Return cached path if we have it and binary still exists
        if let Some(ref cached_path) = self.cached_binary_path {
            let cached_path = PathBuf::from(cached_path);
            if Self::ensure_binary_is_usable(&cached_path, "Cached @effect/tsgo binary").is_ok() {
                return Ok(cached_path.to_string_lossy().to_string());
            }
        }

        // Install or update package as needed
        self.install_package(id, package_version)?;

        let binary_path = Self::get_native_binary_path()
            .map_err(|e| format!("Failed to locate native binary: {}", e))?;

        Self::ensure_binary_is_usable(&binary_path, "Installed @effect/tsgo binary")?;

        Ok(binary_path.to_string_lossy().to_string())
    }

    fn binary_path(
        &mut self,
        id: &LanguageServerId,
        settings: &EffectTsgoSettings,
    ) -> Result<String> {
        self.invalidate_cache_if_settings_changed(settings);

        if let Some(binary_path) = settings.binary_path.as_deref() {
            return Self::resolve_configured_binary_path(binary_path);
        }

        self.installed_binary_path(id, settings.package_version.as_deref())
    }
}

impl zed::Extension for EffectTsgoExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
            cached_settings: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let lsp_settings = LspSettings::for_worktree("effect-tsgo", worktree).ok();

        let env = lsp_settings
            .as_ref()
            .and_then(|s| s.binary.as_ref())
            .and_then(|binary| binary.env.clone());

        let settings = lsp_settings
            .as_ref()
            .map(|s| EffectTsgoSettings::from_lsp_settings(s))
            .unwrap_or_default();

        let executable_path = self.binary_path(language_server_id, &settings)?;

        Ok(zed::Command {
            command: executable_path,
            args: vec!["--lsp".into(), "--stdio".into()],
            env: env.into_iter().flat_map(|env| env.into_iter()).collect(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(EffectTsgoExtension);
