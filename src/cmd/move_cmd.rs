use crate::package;
use crate::utils;
use crate::{Error, Result};

use std::fs;

use crate::cli::Move;

pub fn move_plugin(args: Move) -> Result<()> {
    let mut packs = package::fetch()?;
    let changed = {
        let pack = match packs.iter_mut().find(|p| p.name == args.package) {
            Some(p) => p,
            None => return Err(Error::PluginNotInstalled),
        };

        let origin_path = pack.path();
        if !origin_path.is_dir() {
            return Err(Error::PluginNotInstalled);
        }

        let path = package::Package::new(&args.package, &args.category, args.opt).path();
        if origin_path != path {
            utils::copy_directory(&origin_path, &path)?;
            fs::remove_dir_all(&origin_path)?;
            pack.set_category(&args.category);
            pack.set_opt(args.opt);
            true
        } else {
            false
        }
    };

    if changed {
        packs.sort_by(|a, b| a.name.cmp(&b.name));
        package::save(packs)?;
    }
    Ok(())
}
