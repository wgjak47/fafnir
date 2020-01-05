use git2::Repository;
use std::path::Path;
use std::path::PathBuf;
use failure::Error;

pub trait Store {
    fn create_repo(path: &Path) -> Self;
    fn load_repo(path: &Path) -> Self;
    fn add(&self, filepath: &Path) -> Result<(), Error>;
    fn remove(&self, filepath: &Path) -> Result<(), Error>;
}

pub struct GitStore {
    repo_path: PathBuf,
    repo: Repository,
}

impl Store for GitStore {
    fn create_repo(path: &Path) -> Self {
        unimplemented!();
    }
    fn load_repo(path: &Path) -> Self {
        unimplemented!();
    }
    fn add(&self, filepath: &Path) -> Result<(), Error> {
        unimplemented!();
    }
    fn remove(&self, filepath: &Path) -> Result<(), Error> {
        unimplemented!();
    }
}
