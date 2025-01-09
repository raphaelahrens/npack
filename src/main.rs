use std::env;
use clap::Parser;
use npack::cli;
use npack::cmd;

use color_eyre::eyre::Result;

fn main() -> Result<()>{
    let _ = env::var("PACK_LOG_FILE").map(|x| {
        simple_logging::log_to_file(x, log::LevelFilter::Info).expect("fail to init logging");
    });

    let app_m = cli::CliArgs::parse();
    let cmd = app_m.cmd;
    match cmd {
        cli::Command::List(args) => cmd::list::list_packages(args),
        cli::Command::Install(args)=> cmd::install::install_plugins(args),
        cli::Command::Uninstall(args)=> cmd::uninstall::exec(args),
        cli::Command::Config(args) => cmd::config::config(args),
        cli::Command::Move(args) => cmd::move_cmd::move_plugin(args),
        cli::Command::Update(args) => cmd::update::exec(args),
        cli::Command::Generate => cmd::generate::update_packfile(),
        cli::Command::Completions(_args) => {
            // TODO
            //let shell = m.value_of("SHELL").unwrap();
            // cli::build_cli().gen_completions_to("pack", shell.parse().unwrap(), &mut io::stdout());
            Ok(())
        }
    }?;
    Ok(())
}
