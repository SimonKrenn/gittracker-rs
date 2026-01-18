use clap::Parser;
use serde::Serialize;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(name = "gittracker-rs", about = "Scan folders for git repos with uncommitted changes")]
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
    changes: usize,
}

fn main() {
    let cli = Cli::parse();
    let statuses = scan_root(&cli.root);

    if cli.json {
        print_json(&statuses);
    } else {
        print_human(&statuses, cli.show_clean);
    }

    let has_dirty = statuses.iter().any(|status| status.is_dirty);
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
        .arg("--porcelain")
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let changes = stdout.lines().count();
            RepoStatus {
                path: repo_root.to_path_buf(),
                is_dirty: changes > 0,
                changes,
            }
        }
        Err(_) => RepoStatus {
            path: repo_root.to_path_buf(),
            is_dirty: false,
            changes: 0,
        },
    }
}

fn print_human(statuses: &[RepoStatus], show_clean: bool) {
    for status in statuses {
        if status.is_dirty {
            println!("dirty: {} ({} files)", status.path.display(), status.changes);
        } else if show_clean {
            println!("clean: {}", status.path.display());
        }
    }
}

fn print_json(statuses: &[RepoStatus]) {
    let json = serde_json::to_string_pretty(statuses).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}
