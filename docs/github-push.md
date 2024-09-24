# Documentation for Creating a GitHub Repository and Pushing Code

This document explains the process of creating a GitHub repository and pushing generated code into the repository via REST API calls within your application.

## Input Data

### JSON Format
Input data is sent in JSON format and contains the following parameters:

- **name**: Project name (string) – this will be the name of the project to be generated and later pushed to GitHub.
- **pallets**: List of supported pallets (string array) – these modules will be included in the project.
- **push_to_git**: Boolean value (true/false) – defines whether the code should be pushed to GitHub.
- **github_token**: GitHub Personal Access Token (string) – token that allows authentication with the GitHub API and pushes the repository.

### Example Call
A call using the `curl` command looks like this:

```bash
curl -X POST http://127.0.0.1:8080/generate-project \
-H "Content-Type: application/json" \
-d '{
  "name": "my_project",
  "pallets": ["PalletUtility", "PalletIdentity"],
  "push_to_git": true,
  "github_token": "YOUR_GITHUB_TOKEN"
}'

```
## Project Generation and Pushing to GitHub

1. **Project Generation**  
   When you call the API using the curl request shown above, your application automatically generates a project based on the name and specified modules.

   - **Function**: `generate_a_project()`  
     The application creates the project structure with all necessary files and modules (pallets) in the `generated_code` directory.

2. **GitHub Repository Creation**  
   If `push_to_git` is set to `true`, the application creates a new GitHub repository using the GitHub API and GitHub token for authentication. The repository is created based on the parameters passed in the JSON format.

3. **Pushing to GitHub**  
   After the project is successfully generated and the repository is created, the pushing process involves the following steps:
   
   - **Git Initialization**: Verifies if the project directory is ready. `git init`.
   - **Adding files**: All files from the generated directory are added to the Git index using `git add`.
   - **Committing changes**: After adding the files, the `git commit` command is executed to commit the changes to the local Git repository.
   - **Setting the branch**: A default branch (usually `main`) is created or switched to, if it does not already exist.
   - **Adding remote URL**: A remote repository is created on GitHub using the authentication token, and the remote is linked to the local Git repository.
   - **Pushing the code**: Finally, the changes are pushed to the remote GitHub repository using `git push`.
