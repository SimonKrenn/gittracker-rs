use clap::Parser;
use serde::Serialize;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(
    name = "gittracker-rs",
    about = "Scan folders for git repos with local changes"
)]
struct Cli {
    /// Root folder to scan
    #[arg(default_value = ".")]
    root: PathBuf,

    /// Output JSON instead of human-readable lines
    #[arg(long)]
    json: bool,

    /// Include clean repositories in output
    #[arg(long)]
    show_clean: bool,
}

#[derive(Debug, Serialize)]
struct RepoStatus {
    path: PathBuf,
    is_dirty: bool,
    uncommitted_changes: usize,
    unpushed_commits: usize,
    has_upstream: bool,
}

#[derive(Debug, Serialize)]
struct JsonOutput<'a> {
    total: usize,
    repos: &'a [RepoStatus],
}

fn main() {
    let cli = Cli::parse();
    let statuses = scan_root(&cli.root);

    if cli.json {
        print_json(&statuses);
    } else {
        print_human(&statuses, cli.show_clean);
    }

    let dirty_count = statuses.iter().filter(|status| status.is_dirty).count();
    let clean_count = statuses.len().saturating_sub(dirty_count);
    let uncommitted_count = statuses
        .iter()
        .filter(|status| status.uncommitted_changes > 0)
        .count();
    let unpushed_count = statuses
        .iter()
        .filter(|status| status.unpushed_commits > 0)
        .count();
    let has_dirty = dirty_count > 0;
    if !cli.json && !cli.show_clean && !has_dirty {
        println!("no repositories with local changes found");
    }

    if !cli.json {
        println!("scanned {} repositories", statuses.len());
        println!("dirty: {}, clean: {}", dirty_count, clean_count);
        println!(
            "repos with uncommitted changes: {}, unpushed commits: {}",
            uncommitted_count, unpushed_count
        );
    }

    if has_dirty {
        std::process::exit(1);
    }
}

fn scan_root(root: &Path) -> Vec<RepoStatus> {
    let mut statuses = Vec::new();
    let mut walker = WalkDir::new(root).follow_links(false).into_iter();

    while let Some(entry) = walker.next() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if is_git_marker(&entry) {
            let repo_root = entry.path().parent().unwrap_or(entry.path()).to_path_buf();
            statuses.push(get_repo_status(&repo_root));
        }

        if entry.file_type().is_dir() && entry.file_name() == OsStr::new(".git") {
            walker.skip_current_dir();
        }
    }

    statuses
}

fn is_git_marker(entry: &DirEntry) -> bool {
    if entry.file_name() != OsStr::new(".git") {
        return false;
    }

    entry.file_type().is_dir() || entry.file_type().is_file()
}

fn get_repo_status(repo_root: &Path) -> RepoStatus {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("status")
        .arg("--porcelain=2")
        .arg("-b")
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut uncommitted_changes = 0;
            let mut unpushed_commits = 0;
            let mut has_upstream = false;

            for line in stdout.lines() {
                if line.starts_with("# branch.upstream ") {
                    has_upstream = true;
                    continue;
                }

                if let Some(rest) = line.strip_prefix("# branch.ab ") {
                    for part in rest.split_whitespace() {
                        if let Some(ahead) = part.strip_prefix('+') {
                            if let Ok(value) = ahead.parse::<usize>() {
                                unpushed_commits = value;
                            }
                        }
                    }
                    continue;
                }

                if line.starts_with("1 ")
                    || line.starts_with("2 ")
                    || line.starts_with("u ")
                    || line.starts_with("? ")
                {
                    uncommitted_changes += 1;
                }
            }

            let is_dirty = uncommitted_changes > 0 || unpushed_commits > 0;
            RepoStatus {
                path: repo_root.to_path_buf(),
                is_dirty,
                uncommitted_changes,
                unpushed_commits,
                has_upstream,
            }
        }
        Err(_) => RepoStatus {
            path: repo_root.to_path_buf(),
            is_dirty: false,
            uncommitted_changes: 0,
            unpushed_commits: 0,
            has_upstream: false,
        },
    }
}

fn print_human(statuses: &[RepoStatus], show_clean: bool) {
    for status in statuses {
        if status.is_dirty {
            let upstream_note = if status.has_upstream {
                ""
            } else {
                ", upstream: none"
            };
            println!(
                "dirty: {} (uncommitted: {} files, unpushed: {} commits{})",
                status.path.display(),
                status.uncommitted_changes,
                status.unpushed_commits,
                upstream_note
            );
        } else if show_clean {
            println!("clean: {}", status.path.display());
        }
    }
}

fn print_json(statuses: &[RepoStatus]) {
    let output = JsonOutput {
        total: statuses.len(),
        repos: statuses,
    };
    let json = serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}
