//! Static build info for Native Instruments Reaktor 6 Blocks.

pub const BUILD_VERSION: &str  = "v2.6.22";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "6d38acc938fa";
pub const BUILD_TIMESTAMP: u64 = 1791053225;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Native Instruments Reaktor 6 Blocks", BUILD_VERSION, BUILD_CHANNEL)
}
