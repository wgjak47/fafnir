use dirs::home_dir;
use failure::Error;
use git2::Repository;
use std::path::Path;
use std::path::PathBuf;

pub trait FileOperator {
    fn mv(&self, source: &Path, target: &Path) -> Result<(), Error> {
        unimplemented!()
    }
    fn link(&self, source: &Path, target: &Path) -> Result<(), Error> {
        unimplemented!()
    }
    fn rm(&self, source: &Path) -> Result<(), Error> {
        unimplemented!()
    }
}

// provider dot config storage and sync
pub trait Storage: Sized {
    // add a file to storage
    fn add(&self, filepath: &Path, tags: &Vec<String>) -> Result<(), Error>;
    // remove a file from storage
    fn remove(&self, filepath: &Path) -> Result<(), Error>;
    // get default tags for distribute
    fn get_default_tags(&self) -> Result<&Vec<String>, Error>;
    // update tags config
    fn set_default_tags(&self, tags: &Vec<String>) -> Result<(), Error>;
    // pull from remote
    fn pull(&self) -> Result<(), Error>;
    // push to remote
    fn push(&self) -> Result<(), Error>;
    // load or creat a storage
    fn load_or_new(store_path: &Path) -> Result<Self, Error>;
}

// record config UUID and
pub trait StorageManager: Sized {
    fn load() -> Result<Self, Error>;
    fn add(&self, filepath: &Path, tags: &Vec<String>) -> Result<(), Error>;
    fn remove(&self, filepath: &Path, tags: &Vec<String>) -> Result<(), Error>;
}

pub trait Test {
    fn test(&self) {}
}

pub struct Hehe {}

impl Test for Hehe {}

pub struct YamlStorageManager {
    config_file_path: Box<Path>
}

impl StorageManager for YamlStorageManager {
    fn load() -> Result<Self, Error> {
        unimplemented!()
    }
    fn add(&self, filepath: &Path, tags: &Vec<String>) -> Result<(), Error> {
        unimplemented!()
    }
    fn remove(&self, filepath: &Path, tags: &Vec<String>) -> Result<(), Error> {
        unimplemented!()
    }
}

pub struct GitStore<T: StorageManager> {
    repo: Repository,
    manager: T,
    manager2: Box<dyn Test>,
}

impl<T: StorageManager> FileOperator for GitStore<T> {}

impl<T: StorageManager> Storage for GitStore<T> {
    fn add(&self, filepath: &Path, tags: &Vec<String>) -> Result<(), Error> {
        let filename = filepath.file_name().ok_or(format_err!("invalid filepath"))?;
        let mut target = self.repo.path().parent().ok_or(format_err!("invalid repo path"))?.to_owned();
        target.push(filename);
        self.mv(filepath, &target)?;
        self.link(&target, filepath)?;
        Ok(())
    }

    fn remove(&self, filepath: &Path) -> Result<(), Error> {
        unimplemented!();
    }

    fn get_default_tags(&self) -> Result<&Vec<String>, Error> {
        unimplemented!();
    }

    fn set_default_tags(&self, tags: &Vec<String>) -> Result<(), Error> {
        Ok(())
    }

    fn pull(&self) -> Result<(), Error> {
        unimplemented!();
    }

    fn push(&self) -> Result<(), Error> {
        unimplemented!();
    }
    fn load_or_new(store_path: &Path) -> Result<GitStore<T>, Error> {
        let repo: Repository;
        if store_path.exists() {
            repo = Repository::open(store_path)?;
        } else {
            repo = Repository::init(store_path)?;
        }
        if repo.is_bare() {
            return Err(format_err!("invalid repo, should not be bare repo"));
        }

        let manager = T::load()?;

        Ok(GitStore::<T> { repo: repo, manager: manager })
    }
}

// default store path is $HOME/.fafnir
pub fn get_default_store_path() -> Result<PathBuf, Error> {
    match home_dir() {
        Some(home_path) => {
            let mut store_path = home_path.to_owned();
            store_path.push(".fafnir");
            Ok(store_path)
        }
        None => Err(format_err!("failed to get home dir path")),
    }
}
