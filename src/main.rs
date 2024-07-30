use clap::{App, Arg};
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
    // println!("Git: {}", output.status);
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
    Command::new("code")
        .arg(file_path)
        .arg("--command")
        .arg("markdown.showPreview")
        .spawn()
        .expect("Failed to open file in VS Code preview mode");
}

fn parse_args() -> App<'static, 'static> {
    App::new("Git Commit Summarizer")
        .version("1.0")
        .author("Your Name")
        .about("Summarizes git commits or current diff using AI-powered analysis")
        .arg(Arg::with_name("repo_path")
            .short("r")
            .long("repo")
            .value_name("REPO_PATH")
            .help("Path to the git repository")
            .required(true))
        .arg(Arg::with_name("git_command")
            .short("g")
            .long("git-command")
            .value_name("GIT_COMMAND")
            .help("Git command to execute (e.g., 'log,-n,5' for the last 5 commits, or 'diff' for current changes)")
            .required(true)
            .multiple(true)
            .use_delimiter(true))
        .arg(Arg::with_name("summary_type")
            .short("s")
            .long("summary-type")
            .value_name("SUMMARY_TYPE")
            .help("Type of summary to generate: 'all' (summarize all commits together), 'individual' (summarize each commit separately), or 'both' (only applicable for log commands)")
            .default_value("both")
            .possible_values(&["all", "individual", "both"]))
        .arg(Arg::with_name("prompt_types")
            .short("p")
            .long("prompt-types")
            .value_name("PROMPT_TYPES")
            .help("Types of summaries to generate: 'summary' (general overview), 'technical' (code-focused analysis), 'blog' (project progress narrative). Use commas to separate multiple values.")
            .multiple(true)
            .use_delimiter(true)
            .possible_values(&["summary", "technical", "blog"]))
        .after_help("EXAMPLES:\n    \
            Summarize last 5 commits with both overall and individual summaries:\n    \
            git_commit_summarizer -r /path/to/repo -g log,-n,5 -s both -p summary,technical\n\n    \
            Generate only an overall summary for all commits:\n    \
            git_commit_summarizer -r /path/to/repo -g log,--all -s all -p summary,blog\n\n    \
            Summarize individual commits from a specific branch:\n    \
            git_commit_summarizer -r /path/to/repo -g log,feature-branch -s individual -p technical\n\n    \
            Summarize current diff:\n    \
            git_commit_summarizer -r /path/to/repo -g diff -p summary,technical")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let matches = parse_args().get_matches();

    let repo_path = matches.value_of("repo_path").unwrap();
    let git_command: Vec<String> = matches.values_of("git_command").unwrap().map(String::from).collect();
    let summary_type = matches.value_of("summary_type").unwrap();
    let prompt_types: Vec<String> = matches.values_of("prompt_types")
        .map(|values| values.map(String::from).collect())
        .unwrap_or_else(|| vec!["summary".to_string()]);

    let all_prompt_types = vec![
        ("summary", "You are an AI assistant specializing in summarizing git changes. Provide clear, concise summaries that capture the essence of the changes made. Focus on the most important modifications and their potential impact on the project.", "Summarize the following git changes, highlighting the key modifications:"),
        ("technical", "You are an expert software developer with deep knowledge of various programming languages and software architecture. Analyze git changes from a technical perspective, focusing on code changes, potential optimizations, and adherence to best practices.", "Provide a technical summary of the following git changes, including code analysis and potential implications:"),
        ("blog", "You are a tech blogger with a knack for explaining complex technical concepts in an engaging and accessible way. Your goal is to create interesting, informative content about software development and project progress based on git changes.", "Write a brief, engaging blog post summarizing the following git changes, focusing on the overall progress and interesting developments:"),
    ];

    let selected_prompt_types: Vec<_> = all_prompt_types
        .into_iter()
        .filter(|(name, ..)| prompt_types.contains(&name.to_string()))
        .collect();

    if selected_prompt_types.is_empty() {
        eprintln!("No valid prompt types selected. Using summary prompt.");
        return Ok(());
    }

    let changes = run_git_command(repo_path, &git_command)?;

    if changes.trim().is_empty() {
        eprintln!("No changes found in the specified range.");
        return Ok(());
    }

    let current_dir = env::current_dir()?;
    let file_path = current_dir.join("git_changes_summary.md");
    let file_path_str = file_path.to_str().ok_or("Failed to convert file path to string")?;
    let mut file = File::create(&file_path)?;

    writeln!(file, "# Git Changes Summary\n")?;

    if git_command[0] == "diff" {
        writeln!(file, "## Current Diff Summary\n")?;
        summarize_diff(&changes, &selected_prompt_types, &mut file).await?;
    } else {
        let re = Regex::new(r"commit (\b[0-9a-f]{5,40}\b)")?;
        let commit_hashes: Vec<&str> = re.captures_iter(&changes)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect();

        writeln!(file, "Total number of commits: {}\n", commit_hashes.len())?;

        if summary_type == "all" || summary_type == "both" {
            summarize_all_commits(repo_path, &commit_hashes, &selected_prompt_types, &mut file).await?;
        }

        if summary_type == "individual" || summary_type == "both" {
            summarize_individual_commits(repo_path, &commit_hashes, &selected_prompt_types, &mut file).await?;
        }
    }

    open_md_in_preview(file_path_str);

    Ok(())
}

async fn summarize_all_commits(repo_path: &str, commit_hashes: &[&str], prompt_types: &[(&str, &str, &str)], file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    let all_commit_details = commit_hashes.iter()
        .filter_map(|&commit_hash| {
            let git_show_command = vec!["show".to_string(), commit_hash.to_string()];
            run_git_command(repo_path, &git_show_command).ok()
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    writeln!(file, "## Summary of All Commits\n")?;

    for (prompt_name, system_prompt, user_prompt) in prompt_types {
        match summarize_changes(&all_commit_details, system_prompt, user_prompt, 500).await {
            Ok(summary) => {
                writeln!(file, "### {} for all commits\n\n{}\n", prompt_name, summary)?;
            }
            Err(e) => eprintln!("Error summarizing all changes for {}: {}", prompt_name, e),
        }
    }

    writeln!(file, "------------------------------------------------------------------------\n")?;
    Ok(())
}

async fn summarize_individual_commits(repo_path: &str, commit_hashes: &[&str], prompt_types: &[(&str, &str, &str)], file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(file, "## Individual Commit Summaries\n")?;

    for (index, commit_hash) in commit_hashes.iter().enumerate() {
        let git_show_command = vec!["show".to_string(), commit_hash.to_string()];
        match run_git_command(repo_path, &git_show_command) {
            Ok(commit_details) => {
                for (prompt_name, system_prompt, user_prompt) in prompt_types {
                    match summarize_changes(&commit_details, system_prompt, user_prompt, 200).await {
                        Ok(summary) => {
                            writeln!(file, "<details>\n<summary>{} for commit {} ({})</summary>\n\n{}\n</details>\n",
                                     prompt_name, index + 1, commit_hash, summary)?;
                        }
                        Err(e) => eprintln!("Error summarizing changes for {} ({}): {}", commit_hash, prompt_name, e),
                    }
                }
                writeln!(file, "------------------------------------------------------------------------\n")?;
            }
            Err(e) => eprintln!("Error running git show for {}: {}", commit_hash, e),
        }
    }
    Ok(())
}

async fn summarize_diff(diff: &str, prompt_types: &[(&str, &str, &str)], file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    for (prompt_name, system_prompt, user_prompt) in prompt_types {
        match summarize_changes(diff, system_prompt, user_prompt, 500).await {
            Ok(summary) => {
                writeln!(file, "### {} for current diff\n\n{}\n", prompt_name, summary)?;
            }
            Err(e) => eprintln!("Error summarizing diff for {}: {}", prompt_name, e),
        }
    }
    writeln!(file, "------------------------------------------------------------------------\n")?;
    Ok(())
}