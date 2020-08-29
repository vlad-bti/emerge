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
    pub subslot: Option<String>,
    pub installed_version: Option<PackageVersion<'a>>,
    pub version_list: Vec<PackageVersion<'a>>,
    pub version_need_list: Vec<PackageVersion<'a>>,
    pub use_need_list: Vec<String>,
}

pub struct PackageNameInfo<'a> {
    pub category: Option<&'a str>,
    pub name: Option<&'a str>,
    pub slot: Option<&'a str>,
    pub version: Option<&'a str>,
}

pub struct EbuildInfo<'a> {
    pub slot: Option<&'a str>,
    pub subslot: Option<&'a str>,
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
