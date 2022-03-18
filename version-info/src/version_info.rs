use crate::git_utils::parse_git_version;
use std::fmt::{Display, Formatter};

/// Version information for source code.
#[derive(Debug, Clone)]
pub struct VersionInfo {
    /// Commit hash on which the product was built.
    pub commit_hash: String,

    /// Version tag on which the product was built.
    pub version_tag: String,

    /// Cargo package name.
    pub pkg_name: String,

    /// Cargo package description.
    pub pkg_description: String,

    /// Cargo package version.
    pub pkg_version: String,

    /// Binary name.
    pub bin_name: String,

    /// Build type (empty for release, "debug" for debug).
    pub build_type: String,
}

impl From<&VersionInfo> for String {
    fn from(v: &VersionInfo) -> Self {
        format!("{}", v)
    }
}

impl From<VersionInfo> for String {
    fn from(v: VersionInfo) -> Self {
        From::from(&v)
    }
}

impl Display for VersionInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "version {}, revision {} ({})",
            self.pkg_version, self.commit_hash, self.version_tag
        )
    }
}

impl VersionInfo {
    /// Creates new VersionInfo instance.
    pub fn new(
        git_ver: String,
        pkg_name: String,
        pkg_description: String,
        pkg_version: String,
        bin_name: Option<String>,
        build_type: String,
    ) -> Self {
        let p = parse_git_version(&git_ver);

        let pkg_name = if pkg_name.is_empty() {
            String::from("unknown")
        } else {
            pkg_name
        };

        VersionInfo {
            commit_hash: p.2,
            version_tag: p.0,
            pkg_name,
            pkg_description,
            pkg_version,
            bin_name: bin_name.unwrap_or_else(|| "".to_string()),
            build_type,
        }
    }

    /// Formats package name and description.
    pub fn fmt_description(&self) -> String {
        let s = if self.pkg_description.is_empty() {
            format!("Application '{}'", self.pkg_name)
        } else if !self.bin_name.is_empty() && self.bin_name != self.pkg_name {
            format!("{} ({})", self.pkg_description, self.bin_name)
        } else {
            self.pkg_description.clone()
        };

        if self.build_type.is_empty() {
            s
        } else {
            format!("{} ({})", s, self.build_type)
        }
    }
}