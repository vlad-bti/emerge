struct PackageNameInfo {
    group: String,
    name: String,
    version: String,
    suffix: String,
    revision: String,
}

fn load_ebuild(ebuild_name: String) {
    parse_eapi()?;
    parse_keywords()?;
    parse_slot_subslot()?;
    parse_depends()?;
    parse_uses()?;
}

pub fn load_package_info(package_name: String) {
    parse_package_name()?;
    find_package()?;
    let mut package_version_list = Vec::new();
    if is_version_specify {
        package_version_list.push(version);
    } else {
        package_version_list.append(version_list);
    }
    for version in package_version_list {
        load_ebuild(verson)?;
    }
}
