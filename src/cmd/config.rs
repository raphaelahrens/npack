use crate::package;
use crate::utils;
use crate::{Error, Result};

use crate::cli::Config;
use std::fs;
use std::io::ErrorKind;

pub fn config(args: Config) -> Result<()> {
    let packs = package::fetch()?;
    let temp_pack = package::Package::new(&args.package, "temp", true);
    let pack = packs.iter().find(|x| args.package == x.name).unwrap_or(&temp_pack);

    let path = pack.config_path();

    let modified = match fs::metadata(&path) {
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                None
            } else {
                return Err(Error::Io(e));
            }
        }
        Ok(meta) => Some(meta.modified()?),
    };

    if modified.is_some() && args.delete {
        fs::remove_file(&path)?;
        return Ok(());
    }

    utils::open_editor(&path)?;

    let meta = match fs::metadata(&path) {
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Ok(());
            }
            return Err(Error::Io(e));
        }
        Ok(m) => m,
    };

    if meta.len() == 0 {
        fs::remove_file(&path)?;
        if modified.is_some() {
            package::update_pack_plugin(&packs)?;
        }
    } else if modified.is_none() || meta.modified()? > modified.unwrap() {
        package::update_pack_plugin(&packs)?;
    }
    Ok(())
}
