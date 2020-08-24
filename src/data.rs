pub enum VersionType {
    Stable,
    Unstable,
    Masked,
}

pub enum VersionStatus {
    Unchanged,
    MaskedByUser,
    UnmaskedByUser,
}

pub struct PackageVersion {
    pub version: String,
    pub version_type: VersionType,
    pub version_status: VersionStatus,
    pub use_list: Vec<String>,
    pub use_set_list: Vec<String>,
}

pub struct PackageInfo {
    pub name: String,
    pub slot: String,
    pub subslot: Option(String),
    pub installed_version: Option(PackageVersion),
    pub version_list: Vec<PackageVersion>,
    pub version_need_list: Vec<PackageVersion>,
    pub use_need_list: Vec<String>,
}

struct PackageNameInfo {
    category: Option(&str),
    name: Option(&str),
    slot: Option(&str),
    version: Option(&str),
}
