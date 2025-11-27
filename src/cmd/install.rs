use crate::git;
use crate::package::{self, Package};
use crate::task::{TaskManager, TaskType};
use crate::{Error, Result};

use num_cpus;
use std::os::unix::fs::symlink;
use std::path::Path;

pub fn install_plugins(args: crate::cli::Install) -> Result<()> {
    let threads = args.threads.unwrap_or_else(num_cpus::get);

    let opt = args.on.is_some() || args.for_.is_some() || args.opt;
    let types = args
        .for_
        .map(|e| e.split(',').map(std::string::ToString::to_string).collect::<Vec<String>>());

    let mut packs = package::fetch()?;
    {
        let mut manager = TaskManager::new(TaskType::Install, threads);
        
        // If no package names have been given as an argument try to install all plugins.
        match args.package {
            None => {
                for pack in &packs {
                    manager.add(pack.clone());
                }
            },
            Some(names) => {
                let mut pack = {
                    let mut p = Package::new(&names, &args.category, opt);
                    p.local = if Path::new(&names).is_dir() {
                        true
                    } else {
                        args.local
                    };
                    if let Some(ref c) = args.on {
                        p.set_load_command(c);
                    }
                    if let Some(ref t) = types {
                        p.set_types(t.clone());
                    }
                    if let Some(ref c) = args.build {
                        p.set_build_command(c);
                    }
                    if let Some(ref c) = args.branch {
                        p.set_branch(c);
                    }
                    p
                };
                let having = match packs.iter_mut().find(|x| x.name == pack.name) {
                    Some(x) => {
                        if x.is_installed() {
                            pack.set_category(x.category.as_str());
                            pack.set_opt(x.opt);
                        } else {
                            x.set_category(pack.category.as_str());
                            x.set_opt(pack.opt);
                            x.set_types(pack.for_types.clone());

                            x.load_command = pack.load_command.clone();
                            x.build_command = pack.build_command.clone();
                        }
                        true
                    }
                    None => false,
                };
                if !having {
                    packs.push(pack.clone());
                }
                manager.add(pack);
            }
        }

        for fail in manager.run(install_plugin)? {
            packs.retain(|e| e.name != fail);
        }
    }

    packs.sort_by(|a, b| a.name.cmp(&b.name));

    package::update_pack_plugin(&packs)?;
    package::save(packs)
}

fn install_plugin(pack: &Package) -> (Result<()>, bool) {
    let res = do_install(pack);
    let status = match res {
        Err(Error::PluginInstalled(_)) => true,
        Err(_) => false,
        Ok(()) => true,
    };
    (res, status)
}

fn do_install(pack: &Package) -> Result<()> {
    let path = pack.path();
    if path.is_dir() {
        Err(Error::plugin_installed(&path))
    } else if pack.local {
        let src = Path::new(&pack.name);
        if !src.is_dir() {
            Err(Error::NoPlugin)
        } else {
            symlink(src, &path)?;
            Ok(())
        }
    } else {
        git::clone(&pack.name, &path, &pack.branch)
    }
}
