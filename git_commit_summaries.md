# Git Commit Summaries

Current commit: ae8a4dcc0a5b229d3524a356b64b80385631a4ea

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 12

<details>
<summary>summary for commit 1 (ae8a4dcc0a5b229d3524a356b64b80385631a4ea)</summary>

The commit brings substantial enhancements to the Git Commit Summarizer tool. Here are the key changes:

1. Tweaked the run_git_command function for better command handling. The function now takes the entire git command as arguments rather than just the first argument.
2. Enhanced the summarize_changes function to accept system prompt, user prompt, and max tokens as parameters. This allows for more customizable summaries.
3. Added a print_help function that provides comprehensive usage instructions for the tool.
4. Introduced a get_current_commit function to fetch the hash of the current commit in a given repo path.
5. Updated the main function to support more flexible user input. Users can now choose the type of summary ('all' or 'individual') and the style of the summary ('summary', 'technical', or 'blog').
6. The code now generates summaries for all commits together when 'all' is chosen, or for each commit individually when 'individual' is chosen. For each commit or set of commits, it
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (5d196e3d07d4addd4423230771407120032b5c1a)</summary>

The commit adds the dotenv package to the project and updates the README file. 

The dotenv package, version 0.15.0, was added to the Cargo.toml file. This package is used for managing environment variables and the dotenv() function was introduced in the main.rs file. This implies that the project now utilizes environment variables, potentially for configuration purposes.

The README file was updated with a new command that allows users to retrieve the last n commits from a git log. This enhances the tool's functionality by offering more flexible usage options.

The changes in this commit may impact how the app is configured and how it retrieves git commits.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (985b778ad5c6e60184328303ff4a84e42d83e2fa)</summary>

The commit by matthew-heartful contains minor but notable changes to the README.md file:

1. Improvements in wording and readability: The language used to describe the challenges of reading through team commits and the solution provided has been adjusted to improve clarity. 

2. Clarification in instructions: The instructions for building the project using 'cargo build --release' have been simplified.

3. More accurate description of the tool: The tool's functionality was clarified. It now explicitly states that it helps understand changes made in the most recent commit of the project which are ahead of your local directory. 

These changes enhance the readability and understanding of the README.md file, thereby making it easier for users to understand the project and its usage.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (441e6eeccaf9df6416a94dcab8c80a8b7ac0971c)</summary>

The commit introduces a new environment sample file `.env.sample`. This file includes a placeholder for the `OPENAI_API_KEY` which is necessary for accessing OpenAI services. This commit does not directly impact the project functionality but provides guidance for setting up the required environment variable.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (da8095642965554b7e3e5ce2993275a00f5257bb)</summary>

The commit significantly updated the README.md file for the project 'summarize_recent_commit'. 

Key modifications include:

1. Added a detailed project description, explaining its purpose and why it was created. The author expressed frustration with reading multiple commit messages and this tool aims to solve that issue by summarizing all commits into a single markdown text file.

2. Added a 'Next Steps' section, seeking user feedback and suggesting the possibility of packaging the tool into an npm/brew library for easier use.

3. Included a 'Give it a Star' section encouraging users to star the project on GitHub if they find it useful, and provided a link to do so.

4. Provided contact information for the author, including an email address and Discord handle.

5. Detailed instructions on how to 'Get Started' with the tool, from installing Rust, cloning the repo, setting up the OPENAI API KEY in .env, to building and running the tool.

6. Added an 'Overview' and '
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (ad871d30e76726cff7f0b182a130095d347ef889)</summary>

The commit `ad871d30e76726cff7f0b182a130095d347ef889` by `matthew-heartful` involves updates to the `.gitignore` file. Two new files/directories have been added to the list of files to be ignored by Git: `target/` and `Cargo.lock`. This means any changes to these files will not be tracked. Also, the issue with no newline at the end of the file has been fixed.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (2e13e61e6640c1817b79a8bec8bc12ce0b1d58a7)</summary>

The commit from author Matthew Heartful introduces a new .gitignore file to the project. In this file, ".env" is specified, which means that the Git version control system will ignore the ".env" file. This is typically done to exclude certain environment-specific files that should not be shared publicly, like configuration files containing sensitive data. Note that there is no newline at the end of the file.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (eb3dfb82c159f12bb3eb4f0da1ecde1409430d2e)</summary>

The commit 'logic' by matthew-heartful involves a significant change in how the API key for the OpenAI service is retrieved in the 'summarize_changes' function in main.rs. Previously, the API key was a hardcoded string value "OPENAI_API_KEY". Now, it is retrieved from an environment variable "OPENAI_API_KEY" using the 'env::var' method. If the environment variable is not found, the system will throw an error message "OPENAI_API_KEY not found in .env file". This modification enhances security by not exposing the API key directly in the code and also makes the application more flexible.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 9 (3c373f4372798b4e2adb727c8b87abd1789cfe6b)</summary>

This document contains a summary of 22 git commits, mainly focused on the `screenpipe-audio` and `screenpipe-server` functionalities of the project. Key changes include:

1. The addition of a new benchmark target named `vision_benchmark` in the `Cargo.toml` file and the refactoring of the benchmark for `continuous_capture`.
2. Implementation of a new indexing feature, updates in the `CONTRIBUTING.md` file, and the addition of instructions for running `cargo bench` and creating new migrations with `sqlx-cli`.
3. Integration of Tokio in the `screenpipe-vision` module, the addition of a feature to skip frames during OCR if the CPU is overloaded, and the ability to turn devices on or off via the API in `screenpipe-audio`.
4. A bug fix to ensure that the software listens to all audio devices and a warning log for Linux and Windows users.
5. Enhancements to ensure the system listens to all
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 10 (d02a090938f548ddb1a24e706d70b9fe65cb51ba)</summary>

The primary changes in this commit are focused on the core logic of the project. The author, matthew-heartful, added a new file `src/main.rs` containing the main functions of the project which are:

1. `run_git_command`: This function takes in a repository path and a git command, executes the command, and provides the output or error message.

2. `summarize_changes`: This asynchronous function leverages the OpenAI API to summarize git changes. It sends a POST request to the API, receives a response, and extracts the content from the JSON response.

3. `open_md_in_preview`: This function opens a markdown file in VS Code's preview mode.

4. `main`: This is the primary function that uses the above functions. It reads arguments from the command line, runs the git command, checks for changes, and if found, summarizes these changes via the OpenAI API. It then writes the summaries along with commit details in a markdown file. If
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 11 (9040f7403648c5585a37217f564f6fbb81b4dba5)</summary>

The commit represents the initial project setup. A new file, `Cargo.toml`, has been created, which is the package manifest for the Rust language. This file specifies the package name as `summarize_recent_commit`, its version as `0.1.0`, and the Rust edition as `2018`.

Several dependencies necessary for the project have been added, including `tokio` (version 1) with full features, `reqwest` (version 0.11) with json features, `serde` (version 1.0) with derive features, and `serde_json` (version 1.0). Other dependencies that have been added are `regex` (version 1.5), `tempfile` (version 3.3), `open` (version 3.0), and `colored` (version 2.0). These dependencies are crucial for the project to function properly.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 12 (a6edcd79938f32ade80ea7cc3daff21510317c5c)</summary>

This is the initial commit by Matthew Diakonov. Two key files have been added:

1. A LICENSE file: This file establishes the project under the MIT License, granting permission for free use, modification, and distribution of the software with certain conditions, including the inclusion of the copyright notice and permission notice in all copies or substantial portions of the software.

2. A README.md file: The initial version of this file provides a brief description of the project, stating that its purpose is to summarize commits of teammates using LLM to save time.
</details>

------------------------------------------------------------------------

