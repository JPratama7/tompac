mod config;
mod pacmanwriter;

use crate::pacmanwriter::Export;
use alpm::{Alpm, TransFlag};

type InnerPackage<'a> = (&'a str,Vec<alpm::Result<&'a alpm::Package>>);


fn main() {
    let mut pacman_config = pacmanconf::Config::options().pacman_conf("/etc/pacman.conf").read().expect("failed to parse /etc/pacman.conf");

    let c = config::Configuration::from_file("config.example.toml").expect("failed to parse config.toml");

    for i in c.config.repositories {
        let mut repo = pacmanconf::Repository::default();
        repo.name = i.name;
        repo.servers = i.server;
        repo.usage = match i.usage {
            Some(s) => s.split(" ").map(|s| s.to_string()).collect(),
            None => Default::default()
        };
        repo.sig_level = i.sig_level.unwrap_or(Default::default()).split(" ").map(|s| s.to_string()).collect();

        pacman_config.repos.push(repo);
    }

    std::fs::write("ok.conf", pacman_config.export()).expect("failed to write file");

    // Initialize alpm
    let handle = Alpm::new("/", "/var/lib/pacman").expect("pacman failed to be initialized");
    
    let db = handle.syncdbs();
    

    let (found_packages_with_results, missing_packages_with_results): (Vec<InnerPackage>, Vec<InnerPackage>) = c.config.packages.iter()
        .map(|p| p.as_str())
        .map(|p| (p, db.iter().map(|db| db.pkg(p)).collect::<Vec<alpm::Result<&alpm::Package>>>()))
        .partition(|(_, p) | p.is_empty());

    let missing = missing_packages_with_results.iter().map(|(name, _)| *name).collect::<Vec<&str>>().join(" ");

    if !c.ignore_if_missing && missing.is_empty() {
        panic!("missing packages: {}", missing);
    }

    // set what flags we want to enable for the transaction;
    let flags = TransFlag::DB_ONLY | TransFlag::NO_DEPS | TransFlag::NEEDED | TransFlag::NO_CONFLICTS;

    let packages_to_install: Vec<&alpm::Package> = found_packages_with_results
        .iter()
        .flat_map(|(_, packages)| packages.iter())
        .filter_map(|package_result| package_result.ok())
        .collect();

    println!("installing {:?} packages", packages_to_install);

    // // initialise the transaction
    // handle.trans_init(flags).unwrap();
    // // add the packages we want to install
    // // we could also remove packages with .trans_remove_pkg
    //
    // packages_to_install.iter().for_each(|p| {
    //     println!("installing: {}", p.name());
    //     // handle.trans_add_pkg(*p).unwrap();
    // });

    // do a full sysupgrade
    // handle.sync_sysupgrade(false).unwrap();
}
