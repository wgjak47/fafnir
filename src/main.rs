extern crate clap;
extern crate shellexpand;
#[macro_use] extern crate failure;

use clap::{App, Arg, SubCommand};
use command::run_command;
use std::process::exit;

mod command;

fn main() {
   let options = App::new("fafnir")
        .version("0.1")
        .about("manage your dot/config files with git")
        .author("wgjak47")
        .subcommand(
            SubCommand::with_name("add")
                .about("add a file to store")
                .arg(Arg::with_name("filepath").takes_value(true).help("the file path, will auto convert to the Absolute Path"))
                .arg(Arg::with_name("tags").long("tags").takes_value(true).help("set custom tags for this file, default is the common setting")),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("remove a config file and link, and restore the origin file")
                .arg(Arg::with_name("filepath").takes_value(true).help("the file path, will auto convert to the Absolute Path"))
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("show the stored config files")
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("init git repo")
                .arg(Arg::with_name("url").takes_value(true).help("the git repo remote url"))
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("config this machine for tags, store upstream url, store location")
                .arg(Arg::with_name("url").long("set-url").takes_value(true))
                .arg(Arg::with_name("list").long("list"))
                .arg(Arg::with_name("path").long("set-path").takes_value(true))
                .arg(Arg::with_name("tags").long("set-tags").takes_value(true)),
        )
        .subcommand(SubCommand::with_name("pull").about("pull from the remote"))
        .subcommand(SubCommand::with_name("push").about("push local store to remote"))
        .subcommand(
            SubCommand::with_name("link")
                .about("create link your config file")
                .arg(Arg::with_name("tags").long("tags").takes_value(true))
                .arg(Arg::with_name("filename").long("filename").takes_value(true)),
        )
        .get_matches();

    match run_command(options) {
        Ok(()) => {},
        Err(e) => {
            eprint!("{}", e);
            exit(250);
        }
    }
}
