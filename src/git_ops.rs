// Git operations module - wraps git2 operations

use anyhow::{Context, Result};
use git2::{Repository, Signature, IndexAddOption};
use std::path::Path;
use crate::error::VaultError;

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    /// Initialize a new Git repository
    pub fn init(path: &Path) -> Result<Self> {
        let repo = Repository::init(path)
            .context("Failed to initialize Git repository")?;

        Ok(Self { repo })
    }

    /// Open an existing Git repository
    pub fn open(path: &Path) -> Result<Self> {
        let repo = Repository::open(path)
            .context("Failed to open Git repository")?;

        Ok(Self { repo })
    }

    /// Clone a remote repository
    pub fn clone(url: &str, path: &Path) -> Result<Self> {
        let repo = Repository::clone(url, path)
            .context("Failed to clone repository")?;

        Ok(Self { repo })
    }

    /// Check if the repository is empty (no commits)
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.repo.is_empty()?)
    }

    /// Add a remote to the repository
    pub fn add_remote(&self, name: &str, url: &str) -> Result<()> {
        self.repo.remote(name, url)
            .context("Failed to add remote")?;
        Ok(())
    }

    /// Get the URL of a remote
    pub fn get_remote_url(&self, name: &str) -> Result<Option<String>> {
        let remote = self.repo.find_remote(name);
        match remote {
            Ok(remote) => Ok(remote.url().map(|s| s.to_string())),
            Err(_) => Ok(None),
        }
    }

    /// Check if repository has uncommitted changes
    pub fn has_changes(&self) -> Result<bool> {
        let statuses = self.repo.statuses(None)
            .context("Failed to get repository status")?;

        Ok(!statuses.is_empty())
    }

    /// Add all changes to staging
    pub fn add_all(&self) -> Result<()> {
        let mut index = self.repo.index()
            .context("Failed to get repository index")?;

        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
            .context("Failed to add files to index")?;

        index.write()
            .context("Failed to write index")?;

        Ok(())
    }

    /// Commit staged changes
    pub fn commit(&self, message: &str) -> Result<()> {
        let mut index = self.repo.index()
            .context("Failed to get repository index")?;

        let tree_id = index.write_tree()
            .context("Failed to write tree")?;

        let tree = self.repo.find_tree(tree_id)
            .context("Failed to find tree")?;

        let signature = Signature::now("gfv", "gfv@local")
            .context("Failed to create signature")?;

        if self.is_empty()? {
            // Initial commit - no parent
            self.repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &[],
            ).context("Failed to create initial commit")?;
        } else {
            // Normal commit with parent
            let parent_commit = self.repo.head()?.peel_to_commit()?;
            self.repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &[&parent_commit],
            ).context("Failed to create commit")?;
        }

        Ok(())
    }

    /// Pull changes from remote
    pub fn pull(&self, remote_name: &str, branch: &str) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)
            .context("Failed to find remote")?;

        // Fetch
        remote.fetch(&[branch], None, None)
            .context("Failed to fetch from remote")?;

        // Get the fetch head
        let fetch_head = self.repo.find_reference("FETCH_HEAD")
            .context("Failed to find FETCH_HEAD")?;

        let fetch_commit = self.repo.reference_to_annotated_commit(&fetch_head)
            .context("Failed to get fetch commit")?;

        // Perform merge analysis
        let (analysis, _) = self.repo.merge_analysis(&[&fetch_commit])
            .context("Failed to analyze merge")?;

        if analysis.is_up_to_date() {
            return Ok(());
        }

        if analysis.is_fast_forward() {
            // Fast-forward merge
            let refname = format!("refs/heads/{}", branch);
            let mut reference = self.repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            self.repo.set_head(&refname)?;
            self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else {
            // Need to merge - for now, just report conflict
            return Err(VaultError::GitConflict.into());
        }

        Ok(())
    }

    /// Push changes to remote
    pub fn push(&self, remote_name: &str, branch: &str) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)
            .context("Failed to find remote")?;

        let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);

        remote.push(&[&refspec], None)
            .context("Failed to push to remote")?;

        Ok(())
    }
}
