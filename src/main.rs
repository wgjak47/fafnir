extern crate clap;
use clap::{App, Arg, SubCommand};

mod command;

fn main() {
    App::new("fafnir")
        .version("0.1")
        .about("manage your dot/config files with git")
        .author("wgjak47")
        .subcommand(
            SubCommand::with_name("add")
                .about("add a file to store")
                .arg(Arg::with_name("filename").takes_value(true))
                .arg(Arg::with_name("message").long("msg").takes_value(true))
                .arg(Arg::with_name("git-tag").long("git-tag").takes_value(true))
                .arg(Arg::with_name("tags").long("tags").takes_value(true)),
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
            SubCommand::with_name("distribute")
                .about("distribute your config file")
                .arg(Arg::with_name("tags").long("tags").takes_value(true))
                .arg(Arg::with_name("filename").long("filename").takes_value(true)),
        )
        .get_matches();
}
