use dotenv::dotenv;
use std::env;
use std::process::Command;
use std::str;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use colored::*;

fn run_git_command(repo_path: &str, git_command: &[String]) -> Result<String, String> {
    let mut command = Command::new("git");
    command.arg("-C").arg(repo_path);
    command.args(git_command);

    println!("Running command: {:?}", command);

    let output = command.output().map_err(|e| format!("Failed to execute command: {}", e))?;
    println!("Git: {}", output.status);
    if output.status.success() {
        println!("{}", "all good".green());
        let stdout = str::from_utf8(&output.stdout)
            .map_err(|e| format!("Failed to convert output to string: {}", e))?;
        Ok(stdout.to_string())
    } else {
        let stderr = str::from_utf8(&output.stderr)
            .map_err(|e| format!("Failed to convert error to string: {}", e))?;
        println!("Git command error: {}", stderr);
        Err(stderr.to_string())
    }
}

async fn summarize_changes(changes: &str, system_prompt: &str, user_prompt: &str, max_tokens: u32) -> Result<String, String> {
    let client = reqwest::Client::new();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env file");
    let url = "https://api.openai.com/v1/chat/completions";

    let prompt = format!("{}\n\n{}", user_prompt, changes);

    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-4",
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": prompt}
            ],
            "max_tokens": max_tokens,
            "temperature": 0.7,
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let response_json: serde_json::Value = response.json().await.map_err(|e| format!("Failed to read response: {}", e))?;
    let content = response_json["choices"][0]["message"]["content"].as_str().ok_or("Failed to extract content")?;
    Ok(content.to_string())
}

fn open_md_in_preview(file_path: &str) {
    // Open the file in VS Code and show the preview
    Command::new("code")
        .arg(file_path)
        .arg("--command")
        .arg("markdown.showPreview")
        .spawn()
        .expect("Failed to open file in VS Code preview mode");
}

fn print_help() {
    println!("{}", "Git Commit Summarizer".green().bold());
    println!("{}", "------------------------".green());
    println!("A tool to summarize git commits using AI-powered analysis.\n");

    println!("{}", "USAGE:".yellow().bold());
    println!("  summarize_recent_commit <repo_path> git <git_command> <summary_type> [prompt_types...]\n");

    println!("{}", "ARGUMENTS:".yellow().bold());
    println!("  <repo_path>      Path to the git repository");
    println!("  git              Use 'git' as a fixed argument");
    println!("  <git_command>    Git command to execute (e.g., 'log -n 5')");
    println!("  <summary_type>   Type of summary: 'all' or 'individual'");
    println!("  [prompt_types]   Optional: Types of summaries to generate\n");

    println!("{}", "SUMMARY TYPES:".yellow().bold());
    println!("  all              Summarize all commits together");
    println!("  individual       Summarize each commit individually\n");

    println!("{}", "PROMPT TYPES:".yellow().bold());
    println!("  summary          General summary of changes");
    println!("  technical        Technical analysis of code changes");
    println!("  blog             Blog-style summary of project progress\n");

    println!("{}", "EXAMPLES:".yellow().bold());
    println!("  summarize_recent_commit /path/to/repo git log -n 5 all summary technical");
    println!("  summarize_recent_commit /path/to/repo git log -n 3 individual summary blog\n");

    println!("{}", "NOTE:".yellow().bold());
    println!("  If no prompt types are specified, 'summary' will be used by default.");
}

fn get_current_commit(repo_path: &str) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .map_err(|e| format!("Failed to execute git rev-parse: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(format!("Git rev-parse command failed: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    // Check for help command
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return;
    }

    if args.len() < 5 {
        eprintln!("Error: Not enough arguments provided.");
        eprintln!("Use --help or -h for usage information.");
        return;
    }

    let repo_path = &args[1];
    let summary_type = &args[4];
    let git_command: Vec<String> = args[3..4].to_vec();

    let current_commit = match get_current_commit(repo_path) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Error getting current commit: {}", e);
            String::from("Unknown")
        }
    };

    // Define core prompt types with improved system prompts
    let all_prompt_types: Vec<(&str, &str, &str)> = vec![
        ("summary",
         "You are an AI assistant specializing in summarizing git commits. Provide clear, concise summaries that capture the essence of the changes made in each commit. Focus on the most important modifications and their potential impact on the project.",
         "Summarize the following git changes, highlighting the key modifications:"),

        ("technical",
         "You are an expert software developer with deep knowledge of various programming languages and software architecture. Analyze git commits from a technical perspective, focusing on code changes, potential optimizations, and adherence to best practices.",
         "Provide a technical summary of the following git changes, including code analysis and potential implications:"),

        ("blog",
         "You are a tech blogger with a knack for explaining complex technical concepts in an engaging and accessible way. Your goal is to create interesting, informative content about software development and project progress based on git commits.",
         "Write a brief, engaging blog post summarizing the following git changes, focusing on the overall progress and interesting developments:"),
    ];

    // Find the index where prompt types start (if any)
    let prompt_start_index = args.iter().position(|arg| all_prompt_types.iter().any(|(name, ..)| *name == arg));

    // Select prompt types based on user input or use default if none specified
    let selected_prompts: Vec<String> = if let Some(index) = prompt_start_index {
        args[index..].to_vec()
    } else {
        vec!["summary".to_string()]
    };

    // Filter prompt types based on user selection
    let prompt_types: Vec<_> = all_prompt_types
        .into_iter()
        .filter(|(name, ..)| selected_prompts.contains(&name.to_string()))
        .collect();

    if prompt_types.is_empty() {
        eprintln!("No valid prompt types selected. Using summary prompt.");
        return;
    }

    match run_git_command(repo_path, &git_command) {
        Ok(changes) => {
            if changes.trim().is_empty() {
                eprintln!("No changes found in the specified range.");
                return;
            }

            // Extract commit hashes
            let re = Regex::new(r"commit (\b[0-9a-f]{5,40}\b)").unwrap();
            let commit_hashes: Vec<&str> = re.captures_iter(&changes)
                .map(|cap| cap.get(1).unwrap().as_str())
                .collect();

            println!("-----------------------------------------------------------------------");
            println!("Current commit: {}", current_commit.green());
            println!("Once all commits are processed the script will open a summary text file");
            println!("_______________________________________________________________________");
            println!("Total number of commits: {}", commit_hashes.len());

            let current_dir = env::current_dir().expect("Failed to get current directory");
            let file_path = current_dir.join("git_commit_summaries.md");
            let file_path_str = file_path.to_str().expect("Failed to convert file path to string");
            let mut file = File::create(&file_path).expect("Failed to create file");

            writeln!(file, "# Git Commit Summaries\n").expect("Failed to write to file");
            writeln!(file, "Current commit: {}\n", current_commit).expect("Failed to write to file");
            writeln!(file, "-----------------------------------------------------------------------").expect("Failed to write to file");
            writeln!(file, "-----------------------------------------------------------------------").expect("Failed to write to file");
            writeln!(file, " ").expect("Failed to write to file");
            writeln!(file, "PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN").expect("Failed to write to file");
            writeln!(file, " ").expect("Failed to write to file");
            writeln!(file, "_______________________________________________________________________").expect("Failed to write to file");
            writeln!(file, "-----------------------------------------------------------------------").expect("Failed to write to file");
            writeln!(file, "Total number of commits: {}\n", commit_hashes.len()).expect("Failed to write to file");

            if summary_type == "all" {
                // Summarize all commits together
                let all_commit_details = commit_hashes.iter()
                    .filter_map(|&commit_hash| {
                        let git_show_command = vec!["show".to_string(), commit_hash.to_string()];
                        run_git_command(repo_path, &git_show_command).ok()
                    })
                    .collect::<Vec<String>>()
                    .join("\n\n");

                for (prompt_name, system_prompt, user_prompt) in &prompt_types {
                    match summarize_changes(&all_commit_details, system_prompt, user_prompt, 500).await {
                        Ok(summary) => {
                            writeln!(file, "## {} for all commits\n\n{}\n", prompt_name, summary)
                                .expect("Failed to write to file");
                        }
                        Err(e) => eprintln!("Error summarizing all changes for {}: {}", prompt_name, e),
                    }
                }
            } else {
                // Summarize commits individually
                for (index, commit_hash) in commit_hashes.iter().enumerate() {
                    let git_show_command = vec!["show".to_string(), commit_hash.to_string()];
                    match run_git_command(repo_path, &git_show_command) {
                        Ok(commit_details) => {
                            // println!("Commit details for {}: {}", commit_hash, commit_details); // Debug log
                            for (prompt_name, system_prompt, user_prompt) in &prompt_types {
                                match summarize_changes(&commit_details, system_prompt, user_prompt, 200).await {
                                    Ok(summary) => {
                                        writeln!(file, "<details>\n<summary>{} for commit {} ({})</summary>\n\n{}\n</details>\n",
                                                 prompt_name, index + 1, commit_hash, summary)
                                            .expect("Failed to write to file");
                                    }
                                    Err(e) => eprintln!("Error summarizing changes for {} ({}): {}", commit_hash, prompt_name, e),
                                }
                            }
                            writeln!(file, "------------------------------------------------------------------------\n")
                                .expect("Failed to write to file");
                        }
                        Err(e) => eprintln!("Error running git show for {}: {}", commit_hash, e),
                    }
                }
            }

            open_md_in_preview(file_path_str);
        }
        Err(e) => eprintln!("Error running git command: {}", e),
    }
}