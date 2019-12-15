use clap::ArgMatches;
use failure::Error;

pub struct FaFnirCommandV1 {
    pub tags: Vec<String>,
    pub add: add_command,
    pub config: config_command,
    pub pull: pull_command,
    pub push: pull_command,
    pub distribute: distribute_command,
}


pub trait add_command {
    fn run();
}

pub trait config_command {
    fn set();
}

pub trait pull_command {
    fn run();
}

pub trait push_command {
    fn run();
}

pub trait distribute_command {
    fn run();
}

pub trait fafnir_command {
    fn run_command(self, options: ArgMatches) -> Result<(), Error>;
}

impl fafnir_command for FaFnirCommandV1 {
    fn run_command(self, options: ArgMatches) -> Result<(), Error> {
        match options.subcommand_name() {
            Some("add") => {},
            Some("config") => {},
            Some("pull") => {},
            Some("push") => {},
            Some("distribute") => {},
            _ => {},
        };
    }
}
