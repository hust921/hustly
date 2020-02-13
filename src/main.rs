#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

mod modules;
use modules::{ModuleCollection, ModuleOperation, ZshModule};

fn main() {
    let mod_col = ModuleCollection { modules: vec![
        Box::new(ZshModule::new()),
    ]};

    let _ = mod_col.operation(ModuleOperation::Install);

    //modules::add_module(Zsh::new());
    let arguments = App::new("hustly dotfiles installer")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Personal dotfiles installer")
        .arg(Arg::with_name("Version")
                .short("V")
                .long("version")
                .help("Print version info and exit"),
        )
        .arg(Arg::with_name("Verbose")
                .short("v")
                .long("verbose")
                .help("Use verbose output (-vv very verbose)")
                .multiple(true)
        )
        .arg(Arg::with_name("Quiet")
                .short("q")
                .long("quiet")
                .help("No output printed to stdout")
        )
        .arg(Arg::with_name("Dry-Run")
                .short("n")
                .long("dry-run")
                .help("Run without execution")
        )
        .arg(Arg::with_name("SystemInstall")
                .long("all")
                .help("Install for all users on system")
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Install module/environment/dependency")
                .arg(Arg::with_name("module")
                        .multiple(true)
                        .required(true)
                        .takes_value(true)
                        //.value_names(modules::names)
                )
        )
        .get_matches();

    println!("SubCommand arguments: {:?}", arguments.subcommand_matches("install"));
}
