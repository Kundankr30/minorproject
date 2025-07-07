use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::DirEntry;

pub fn process_files_parallel<T, F>(files: &[DirEntry], process_fn: F) -> Vec<T>
where
    F: Fn(&DirEntry) -> T + Sync,
    T: Send,
{
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .unwrap()
        .progress_chars("#>-") );
    let results: Vec<T> = files.par_iter().map(|entry| {
        pb.inc(1);
        process_fn(entry)
    }).collect();
    pb.finish();
    results
} 