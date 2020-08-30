use regex::Regex;
use std::fs;
use std::path::Path;
use std::result::Result;

use logos::Logos;

use crate::data::{
    EbuildInfo, PackageInfo, PackageNameInfo, PackageVersion, VersionStatus, VersionType,
};

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

    #[regex(r"[\w-]+/[\w\+-\.]+")]
    PackageName,

    #[regex(r":[\w\.]+(/[\w\.]+)*[=\*]*")]
    PackageSlot,

    #[regex(r"\[[\w,\+-=!\?\(\) ]+\]")]
    PackageUses,

    #[regex(r"[\w]+")]
    Uses,

    #[regex(r"[!=<>|?~]+")]
    Conditional,

    #[regex(r"[()]")]
    Brackets,
}
/*
fn package_uses(lex: &mut Lexer<Token>) -> Option<Vec<&str>> {
    let slice = lex.slice();
    Some(slice.split(',').collect())
}
fn package_slot<'a>(lex: &mut Lexer<Token>) -> Option<&'a str> {
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
*/

fn parse_depends(depends: &str) -> Vec<String> {
    let mut lex = Token::lexer(depends);

    let mut result = Vec::new();
    loop {
        let token = lex.next();
        if let Some(Token::PackageName) = token {
            result.push(String::from(lex.slice()));
        } else if let None = token {
            break;
        }
    }

    result
}

// TODO: https://devmanual.gentoo.org/ebuild-writing/index.html
fn load_ebuild(path: &str, ebuild_name: &str) -> Result<EbuildInfo, String> {
    lazy_static! {
        static ref EAPI_RE: Regex = Regex::new(r"EAPI=\x22*(?P<eapi>\d+)\x22*").unwrap();
        static ref SLOT_RE: Regex =
            Regex::new(r"SLOT=\x22(?P<slot>\w*?)(/(?P<subslot>\w*?)*)?\x22").unwrap();
        static ref KEYWORDS_RE: Regex =
            Regex::new(r"(?m)KEYWORDS=\x22(?P<keywords>[\w\W]*?)\x22").unwrap();
        static ref DEPENDS_RE: Regex =
            Regex::new(r"(?m)\nDEPEND=\x22(?P<depends>[\w\W]*?)\x22").unwrap();
        static ref IUSE_RE: Regex = Regex::new(r"(?m)IUSE=\x22(?P<iuse>[\w\W]*?)\x22").unwrap();
    }

    let result = fs::read_to_string(Path::new(&path).join(&ebuild_name));
    if let Err(e) = result {
        return Err(format!("'{}' - {}", ebuild_name, e.to_string()));
    }

    let content = result.unwrap();

    let eapi_cap = EAPI_RE.captures(&content).unwrap();
    let eapi_str = eapi_cap.name("eapi").map(|eapi| eapi.as_str()).unwrap();
    if let Ok(eapi) = eapi_str.parse::<u8>() {
        if eapi < 5 || eapi > 7 {
            return Err(format!("'{}' is not a valid EAPI. '{}'", eapi, ebuild_name));
        }
    } else {
        return Err(format!("EAPI must be defined. '{}'", ebuild_name));
    }

    let mut ebuild_info = EbuildInfo {
        slot: Some("0".into()),
        subslot: None,
        keywords: vec![],
        depends: vec![],
        ises: vec![],
    };

    let slot_cap = SLOT_RE.captures(&content);
    let keywords_cap = KEYWORDS_RE.captures(&content);
    let depends_cap = DEPENDS_RE.captures(&content);
    let iuse_cap = IUSE_RE.captures(&content);

    if let Some(cap) = slot_cap {
        ebuild_info.slot = cap.name("slot").map(|slot| slot.as_str().into());
        ebuild_info.subslot = cap.name("subslot").map(|subslot| subslot.as_str().into());
    }

    if let Some(cap) = keywords_cap {
        ebuild_info.keywords = cap
            .name("keywords")
            .map(|keywords| keywords.as_str())
            .unwrap()
            .split_ascii_whitespace()
            .map(|keywords| keywords.into())
            .collect();
    }

    if let Some(cap) = depends_cap {
        ebuild_info.depends =
            parse_depends(cap.name("depends").map(|depends| depends.as_str()).unwrap());
    }

    if let Some(cap) = iuse_cap {
        ebuild_info.ises = cap
            .name("iuse")
            .map(|iuse| iuse.as_str())
            .unwrap()
            .split_ascii_whitespace()
            .map(|iuse| iuse.into())
            .collect();
    }

    Ok(ebuild_info)
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
        static ref PACKAGE_NAME_RE: Regex = Regex::new(get_package_name_re().as_str()).unwrap();
    }
    if !PACKAGE_NAME_RE.is_match(&package_name) {
        return Err(format!("'{}' is not a valid package atom.", package_name));
    }

    let cap = PACKAGE_NAME_RE.captures(package_name).unwrap();

    Ok(PackageNameInfo {
        category: cap.name("cat").map(|cat| cat.as_str().into()),
        name: cap.name("name").map(|name| name.as_str().into()),
        slot: Some(
            cap.name("slot")
                .map_or_else(|| String::from("0"), |slot| slot.as_str().into()),
        ),
        version: cap.name("ver").map(|ver| ver.as_str().into()),
    })
}

fn get_category(package_name: &str) -> Result<Option<String>, String> {
    // TODO: config
    let path = Path::new("/usr/portage");
    // TODO: map() filter() dub
    let mut category_list = Vec::new();
    let dir_content = path.read_dir();
    if let Err(e) = dir_content {
        return Err(e.to_string());
    }
    for entry in dir_content.unwrap() {
        let dir = entry.unwrap();

        if dir.path().is_dir() && dir.path().join(package_name).exists() {
            category_list.push(dir.file_name());
        }
    }

    if category_list.is_empty() {
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

    let category = category_list.pop().unwrap().into_string().unwrap();
    Ok(std::option::Option::Some(category))
}

fn get_ebuild_list(package_name_info: &PackageNameInfo) -> Result<Vec<String>, String> {
    let cat = package_name_info.category.clone().unwrap();
    let name = package_name_info.name.clone().unwrap();
    let mut ver = String::from("");
    if package_name_info.version.is_some() {
        ver = package_name_info.version.clone().unwrap();
    }

    // TODO: config
    let path = format!("/usr/portage/{}/{}", cat, name);
    // TODO: map() filter() dub
    let mut ebuild_list = Vec::new();
    let dir_content = Path::new(path.as_str()).read_dir();
    if let Err(e) = dir_content {
        return Err(e.to_string());
    }

    for entry in dir_content.unwrap() {
        let dir = entry.unwrap();
        if dir.path().is_file() {
            let file_name = dir.file_name().into_string().unwrap();
            // TODO: config
            if !file_name.contains(".ebuild") {
                continue;
            }
            //if package_name_info.version.is_some() && !file_name.contains(ver.as_str()) {
            //    continue;
            //}

            ebuild_list.push(file_name);
        }
    }

    if ebuild_list.is_empty() {
        return Err(format!("there are no ebuilds to satisfy '{}'.", name));
    }

    Ok(ebuild_list)
}

pub fn load_package_info(package_name: &str) -> Result<PackageInfo, String> {
    let mut package_name_info = parse_package_name(package_name)?;
    if package_name_info.category.is_none() {
        package_name_info.category =
            get_category(package_name_info.name.clone().unwrap().as_str())?;
    }

    let ebuild_list = get_ebuild_list(&package_name_info)?;
    let mut package_info = PackageInfo {
        name: package_name_info.name.clone().unwrap(),
        slot: package_name_info.slot.clone().unwrap(),
        subslot: None,
        installed_version: None,
        version_list: vec![],
        version_need_list: vec![],
        use_need_list: vec![],
    };

    let cat = package_name_info.category.clone().unwrap();
    let name = package_name_info.name.clone().unwrap();

    // TODO: config
    let path = format!("/usr/portage/{}/{}", cat, name);
    for ebuild in ebuild_list {
        let ebuild_info = load_ebuild(&path, &ebuild)?;
        let ebuild_name_info = parse_package_name(
            Path::new(ebuild.as_str())
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap(),
        )?;
        let package_version = PackageVersion {
            version: ebuild_name_info.version.unwrap(),
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
