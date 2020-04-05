use crate::store::{GitStore, Storage, YamlStorageManager};
use clap::ArgMatches;
use dirs::home_dir;
use failure::Error;
use shellexpand::full as shell_expand;
use std::path::Path;
use std::path::PathBuf;

pub trait FaFnirCore {
    fn init(&self, url: &String) -> Result<(), Error>;
    fn show(&self) -> Result<(), Error>;
    fn add(&self, file_path: &Path, tags: &Vec<String>) -> Result<(), Error>;
    fn remove(&self, file_path: &Path) -> Result<(), Error>;
    fn config(&self, url: &str, tags: &Vec<String>) -> Result<(), Error>;
    fn pull(&self) -> Result<(), Error>;
    fn push(&self) -> Result<(), Error>;
    fn link(&self, file_path: &Path, tags: &Vec<String>) -> Result<(), Error>;
}

pub struct FaFnirCoreV1<T: Storage> {
    pub store: T,
    pub tags: Vec<String>,
}

impl<T: Storage> FaFnirCoreV1<T> {
    // TODO load from git repo ~/.fafnir or ~/.config/fafnir
    pub fn load() -> Result<FaFnirCoreV1<T>, Error> {
        let mut home = home_dir().ok_or(format_err!("failed to get home dir"))?;
        home.push(".fafnir");
        let store = T::load_or_new(&home)?;
        Ok(FaFnirCoreV1::<T> {
            tags: vec!["emacs", "linux"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            store: store,
        })
    }
}

impl<T: Storage> FaFnirCore for FaFnirCoreV1<T> {
    fn init(&self, url: &String) -> Result<(), Error> {
        unimplemented!()
    }

    fn show(&self) -> Result<(), Error> {
        unimplemented!()
    }

    fn add(&self, file_path: &Path, tags: &Vec<String>) -> Result<(), Error> {
        unimplemented!()
    }

    fn remove(&self, file_path: &Path) -> Result<(), Error> {
        unimplemented!()
    }

    fn config(&self, url: &str, tags: &Vec<String>) -> Result<(), Error> {
        unimplemented!()
    }

    fn pull(&self) -> Result<(), Error> {
        unimplemented!()
    }

    fn push(&self) -> Result<(), Error> {
        unimplemented!()
    }

    fn link(&self, file_path: &Path, tags: &Vec<String>) -> Result<(), Error> {
        unimplemented!()
    }
}

fn get_file_path(raw_path: Option<&str>) -> Result<PathBuf, Error> {
    raw_path
        .ok_or(format_err!("invalid path!"))
        .and_then(|path: &str| -> Result<PathBuf, Error> {
            let _path = shell_expand(path)?;
            Ok(PathBuf::from(&_path.into_owned()))
        })
}

pub fn run_command(options: ArgMatches) -> Result<(), Error> {
    let command = FaFnirCoreV1::<GitStore<YamlStorageManager>>::load()?;

    match options.subcommand() {
        ("add", Some(sub_m)) => {
            let raw_path = sub_m.value_of("filepath");
            let file_path = get_file_path(raw_path)?;
            let tags_str = sub_m.value_of("tags").unwrap_or_default();
            let tags: Vec<String> = tags_str.split(",").map(|s| s.to_string()).collect();
            command.add(&file_path, &tags)
        }
        ("config", Some(sub_m)) => unimplemented!(),
        ("pull", Some(sub_m)) => unimplemented!(),
        ("push", Some(sub_m)) => unimplemented!(),
        ("link", Some(sub_m)) => unimplemented!(),
        ("show", Some(sub_m)) => unimplemented!(),
        ("init", Some(sub_m)) => unimplemented!(),
        ("remove", Some(sub_m)) => unimplemented!(),
        _ => unimplemented!(),
    }
}
