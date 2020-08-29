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
    pub depends_list: Vec<String>,
}

pub struct PackageInfo {
    pub name: String,
    pub slot: String,
    pub subslot: Option<String>,
    pub installed_version: Option<PackageVersion>,
    pub version_list: Vec<PackageVersion>,
    pub version_need_list: Vec<PackageVersion>,
    pub use_need_list: Vec<String>,
}

pub struct PackageNameInfo {
    pub category: Option<String>,
    pub name: Option<String>,
    pub slot: Option<String>,
    pub version: Option<String>,
}

pub struct EbuildInfo {
    pub slot: Option<String>,
    pub subslot: Option<String>,
    pub keywords: Vec<String>,
    pub depends: Vec<String>,
    pub ises: Vec<String>,
}

pub enum Brackets {
    Open,
    Close,
}

pub enum Conditional {
    WeakBlocker,
    StrongBlocker,
    BlockLess,
    BlockGreater,
    Less,
    Equal,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    Or,
    If,
}
