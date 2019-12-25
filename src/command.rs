use clap::ArgMatches;
use failure::Error;
use std::path::Path;


pub trait FaFnirCommand {
    fn init(url: &String);
    fn show();
    fn add(file_path: Path, tags: &Vec<String>);
    fn remove(file_path: Path);
    fn config(url: &str, tags: &Vec<String>);
    fn pull();
    fn push();
    fn link(file_path: &str, tags: &Vec<String>);
}

pub struct FaFnirCommandV1 {
    pub tags: Vec<String>,
}

impl FaFnirCommand for FaFnirCommandV1 {
}


fn run_command(self, options: ArgMatches) -> Result<(), Error> {
    match options.subcommand_name() {
        Some("add") => {},
        Some("config") => {},
        Some("pull") => {},
        Some("push") => {},
        Some("link") => {},
        Some("show") => {},
        Some("init") => {},
        Some("remove") => {},
        _ => {},
    };
}
