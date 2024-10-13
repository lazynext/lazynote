#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]

pub const OS_TYPE: &str = "linux";
#[cfg(target_os = "windows")]
pub const OS_TYPE: &str = "windows";
#[cfg(target_os = "macos")]
pub const OS_TYPE: &str = "macos";
#[cfg(target_os = "ios")]
pub const OS_TYPE: &str = "ios";
#[cfg(target_os = "android")]
pub const OS_TYPE: &str = "android";

/// - x86
/// - x86_64
/// - arm
/// - aarch64
/// - loongarch64
/// - m68k
/// - csky
/// - mips
/// - mips64
/// - powerpc
/// - powerpc64
/// - riscv64
/// - s390x
/// - sparc64
pub fn arch() -> &'static str {
    std::env::consts::ARCH
}

/// - unix
/// - windows
pub fn family() -> &'static str {
    std::env::consts::FAMILY
}

/// hostname
pub fn hostname() -> String {
    let hostname = gethostname::gethostname().to_string_lossy().to_string();
    hostname
}

/// - en-US
pub fn locale() -> Option<String> {
    sys_locale::get_locale()
}

/// - windows
/// - macos
/// - linux
/// - ios
/// - android
pub fn os_type() -> &'static str {
    OS_TYPE
}

/// - linux
/// - macos
/// - ios
/// - freebsd
/// - dragonfly
/// - netbsd
/// - openbsd
/// - solaris
/// - android
/// - windows
pub fn platform() -> &'static str {
    std::env::consts::OS
}

/// version
pub fn version() -> Option<String> {
    let s = match os_info::get().version().clone() {
        os_info::Version::Unknown => None,
        os_info::Version::Semantic(major, minor, patch) => {
            Some(format!("{}.{}.{}", major, minor, patch))
        }
        os_info::Version::Rolling(s) => s,
        os_info::Version::Custom(s) => Some(s),
    };
    s
}
