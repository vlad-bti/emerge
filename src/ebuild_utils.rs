use regex::Regex;
use std::fs;
use std::path::Path;
use std::result::Result;

use logos::{Lexer, Logos};

use crate::data::PackageNameInfo;
use crate::data::{
    Brackets, Conditional, EbuildInfo, PackageInfo, PackageVersion, VersionStatus, VersionType,
};

#[derive(Logos, Debug, PartialEq)]
enum Token<'a> {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

    #[regex(r"[\w-]+/[\w\+-\.]+")]
    PackageName,

    #[regex(r":[\w\.]+(/[\w\.]+)*[=\*]*", package_slot)]
    PackageSlot(&'a str),

    #[regex(r"\[[\w,\+-=!\?\(\) ]+\]", package_uses)]
    PackageUses(Vec<&'a str>),

    #[regex(r"[\w]+")]
    Uses,

    #[regex(r"[!=<>|?~]+", conditional)]
    Conditional(Conditional),

    #[regex(r"[()]", brackets)]
    Brackets(Brackets),
}

fn package_uses(lex: &mut Lexer<Token>) -> Option<Vec<&str>> {
    let slice = lex.slice();
    Some(slice.split(',').collect())
}
fn package_slot(lex: &mut Lexer<Token>) -> Option<&str> {
    let slice = lex.slice();
    Some(slice[1..])
}

fn conditional(lex: &mut Lexer<Token>) -> Option<Conditional> {
    let slice = lex.slice();
    match slice {
        Some("!") => Some(Conditional::WeakBlocker),
        Some("!!") => Some(Conditional::StrongBlocker),
        Some("!<") => Some(Conditional::BlockLess),
        Some("!>") => Some(Conditional::BlockGreater),
        Some("<") => Some(Conditional::Less),
        Some("=") => Some(Conditional::Equal),
        Some(">") => Some(Conditional::Greater),
        Some("=<") => Some(Conditional::LessOrEqual),
        Some("=>") => Some(Conditional::GreaterOrEqual),
        Some("||") => Some(Conditional::Or),
        Some("?") => Some(Conditional::If),
        _ => None,
    }
}

fn brackets(lex: &mut Lexer<Token>) -> Option<Brackets> {
    let slice = lex.slice();
    match slice {
        Some("(") => Some(Brackets::Open),
        Some(")") => Some(Brackets::Close),
    }
}

fn parse_depends(depends: &str) -> Vec<String> {
    let mut lex = Token::lexer(depends);

    let mut result = Vec::new();
    for token in lex {
        if let Token::PackageName = token {
            result.push(token.slice());
        }
    }

    result
}

// TODO: https://devmanual.gentoo.org/ebuild-writing/index.html
fn load_ebuild(ebuild_name: &str) -> Result<EbuildInfo, String> {
    lazy_static! {
        static ref EAPI_RE: Regex = Regex::new(r"EAPI=\x22*(?P<eapi>\d+)\x22*").unwrap();
        static ref SLOT_RE: Regex =
            Regex::new(r"SLOT=\x22(?P<slot>[\.\w]+)(/(?P<subslot>[\.\w-]+)*)?\x22").unwrap();
        static ref KEYWORDS_RE: Regex =
            Regex::new(r"(?m)KEYWORDS=\x22(?P<keywords>[\w\-\*~ ]+)\x22").unwrap();
        static ref DEPENDS_RE: Regex =
            Regex::new(r"(?m)DEPEND=\x22(?P<depends>[\w\-<>=!\?\n\*\+/\(\):|\[\] ]+)\x22").unwrap();
        static ref IUSE_RE: Regex = Regex::new(r"(?m)IUSE=\x22(?P<iuse>[\w\-\+)\x22").unwrap();
    }

    let content = fs::read_to_string(&ebuild_name)?;

    let eapi_cap = EAPI_RE.captures(&content).unwrap();
    let eapi_str = eapi_cap.name("eapi").map(|eapi| eapi.as_str());
    if let Ok(eapi) = eapi_str.parse::<u8>() {
        if eapi < 5 || eapi > 7 {
            return Err(format!("'{}' is not a valid EAPI. '{}'", eapi, ebuild_name));
        }
    } else {
        return Err(format!("EAPI must be defined. '{}'", ebuild_name));
    }

    let slot_cap = SLOT_RE.captures(&content).unwrap();
    let keywords_cap = KEYWORDS_RE.captures(&content).unwrap();
    let depends_cap = DEPENDS_RE.captures(&content).unwrap();
    let iuse_cap = IUSE_RE.captures(&content).unwrap();

    Ok(EbuildInfo {
        slot: slot_cap.name("slot").map(|slot| slot.as_str()),
        subslot: slot_cap.name("subslot").map(|subslot| subslot.as_str()),
        keywords: keywords_cap
            .name("keywords")
            .unwrap()
            .split_ascii_whitespace()
            .collect(),
        depends: parse_depends(depends_cap.name("depends").unwrap()),
        ises: iuse_cap
            .name("iuse")
            .unwrap()
            .split_ascii_whitespace()
            .collect(),
    })
}

fn get_package_name_re() -> String {
    // TODO subslot
    let _slot = r"([\w+][\w+.-]*)";
    let _cat = r"[\w+][\w+.-]*";
    let _pkg = r"[\w+][\w+-]*?";

    let _v = r"(\d+)((\.\d+)*)([a-z]?)((_(pre|p|beta|alpha|rc)\d*)*)";
    let _rev = r"\d+";
    let _vr = format!("{}(-r({}))?", _v, _rev);

    format!(
        "^((?P<cat>{})/)?(?P<name>{})(-(?P<ver>{}))?(:(?P<slot>{}))?$",
        _cat, _pkg, _vr, _slot
    )
}

// TODO: https://gitweb.gentoo.org/proj/portage.git/tree/lib/_emerge/is_valid_package_atom.py?h=portage-2.3.103
//      https://gitweb.gentoo.org/proj/portage.git/tree/lib/portage/dep/__init__.py?h=portage-2.3.103
//      https://gitweb.gentoo.org/proj/portage.git/tree/lib/portage/versions.py?h=portage-2.3.103
fn parse_package_name(package_name: &str) -> Result<PackageNameInfo, String> {
    lazy_static! {
        static ref PACKAGE_NAME_RE: Regex = Regex::new(get_package_name_re()).unwrap();
    }
    if !PACKAGE_NAME_RE.is_match(&package_name) {
        return Err(format!("'{}' is not a valid package atom.", package_name));
    }

    let cap = PACKAGE_NAME_RE.captures(input).unwrap();

    Ok(PackageNameInfo {
        category: cap.name("cat").map(|cat| cat.as_str()),
        name: cap.name("name").map(|name| name.as_str()),
        slot: cap.name("slot").map(|slot| slot.as_str()),
        version: cap.name("ver").map(|ver| ver.as_str()),
    })
}

fn get_category(package_name: &str) -> Result<Option(&str), String> {
    // TODO: config
    let path = Path::new("/usr/portage");
    // TODO: map() filter() dub
    let mut category_list = Vec::new();
    for entry in path.read_dir()? {
        let dir = entry?;
        if dir.path().is_dir() && dir.path().join(package_name).exists() {
            category_list.push(dir.file_name());
        }
    }

    if category_list.len() = 0 {
        return Err(format!(
            "there are no ebuilds to satisfy '{}'.",
            package_name
        ));
    } else if category_list.len() > 1 {
        return Err(format!(
            "The short ebuild name '{}' is ambiguous.",
            package_name
        ));
    }

    let category = category_list.pop()?.to_str().unwrap();
    Ok(Option(category))
}

fn get_ebuild_list(package_name_info: &PackageNameInfo) -> Result<Vec<&str>, String> {
    let cat = package_name_info.category.as_ref().copied().unwrap().0;
    let name = package_name_info.name.as_ref().copied().unwrap().0;
    let mut ver = "";
    if package_name_info.version.is_some() {
        ver = package_name_info.version.as_ref().copied().unwrap().0;
    }

    package_name_info.version.as_ref().copied();
    // TODO: config
    let path = Path::new(format!("/usr/portage/{}/{}", cat, name).as_str());
    // TODO: map() filter() dub
    let mut ebuild_list = Vec::new();
    for entry in path.read_dir()? {
        let file = entry?;
        if file.path().is_file() {
            let file_name = file.file_name().to_str().unwrap();
            // TODO: config
            if !file_name.contains(".ebuild") {
                continue;
            }
            if package_name_info.version.is_some() && !file_name.contains(ver) {
                continue;
            }

            ebuild_list.push(file_name);
        }
    }

    if ebuild_list.len() = 0 {
        return Err(format!(
            "there are no ebuilds to satisfy '{}'.",
            package_name
        ));
    }

    Ok(ebuild_list)
}

pub fn load_package_info(package_name: &str) -> Result<PackageInfo, String> {
    let mut package_name_info = parse_package_name(package_name)?;
    if package_name_info.category.is_none() {
        let name_copy = package_name_info.name.as_ref().copied();
        package_name_info.category = get_category(name_copy.unwrap().0)?;
    }

    let ebuild_list = get_ebuild_list(&package_name_info)?;
    let mut package_info = PackageInfo {
        name: package_name_info.name.unwrap().0,
        slot: package_name_info.slot.unwrap().0,
        subslot: None,
        installed_version: None,
        version_list: vec![],
        version_need_list: vec![],
        use_need_list: vec![],
    };
    for ebuild in ebuild_list {
        let ebuild_info = load_ebuild(&ebuild.0)?;
        let ebuild_name_info =
            parse_package_name(Path::new(&ebuild.0).file_stem().unwrap().to_str().unwrap())?;
        let package_version = PackageVersion {
            version: ebuild_name_info.version.unwrap().0,
            version_type: VersionType::Stable,
            version_status: VersionStatus::Unchanged,
            use_list: ebuild_info.ises,
            use_set_list: vec![],
            depends_list: ebuild_info.depends,
        };

        package_info.version_list.push(package_version);
    }
    Ok(package_info)
}
