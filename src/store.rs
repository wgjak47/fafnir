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
}

// provider dot config storage and sync
pub trait Storage {
    // add a file to storage
    fn add(&self, filepath: &Path, tags: &Vec<String>) -> Result<(), Error>;
    // remove a file from storage
    fn remove(&self, filepath: &Path) -> Result<(), Error>;
    // get default tags for distribute
    fn get_default_tags(&self) -> Result<&Vec<String>, Error>;
    // update tags config
    fn set_tags(&self, tags: &Vec<String>) -> Result<(), Error>;
    // pull from remote
    fn pull(&self) -> Result<(), Error>;
    // push to remote
    fn push(&self) -> Result<(), Error>;
}

pub struct GitStore {
    repo: Repository,
}

impl FileOperator for GitStore {}

impl Storage for GitStore {
    fn add(&self, filepath: &Path, tags: &Vec<String>) -> Result<(), Error> {
        let home = home_dir().ok_or(format_err!("failed to get home dir"))?;
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

    fn set_tags(&self, tags: &Vec<String>) -> Result<(), Error> {
        Ok(())
    }

    fn pull(&self) -> Result<(), Error> {
        unimplemented!();
    }

    fn push(&self) -> Result<(), Error> {
        unimplemented!();
    }
}

impl GitStore {
    pub fn load_or_new(store_path: &Path) -> Result<GitStore, Error> {
        let repo: Repository;
        if store_path.exists() {
            let repo = Repository::open(store_path)?;
        } else {
            let repo = Repository::init(store_path)?;
        }
        if repo.is_bare() {
            return Err(format_err!("invalid repo, should not be bare repo"));
        }
        Ok(GitStore { repo: repo })
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
