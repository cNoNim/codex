/// The current Codex CLI version as embedded at compile time.
#[cfg(fork_cnonim)]
pub const CODEX_CLI_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-", env!("CODEX_FORK_SUFFIX"));

#[cfg(not(fork_cnonim))]
pub const CODEX_CLI_VERSION: &str = env!("CARGO_PKG_VERSION");
