# Summarize commits and diffs of your team members with a single command

I hate reading through thousands of commits from my team or trying to understand large diffs. It takes a lot of time, the commit messages are often incomplete, and the file diffs don't provide an immediate understanding of the changes. Additionally, copying commits or diffs manually to ChatGPT is frustrating.

Solved! Use your normal git commands to summarize all commits or current diffs from your team into a single markdown text file.

## Next steps

If you like it, tell me, I'll package it into a npm/brew library to make it easier to use.
Need a feature? Tell me
[Didn't work?](https://github.com/m13v/summarize_recent_commit/issues/new?assignees=&labels=dislike&template=dislike.yml&title=installation+didnt+work)

## Give it a Star!

If you find this project useful, please give it a star! It helps us to grow and improve.
[![GitHub stars](https://img.shields.io/github/stars/m13v/summarize_recent_commit.svg?style=social&label=Star)](https://github.com/m13v/summarize_recent_commit/stargazers)

## Reach out:

i@m13v.com. Discord: matthew.ddy

## Getting started

Install [Rust](https://www.rust-lang.org/tools/install).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repo:

```bash
git clone https://github.com/m13v/summarize_recent_commit.git
```

Set up you OPENAI API KEY in .env

```bash
echo "OPENAI_API_KEY=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" > .env
```

Build it

```bash
cargo build --release
```

Run it # enter path of your git project repo (# under the hood it runs a separate command to get details of each commit or the current diff)

```bash
# Summarize all commits between HEAD and origin/main
cargo run --release -- -r /path/to/your/repo -g log,HEAD..origin/main -s all -p summary,technical

# Summarize the last 5 commits individually
cargo run --release -- -r /path/to/your/repo -g log,-n,5 -s individual -p summary,blog

# Summarize the current diff
cargo run --release -- -r /path/to/your/repo -g diff -p summary,technical

# Show help information
cargo run --release -- --help
```

Replace `/path/to/your/repo` with the path to your git repository.

## Usage

```
cargo run --release -- -r <repo_path> -g <git_command> [-s <summary_type>] -p <prompt_types>
```

- `-r <repo_path>`: Path to the git repository
- `-g <git_command>`: Git command to execute (e.g., 'log,-n,5', 'log,HEAD..origin/main', 'diff')
- `-s <summary_type>`: Type of summary: 'all', 'individual', or 'both' (only for log commands)
- `-p <prompt_types>`: Types of summaries to generate (summary, technical, blog)

For more detailed information, use the help command:

```
cargo run --release -- --help
```

## Overview

Summarize Recent Commit is a tool that helps you quickly understand the changes made in the most recent commits of your project or in the current diff. It provides a concise summary of the commit messages, files changed, and the impact of those changes.

## Features

- **Commit Summary**: Get a brief overview of the latest commits.
- **Diff Summary**: Understand the current changes in your working directory.
- **File Changes**: See which files were added, modified, or deleted.
- **Impact Analysis**: Understand the potential impact of the changes.

Written in Rust

Matthew Diakonov
