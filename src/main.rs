use std::fs;
use std::io::{self, Read};
use std::path::Path;

use sha2::{Sha256, Digest};
use blake3;
use twox_hash::XxHash64;
use std::hash::Hasher;
use rayon::prelude::*;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use serde::Serialize;

mod hashing;
mod filter;
mod parallel;
mod quarantine;
mod report;

use hashing::{HashAlgo, calculate_hash};
use filter::file_filter;
use parallel::process_files_parallel;
use quarantine::move_to_quarantine;
use report::{generate_json_report, generate_html_report};

#[derive(Serialize)]
pub struct FileEntry {
    pub path: String,
    pub size: u64,
    pub hash: String,
}

fn main() -> io::Result<()> {
    // Configurable parameters
    let target_dir = ".";
    let min_size = 1; // bytes
    let max_size = u64::MAX;
    let allowed_exts = vec!["txt", "rs", "md"];
    let name_regex = Regex::new(r".*").unwrap();

    // 1. Walk directory and collect files
    let files: Vec<_> = WalkDir::new(target_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| file_filter(e, min_size, max_size, &allowed_exts, &name_regex))
        .collect();

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .unwrap()
        .progress_chars("#>-") );

    let results: Vec<_> = process_files_parallel(&files, |entry| {
        let path = entry.path();
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        let hash = calculate_hash(path, HashAlgo::Blake3)
            .map(|h| hex::encode(h))
            .unwrap_or_else(|_| "ERROR".to_string());
        FileEntry {
            path: path.display().to_string(),
            size,
            hash,
        }
    });
    pb.finish();

    // 3. Group by hash (find duplicates)
    use std::collections::HashMap;
    let mut groups: HashMap<String, Vec<&FileEntry>> = HashMap::new();
    for entry in &results {
        groups.entry(entry.hash.clone()).or_default().push(entry);
    }
    let duplicates: Vec<_> = groups.values().filter(|g| g.len() > 1).collect();

    // 4. Print JSON report
    let report = generate_json_report(&duplicates);
    println!("{}", report);

    // Print user-friendly duplicate info
    if duplicates.is_empty() {
        println!("No duplicate files found.");
    } else {
        println!("Duplicate files found:");
        for group in &duplicates {
            println!("Group:");
            for entry in *group {
                println!("  {}", entry.path);
            }
        }
    }

    // TODO: Quarantine system (move duplicates to safe folder before deletion)
    // TODO: HTML report generation

    Ok(())
}

