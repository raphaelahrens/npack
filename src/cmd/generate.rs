use crate::package;
use crate::Result;

pub fn update_packfile() -> Result<()> {
    let mut packs = package::fetch()?;

    packs.sort_by(|a, b| a.name.cmp(&b.name));
    package::update_pack_plugin(&packs)?;

    Ok(())
}
