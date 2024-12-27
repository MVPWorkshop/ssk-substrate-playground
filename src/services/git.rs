use std::path::Path;

use async_trait::async_trait;
use log::info;
use reqwest::Client;
use serde_json::json;
use tokio::process::Command;

use super::version_control::{VersionControlError, VersionControlService};

pub struct GitService;

#[async_trait]
impl VersionControlService for GitService {
    async fn create_remote_repo(
        &self,
        github_username: &str,
        github_token: &str,
        repo_name: &str,
    ) -> Result<(), VersionControlError> {
        let client = Client::new();
        let url = "https://api.github.com/user/repos".to_string();

        let body = json!({
            "name": repo_name,
            "private": false,
        });

        // Send the POST request to the GitHub API to create the repository
        let response = client
            .post(&url)
            .basic_auth(github_username, Some(github_token))
            .header("User-Agent", github_username)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                VersionControlError::RemoteError(format!(
                    "Failed to send request to create GitHub repository '{}': {}",
                    repo_name, e
                ))
            })?;

        if !response.status().is_success() {
            let error_message = response.text().await.map_err(|e| {
                VersionControlError::RemoteError(format!(
                    "Failed to read response when creating GitHub repository '{}': {}",
                    repo_name, e
                ))
            })?;
            return Err(VersionControlError::FileError(format!(
                "Failed to create GitHub repository: {}",
                error_message
            )));
        }
        Ok(())
    }
    async fn push_folder_to_repo(
        &self,
        folder: &Path,
        repo_name: &str,
        username: &str,
        token: &str,
        email: &str,
    ) -> Result<(), VersionControlError> {
        // Check if the project directory exists
        if !folder.exists() {
            return Err(VersionControlError::FileError(format!(
                "Dolder {} not found",
                folder.display()
            )));
        }

        // Initialize Git repository in the project directory
        let init_output = Command::new("git")
            .arg("init")
            .current_dir(folder)
            .output()
            .await
            .map_err(|e| {
                VersionControlError::RepoError(format!(
                    "Failed to initialize Git repository in '{}': {:?}",
                    folder.display(),
                    e
                ))
            })?;

        // Check if Git initialization was successful
        if !init_output.status.success() {
            VersionControlError::RepoError(format!(
                "Git initialization failed for directory '{}'.",
                folder.display()
            ));
        }

        info!(
            "Git repository successfully initialized in '{}'.",
            folder.display(),
        );

        // Set Git user email and name
        let config_email_output = Command::new("git")
            .arg("config")
            .arg("user.email")
            .arg(email)
            .current_dir(folder)
            .output()
            .await
            .map_err(|e| {
                VersionControlError::RepoError(format!(
                    "Failed to set Git user email in '{}': {:?}",
                    folder.display(),
                    e
                ))
            })?;

        if !config_email_output.status.success() {
            return Err(VersionControlError::RepoError(format!(
                "Git 'config user.email' failed in directory '{}'. Output: {:?}",
                folder.display(),
                String::from_utf8_lossy(&config_email_output.stderr),
            )));
        }

        let config_name_output = Command::new("git")
            .arg("config")
            .arg("user.name")
            .arg(username)
            .current_dir(folder)
            .output()
            .await
            .map_err(|e| {
                VersionControlError::RepoError(format!(
                    "Failed to set Git user name in '{}': {:?}",
                    folder.display(),
                    e
                ))
            })?;

        if !config_name_output.status.success() {
            return Err(VersionControlError::RepoError(format!(
                "Git 'config user.name' failed in directory '{}'. Output: {:?}",
                folder.display(),
                String::from_utf8_lossy(&config_name_output.stderr)
            )));
        }

        // Add all files in the project directory to the Git repository
        let add_output = Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(folder)
            .output()
            .await
            .map_err(|e| {
                VersionControlError::RepoError(format!(
                    "Error adding files to Git repository in directory '{}': {:?}",
                    folder.display(),
                    e
                ))
            })?;

        if !add_output.status.success() {
            return Err(VersionControlError::RepoError(format!(
                "Git 'add' command failed in directory '{}'. Output: {:?}",
                folder.display(),
                String::from_utf8_lossy(&add_output.stderr)
            )));
        }

        info!(
            "Files successfully added to the Git repository in directory '{}'.",
            folder.display()
        );

        // Commit the changes in the repository with a message
        let commit_output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("Initial commit of generated project")
            .current_dir(folder)
            .output()
            .await
            .map_err(|e| {
                VersionControlError::RepoError(format!(
                    "Error commiting files to Git repository in directory '{}': {:?}",
                    folder.display(),
                    e
                ))
            })?;

        // Check if the `git commit` command was successful
        if !commit_output.status.success() {
            return Err(VersionControlError::RepoError(format!(
                "Git 'commit' command failed in directory '{}'. Output: {:?}",
                folder.display(),
                String::from_utf8_lossy(&commit_output.stderr)
            )));
        }

        info!(
            "Changes successfully committed in directory '{}'.",
            folder.display()
        );

        // Prepare the remote URL for pushing to GitHub

        let remote_url = format!(
            "https://{}:{}@github.com/{}/{}.git",
            username, token, username, repo_name,
        );

        // Add the remote origin for the GitHub repository
        let remote_output = Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(&remote_url)
            .current_dir(folder)
            .output()
            .await
            .map_err(|e| {
                VersionControlError::RepoError(format!(
                    "Failed to add remote origin in directory '{}': {:?}",
                    folder.display(),
                    e
                ))
            })?;

        if !remote_output.status.success() {
            return Err(VersionControlError::RepoError(format!(
                "Git 'remote add' command failed in directory '{}'. Output: {:?}",
                folder.display(),
                String::from_utf8_lossy(&commit_output.stderr)
            )));
        }

        info!(
            "Remote origin successfully added in directory '{}'.",
            folder.display()
        );

        // Set the branch to 'main'
        let branch_output = Command::new("git")
            .arg("branch")
            .arg("-M")
            .arg("main")
            .current_dir(folder)
            .output()
            .await
            .map_err(|e| {
                VersionControlError::RepoError(format!(
                    "Failed to set branch to 'main' in directory '{}': {:?}",
                    folder.display(),
                    e
                ))
            })?;

        if !branch_output.status.success() {
            return Err(VersionControlError::RepoError(format!(
                "Git 'branch -M main' command failed in directory '{}'. Output: {:?}",
                folder.display(),
                String::from_utf8_lossy(&branch_output.stderr)
            )));
        }

        info!(
            "Branch successfully set to 'main' in directory '{}'.",
            folder.display()
        );

        // Push the changes to the GitHub repository using the provided credentials
        let push_output = Command::new("git")
            .arg("push")
            .arg("-u")
            .arg("origin")
            .arg("main")
            .current_dir(folder)
            .output()
            .await
            .map_err(|e| {
                VersionControlError::RepoError(format!(
                    "Failed to push changes to GitHub from directory '{}': {:?}",
                    folder.display(),
                    e
                ))
            })?;

        if !push_output.status.success() {
            return Err(VersionControlError::RepoError(format!(
                "Git 'push' command failed for directory '{}'. Output: {:?}",
                folder.display(),
                String::from_utf8_lossy(&push_output.stderr)
            )));
        }

        info!(
            "Project '{}' successfully pushed to GitHub from directory '{}'.",
            repo_name,
            folder.display()
        );

        Ok(())
    }
}
