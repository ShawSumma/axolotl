pub mod runtime;
/// axolotl version of stable-x86_64-pc-windows-gnu
pub struct Toolchain {
    stability: String,
    arch: String,
    motherboard: String,
    operating_system: String,
    backend: String,
}

/// to be determined
pub struct ToolchainConfig {
    name: String,
}
