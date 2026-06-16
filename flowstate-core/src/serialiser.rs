//! Loading and saving of environment data.
//!
//! Environments are declarative RON files (render doc §7). Shared custom
//! environments are exported as `.flowenv` (small JSON, no bundled assets —
//! FlowState §11.3). Session-data persistence (SQLite) arrives in Phase 2.

use crate::environment::Environment;

/// Parse an `Environment` from a RON document.
pub fn load_environment_ron(src: &str) -> Result<Environment, ron::error::SpannedError> {
    ron::from_str(src)
}

/// Serialise an `Environment` to a pretty RON document.
pub fn save_environment_ron(env: &Environment) -> Result<String, ron::Error> {
    ron::ser::to_string_pretty(env, ron::ser::PrettyConfig::default())
}

// TODO: `.flowenv` JSON import/export for shared community environments
// (FlowState §11.3), and SQLite-backed session storage (Phase 2).
