mod config;
mod pacmanwriter;

use std::fmt::Debug;
use alpm::{Alpm};
use crate::pacmanwriter::Export;

fn main() {

    let mut pacman_config = pacmanconf::Config::options().pacman_conf("/etc/pacman.conf").read().expect("failed to parse /etc/pacman.conf");

    let c = config::Configuration::from_file("config.example.toml").expect("failed to parse config.toml");

    for i in c.config.repositories {
        let mut repo = pacmanconf::Repository::default();
        repo.name = i.name;
        repo.servers = i.server;
        repo.usage = match i.usage{
            Some(s ) => s.split(" ").map(|s| s.to_string()).collect(),
            None => Default::default()
        };
        repo.sig_level = i.sig_level.unwrap_or(Default::default()).split(" ").map(|s| s.to_string()).collect();

        pacman_config.repos.push(repo);
    }



    std::fs::write("ok.conf", pacman_config.export()).expect("failed to write file");


    let mut alpm = Alpm::new("/", "/var/lib/pacman").expect("pacman failed to be initialized");


}
