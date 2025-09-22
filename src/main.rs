use deliver::pkg_info::PkgInfo;

fn main() {
    let pi = PkgInfo::new();
    println!("Package Name: {}", pi.get_pkg_name());
    println!("Package Version: {}", pi.get_pkg_version());
    println!("Package Authors: {}", pi.get_pkg_authors());
    println!("Package cache dir: {:?}", pi.get_cache_dir());
    println!("Package config dir: {:?}", pi.get_config_dir());
}