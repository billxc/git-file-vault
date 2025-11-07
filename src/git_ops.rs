// Git operations module - wraps git2 operations

use anyhow::{bail, Context, Result};
use git2::{Repository, Signature, IndexAddOption, Cred, RemoteCallbacks};
use std::path::Path;
use crate::error::VaultError;

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    /// Create callbacks for Git authentication
    /// Supports both SSH keys and Git credential manager (for HTTPS)
    fn create_auth_callbacks<'a>() -> RemoteCallbacks<'a> {
        let mut callbacks = RemoteCallbacks::new();

        callbacks.credentials(|url, username_from_url, allowed_types| {
            // Try different credential methods based on URL type and allowed types

            // 1. Try credential helper first (works for HTTPS with credential manager)
            if allowed_types.contains(git2::CredentialType::USERNAME | git2::CredentialType::USER_PASS_PLAINTEXT)
                || allowed_types.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
                if let Ok(cred) = Cred::credential_helper(&git2::Config::open_default().unwrap(), url, username_from_url) {
                    return Ok(cred);
                }
            }

            // 2. Try SSH agent for SSH URLs
            if allowed_types.contains(git2::CredentialType::SSH_KEY) {
                if let Ok(cred) = Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                    return Ok(cred);
                }
            }

            // 3. Try default SSH key
            if allowed_types.contains(git2::CredentialType::SSH_KEY) {
                if let Ok(cred) = Cred::ssh_key(
                    username_from_url.unwrap_or("git"),
                    None,
                    std::path::Path::new(&format!("{}/.ssh/id_rsa",
                        dirs::home_dir().unwrap().display())),
                    None,
                ) {
                    return Ok(cred);
                }
            }

            // If all else fails, return an error
            Err(git2::Error::from_str("No authentication method available"))
        });

        callbacks
    }

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
        // Set up authentication callbacks
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(Self::create_auth_callbacks());

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        let repo = builder.clone(url, path)
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

    /// Set or update a remote URL
    pub fn set_remote(&self, name: &str, url: &str) -> Result<()> {
        // Check if remote exists
        if self.repo.find_remote(name).is_ok() {
            // Update existing remote
            self.repo.remote_set_url(name, url)
                .context("Failed to update remote URL")?;
        } else {
            // Add new remote
            self.repo.remote(name, url)
                .context("Failed to add remote")?;
        }
        Ok(())
    }

    /// Check if repository has uncommitted changes
    pub fn has_changes(&self) -> Result<bool> {
        let statuses = self.repo.statuses(None)
            .context("Failed to get repository status")?;

        Ok(!statuses.is_empty())
    }

    /// Check if a remote branch exists
    pub fn remote_branch_exists(&self, remote_name: &str, branch: &str) -> bool {
        let refname = format!("refs/remotes/{}/{}", remote_name, branch);
        self.repo.find_reference(&refname).is_ok()
    }

    /// Set the current branch name (useful when initializing with a specific branch)
    pub fn set_branch(&self, branch: &str) -> Result<()> {
        let head = self.repo.head()?;
        let oid = head.target().context("HEAD has no target")?;

        let refname = format!("refs/heads/{}", branch);
        self.repo.reference(&refname, oid, true, "Set branch name")?;
        self.repo.set_head(&refname)?;

        Ok(())
    }

    /// Get the current branch name
    pub fn current_branch(&self) -> Result<String> {
        let head = self.repo.head()
            .context("Failed to get HEAD")?;

        if let Some(name) = head.shorthand() {
            Ok(name.to_string())
        } else {
            bail!("HEAD is not pointing to a branch")
        }
    }

    /// Check if local branch is ahead of remote (has unpushed commits)
    pub fn has_unpushed_commits(&self, remote_name: &str, branch: &str) -> Result<bool> {
        let local_refname = format!("refs/heads/{}", branch);
        let remote_refname = format!("refs/remotes/{}/{}", remote_name, branch);

        // Get local branch reference
        let local_ref = self.repo.find_reference(&local_refname)
            .context("Failed to find local branch")?;
        let local_oid = local_ref.target().context("Local branch has no target")?;

        // Get remote tracking branch reference
        let remote_ref = match self.repo.find_reference(&remote_refname) {
            Ok(r) => r,
            Err(_) => return Ok(true), // Remote branch doesn't exist, so we're ahead
        };
        let remote_oid = remote_ref.target().context("Remote branch has no target")?;

        // If OIDs are different, check if local is ahead
        if local_oid != remote_oid {
            // Check if local contains remote (local is ahead)
            match self.repo.graph_descendant_of(local_oid, remote_oid) {
                Ok(is_descendant) => Ok(is_descendant),
                Err(_) => Ok(true), // If we can't determine, assume we're ahead
            }
        } else {
            Ok(false) // Same commit, not ahead
        }
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

    /// Fetch from remote (without merging)
    pub fn fetch(&self, remote_name: &str, branch: &str) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)
            .context("Failed to find remote")?;

        // Set up authentication callbacks
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(Self::create_auth_callbacks());

        // Fetch from remote
        let refspec = format!("refs/heads/{}", branch);
        remote.fetch(&[&refspec], Some(&mut fetch_options), None)
            .context("Failed to fetch from remote")?;

        Ok(())
    }

    /// Pull changes from remote
    pub fn pull(&self, remote_name: &str, branch: &str) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)
            .context("Failed to find remote")?;

        // Set up authentication callbacks
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(Self::create_auth_callbacks());

        // Fetch from remote
        let refspec = format!("refs/heads/{}", branch);
        remote.fetch(&[&refspec], Some(&mut fetch_options), None)
            .context("Failed to fetch from remote")?;

        // Get the remote branch reference
        let fetch_refname = format!("refs/remotes/{}/{}", remote_name, branch);
        let fetch_ref = self.repo.find_reference(&fetch_refname)
            .context("Failed to find remote branch after fetch")?;

        let fetch_commit = self.repo.reference_to_annotated_commit(&fetch_ref)
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

        // Set up authentication callbacks
        let mut push_options = git2::PushOptions::new();
        push_options.remote_callbacks(Self::create_auth_callbacks());

        remote.push(&[&refspec], Some(&mut push_options))
            .context("Failed to push to remote")?;

        Ok(())
    }
}
