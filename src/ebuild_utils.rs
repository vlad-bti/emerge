#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

struct PackageNameInfo {
    category: Option(&str),
    name: Option(&str),
    slot: Option(&str),
    version: Option(&str),
}

fn load_ebuild(ebuild_name: String) {
    parse_eapi()?;
    parse_keywords()?;
    parse_slot_subslot()?;
    parse_depends()?;
    parse_uses()?;
}

// TODO: https://gitweb.gentoo.org/proj/portage.git/tree/lib/_emerge/is_valid_package_atom.py?h=portage-2.3.103
//      https://gitweb.gentoo.org/proj/portage.git/tree/lib/portage/dep/__init__.py?h=portage-2.3.103
//      https://gitweb.gentoo.org/proj/portage.git/tree/lib/portage/versions.py?h=portage-2.3.103
fn parse_package_name(package_name: String) -> Result<PackageNameInfo, String> {
    lazy_static! {
        let _slot = "([\w+][\w+.-]*)";
        let _cat = "[\w+][\w+.-]*";
        let _pkg = "[\w+][\w+-]*?";

        let _v = "(\d+)((\.\d+)*)([a-z]?)((_(pre|p|beta|alpha|rc)\d*)*)";
        let _rev = "\d+";
        let _vr = format!("{}(-r({}))?", _v, _rev);

        let _cp = format!("^((?P<cat>{})/)?(?P<name>{})(-(?P<ver>{}))?(:(?P<slot>{}))?$", _cat, _pkg, _vr, _slot);

        static ref RE: Regex = Regex::new(_cp).unwrap();
    }
    if !RE.is_match(&package_name) {
        return Err(format!("'{}' is not a valid package atom.", package_name));
    }

    let cap = RE.captures(input).unwrap();

    Ok(PackageNameInfo {
        category: cap.name("cat").map(|cat| cat.as_str()),
        name: cap.name("name").map(|name| name.as_str()),
        slot: cap.name("slot").map(|slot| slot.as_str()),
        version: cap.name("ver").map(|ver| ver.as_str()),
    })
}

fn get_category(package_name: &str) -> Result<Option(&str), String> {
// TODO: config
    let path= "/usr/portage/";

}

fn get_version_list(package_name_info: &PackageNameInfo)-> Result<Vec<&str>, String> {

}

pub fn load_package_info(package_name: String) {
    let mut package_name_info = parse_package_name(package_name)?;
    if package_name_info.category.is_none() {
        let name_copy = package_name_info.name.as_ref().copied();
        package_name_info.category = get_category(name_copy.unwrap().0)?;
    }

    let version_list = get_version_list(&package_name_info)?;

    let mut package_version_list = Vec::new();
    if let Some(version) = package_name_info.version {
        package_version_list.push(version);
    } else {
        package_version_list.extend(version_list);
    }
    for version in package_version_list {
        load_ebuild(verson)?;
    }
}
