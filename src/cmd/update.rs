use crate::git;
use crate::package::{self, Package};
use crate::task::{TaskManager, TaskType};
use crate::cli::Update;
use crate::{Error, Result};

pub fn exec(args: Update) -> Result<()>{

    if args.packfile {
        return update_packfile();
    }

    let threads = args.threads.unwrap_or_else(num_cpus::get);

    update_plugins(&args.package, threads, &args.skip)
}

fn update_packfile() -> Result<()> {
    println!("Update _pack file for all plugins.");
    let mut packs = package::fetch()?;

    packs.sort_by(|a, b| a.name.cmp(&b.name));
    package::update_pack_plugin(&packs)?;

    Ok(())
}

fn update_plugins(plugins: &[String], threads: usize, skip: &[String]) -> Result<()> {
    let mut packs = package::fetch()?;

    let mut manager = TaskManager::new(TaskType::Update, threads);
    if plugins.is_empty() {
        for pack in &packs {
            if skip.iter().any(|x| pack.name.contains(x)) {
                println!("Skip {}", pack.name);
                continue;
            }
            manager.add(pack.clone());
        }
    } else {
        for pack in packs.iter().filter(|x| plugins.contains(&x.name)) {
            manager.add(pack.clone());
        }
    }

    for fail in manager.run(update_plugin)? {
        packs.retain(|e| e.name != fail);
    }

    packs.sort_by(|a, b| a.name.cmp(&b.name));

    package::update_pack_plugin(&packs)?;

    Ok(())
}

fn update_plugin(pack: &Package) -> (Result<()>, bool) {
    let res = do_update(pack);
    let status = match res {
        Err(Error::SkipLocal) | Err(Error::Git(_)) => true,
        Err(_) => false,
        _ => true,
    };
    (res, status)
}

fn do_update(pack: &Package) -> Result<()> {
    let path = pack.path();
    if !path.is_dir() {
        Err(Error::PluginNotInstalled)
    } else if pack.local {
        Err(Error::SkipLocal)
    } else {
        git::update(&pack.name, &path)
    }
}
