use dirs::home_dir;
use failure::Error;
use fs_extra::{dir, move_items, remove_items};
use git2::{Commit, Direction, ObjectType, Remote, Repository, Signature};
use git2_credentials::CredentialHandler;
use std::os::unix::fs;
use std::path::Path;
use std::path::PathBuf;

pub trait FileOperator {
    fn mv(&self, source: &Path, target: &Path) -> Result<(), Error> {
        let options = dir::CopyOptions::new();
        let mut from_paths = Vec::new();
        from_paths.push(source);
        move_items(&from_paths, target, &options)?;
        Ok(())
    }

    // not support windows for now
    fn link(&self, source: &Path, target: &Path) -> Result<(), Error> {
        fs::symlink(source, target)?;
        Ok(())
    }

    fn rm(&self, source: &Path) -> Result<(), Error> {
        let mut from_paths = Vec::new();
        from_paths.push(source);
        remove_items(&from_paths)?;
        Ok(())
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
    // set remote
    fn set_remote(&self, remote_url: &str) -> Result<(), Error>;
}

// record config UUID and
pub trait StorageManager: Sized {
    fn load() -> Result<Self, Error>;
    fn add(&self, filepath: &Path, tags: &Vec<String>) -> Result<(), Error>;
    fn remove(&self, filepath: &Path, tags: &Vec<String>) -> Result<(), Error>;
}

pub struct YamlStorageManager {
    config_file_path: Box<Path>,
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
}

impl<T: StorageManager> GitStore<T> {
    // get commit signature from git config
    fn get_signature(&self) -> Result<Signature, Error> {
        let config = self.repo.config()?;
        let username = config.get_str("user.name")?;
        let email = config.get_str("user.email")?;
        let signature = Signature::now(username, email)?;
        Ok(signature)
    }

    // find the last commit
    fn find_last_commit(&self) -> Result<Option<Commit>, Error> {
        let obj = self.repo.head()?.resolve()?.peel(ObjectType::Commit)?;
        let commit = obj.into_commit().ok();
        Ok(commit)
    }

    // commit on current tree
    fn save(&self) -> Result<(), Error> {
        let mut index = self.repo.index()?;
        // TODO check the real path
        let path = self
            .repo
            .path()
            .parent()
            .ok_or(format_err!("invalid git path"))?;
        index.add_path(path)?;
        let oid = index.write_tree()?;
        let commit = self.find_last_commit()?;
        let tree = self.repo.find_tree(oid)?;
        let mut parents = Vec::<&Commit>::new();

        if let Some(parent) = &commit {
            parents.push(parent)
        }

        let signature = self.get_signature()?;
        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "update dot file",
            &tree,
            &parents,
        )?;
        Ok(())
    }

    // prepare remote, set ssh auth to fetch or push
    fn prepare_remote(&self) -> Result<Remote, Error> {
        let remote = self.repo.find_remote("origin")?;
        let mut cb = git2::RemoteCallbacks::new();
        let git_config = self.repo.config()?;
        let mut ch = CredentialHandler::new(git_config);
        cb.credentials(move |url, username, allowed| {
            ch.try_next_credential(url, username, allowed)
        });
        Ok(remote)
    }

    fn fast_forward(
        &self,
        lb: &mut git2::Reference,
        rc: &git2::AnnotatedCommit,
    ) -> Result<(), git2::Error> {
        let name = match lb.name() {
            Some(s) => s.to_string(),
            None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
        };
        let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
        println!("{}", msg);
        lb.set_target(rc.id(), &msg)?;
        self.repo.set_head(&name)?;
        self.repo.checkout_head(Some(
            git2::build::CheckoutBuilder::default()
                // For some reason the force is required to make the working directory actually get updated
                // I suspect we should be adding some logic to handle dirty working directory states
                // but this is just an example so maybe not.
                .force(),
        ))?;
        Ok(())
    }

    fn normal_merge(
        &self,
        local: &git2::AnnotatedCommit,
        remote: &git2::AnnotatedCommit,
    ) -> Result<(), git2::Error> {
        let local_tree = self.repo.find_commit(local.id())?.tree()?;
        let remote_tree = self.repo.find_commit(remote.id())?.tree()?;
        let ancestor = self
            .repo
            .find_commit(self.repo.merge_base(local.id(), remote.id())?)?
            .tree()?;
        let mut idx = self
            .repo
            .merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

        if idx.has_conflicts() {
            println!("Merge conficts detected...");
            self.repo.checkout_index(Some(&mut idx), None)?;
            return Ok(());
        }
        let result_tree = self.repo.find_tree(idx.write_tree_to(&self.repo)?)?;
        // now create the merge commit
        let msg = format!("Merge: {} into {}", remote.id(), local.id());
        let sig = self.repo.signature()?;
        let local_commit = self.repo.find_commit(local.id())?;
        let remote_commit = self.repo.find_commit(remote.id())?;
        // Do our merge commit and set current branch head to that commit.
        let _merge_commit = self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &msg,
            &result_tree,
            &[&local_commit, &remote_commit],
        )?;
        // Set working tree to match head.
        self.repo.checkout_head(None)?;
        Ok(())
    }

    // modify from https://github.com/rust-lang/git2-rs/blob/master/examples/pull.rs
    fn do_merge(
        &self,
        remote_branch: &str,
        fetch_commit: git2::AnnotatedCommit,
    ) -> Result<(), Error> {
        // 1. do a merge analysis
        let analysis = self.repo.merge_analysis(&[&fetch_commit])?;

        // 2. Do the appopriate merge
        if analysis.0.is_fast_forward() {
            println!("Doing a fast forward");
            // do a fast forward
            let refname = format!("refs/heads/{}", remote_branch);
            match self.repo.find_reference(&refname) {
                Ok(mut r) => {
                    self.fast_forward(&mut r, &fetch_commit)?;
                }
                Err(_) => {
                    // The branch doesn't exist so just set the reference to the
                    // commit directly. Usually this is because you are pulling
                    // into an empty repository.
                    self.repo.reference(
                        &refname,
                        fetch_commit.id(),
                        true,
                        &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
                    )?;
                    self.repo.set_head(&refname)?;
                    self.repo.checkout_head(Some(
                        git2::build::CheckoutBuilder::default()
                            .allow_conflicts(true)
                            .conflict_style_merge(true)
                            .force(),
                    ))?;
                }
            };
        } else if analysis.0.is_normal() {
            // do a normal merge
            let head_commit = self
                .repo
                .reference_to_annotated_commit(&self.repo.head()?)?;
            self.normal_merge(&head_commit, &fetch_commit)?;
        } else {
            println!("Nothing to do...");
        }
        Ok(())
    }
}

impl<T: StorageManager> FileOperator for GitStore<T> {}

impl<T: StorageManager> Storage for GitStore<T> {
    fn add(&self, filepath: &Path, tags: &Vec<String>) -> Result<(), Error> {
        let filename = filepath
            .file_name()
            .ok_or(format_err!("invalid filepath"))?;
        let mut target = self
            .repo
            .path()
            .parent()
            .ok_or(format_err!("invalid repo path"))?
            .to_owned();
        target.push(filename);
        self.mv(filepath, &target)?;
        self.link(&target, filepath)?;
        self.save()?;
        Ok(())
    }

    fn set_remote(&self, remote_url: &str) -> Result<(), Error> {
        self.repo.remote("origin", remote_url)?;
        Ok(())
    }

    fn remove(&self, filepath: &Path) -> Result<(), Error> {
        unimplemented!();
    }

    fn get_default_tags(&self) -> Result<&Vec<String>, Error> {
        unimplemented!();
    }

    fn set_default_tags(&self, tags: &Vec<String>) -> Result<(), Error> {
        unimplemented!()
    }

    fn pull(&self) -> Result<(), Error> {
        let mut remote = self.prepare_remote()?;
        remote.connect(Direction::Fetch)?;
        remote.fetch(&["master"], None, None)?;
        let fetch_head = self
            .repo
            .reference_to_annotated_commit(&self.repo.find_reference("FETCH_HEAD")?)?;
        self.do_merge("master", fetch_head)
    }

    fn push(&self) -> Result<(), Error> {
        let mut remote = self.prepare_remote()?;
        remote.push(&["master"], None)?;
        remote.connect(Direction::Push)?;
        Ok(())
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

        Ok(GitStore::<T> {
            repo: repo,
            manager: manager,
        })
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
