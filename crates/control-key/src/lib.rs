//! The control channel's key file, one rule for the app (which writes it) and the MCP bridge
//! (which reads it): `<settings>/control-key.local-<port>`. See `docs/control-protocol.md`, Keys.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

/// The app's settings folder (preferences, logs, key files): macOS
/// `~/Library/Application Support/GridCraft`, Windows `%APPDATA%\GridCraft`, otherwise
/// `$XDG_CONFIG_HOME/gridcraft` or `~/.config/gridcraft`.
pub fn settings_dir() -> Option<PathBuf> {
    if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Application Support/GridCraft"))
    } else if cfg!(windows) {
        std::env::var_os("APPDATA").map(|a| PathBuf::from(a).join("GridCraft"))
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
            .map(|c| c.join("gridcraft"))
    }
}

/// `<settings>/control-key.local-<port>`: the key file of the window listening on `port`.
pub fn key_path(settings: &Path, port: u16) -> PathBuf {
    settings.join(format!("control-key.local-{port}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_file_per_port() {
        let dir = Path::new("/settings/gridcraft");
        assert_eq!(key_path(dir, 7979), dir.join("control-key.local-7979"));
        assert_eq!(key_path(dir, 7980), dir.join("control-key.local-7980"));
    }

    #[test]
    fn settings_dir_is_the_apps_folder() {
        if let Some(d) = settings_dir() {
            let name = d.file_name().and_then(|n| n.to_str()).unwrap_or("");
            assert!(name == "gridcraft" || name == "GridCraft", "{}", d.display());
        }
    }
}
