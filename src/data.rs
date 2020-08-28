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

pub struct PackageVersion<'a> {
    pub version: &'a str,
    pub version_type: VersionType,
    pub version_status: VersionStatus,
    pub use_list: Vec<String>,
    pub use_set_list: Vec<String>,
    pub depends_list: Vec<String>,
}

pub struct PackageInfo<'a> {
    pub name: &'a str,
    pub slot: &'a str,
    pub subslot: Option(String),
    pub installed_version: Option(PackageVersion),
    pub version_list: Vec<PackageVersion<'a>>,
    pub version_need_list: Vec<PackageVersion<'a>>,
    pub use_need_list: Vec<String>,
}

pub struct PackageNameInfo {
    pub category: Option(&str),
    pub name: Option(&str),
    pub slot: Option(&str),
    pub version: Option(&str),
}

pub struct EbuildInfo {
    pub slot: Option(&str),
    pub subslot: Option(&str),
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
