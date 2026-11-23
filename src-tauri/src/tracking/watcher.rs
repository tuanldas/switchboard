//! Phase 2 file watching module.
//!
//! This module will watch provider log directories for changes (e.g., Claude,
//! Cursor, GitHub Copilot log files) and trigger parsing or notifications when
//! updates are detected.

use std::path::PathBuf;

/// Watches provider log directories for file system changes.
#[derive(Debug, Clone, Default)]
pub struct LogWatcher {
    /// Paths to the provider log directories or files currently being watched.
    pub watched_paths: Vec<PathBuf>,
}

impl LogWatcher {
    /// Creates a new empty `LogWatcher`.
    pub fn new() -> Self {
        Self {
            watched_paths: Vec::new(),
        }
    }

    /// Starts watching the configured log directories for changes.
    pub fn start(&self) -> anyhow::Result<()> {
        log::info!("LogWatcher: not yet implemented");
        Ok(())
    }

    /// Stops watching the configured log directories.
    pub fn stop(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
