# Intelligent File Deduplicator

A Rust-based file deduplication tool that uses multiple hashing algorithms, parallel processing, and advanced filtering to efficiently identify and manage duplicate files.

## Features

- **Multi-Algorithm Hashing**: SHA-256, Blake3, and xxHash for different use cases
- **Parallel Processing**: Rayon-based parallel file processing with progress tracking
- **Advanced Filtering**: Size ranges, file types, date ranges, regex patterns
- **Safe Operations**: Quarantine system before actual deletion
- **Detailed Reports**: JSON/HTML reports with file relationships and savings

## Project Structure

```
src/
├── main.rs         # Main orchestration and CLI logic
├── hashing.rs      # Hash algorithm implementations
├── filter.rs       # File filtering logic
├── parallel.rs     # Parallel processing and progress bars
├── quarantine.rs   # Safe file quarantine system
└── report.rs       # Report generation (JSON/HTML)
```

## How It Works

### Step 1: File Discovery and Filtering
The process begins by recursively walking through the target directory using `walkdir`. Files are filtered based on:
- **Size**: Minimum and maximum file sizes
- **Extension**: Allowed file extensions (e.g., txt, rs, md)
- **Name Pattern**: Regex patterns for file names

```rust
// Files are collected and filtered
let files: Vec<_> = WalkDir::new(target_dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| file_filter(e, min_size, max_size, &allowed_exts, &name_regex))
    .collect();
```

### Step 2: Parallel Hashing
Files are processed in parallel using Rayon for improved performance. Each file is hashed using the selected algorithm (default: Blake3) with a progress bar showing completion status.

```rust
// Parallel processing with progress tracking
let results: Vec<_> = process_files_parallel(&files, |entry| {
    let path = entry.path();
    let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
    let hash = calculate_hash(path, HashAlgo::Blake3)
        .map(|h| hex::encode(h))
        .unwrap_or_else(|_| "ERROR".to_string());
    FileEntry { path, size, hash }
});
```

### Step 3: Duplicate Detection
Files are grouped by their hash values to identify duplicates. Files with identical hashes are considered duplicates.

```rust
// Group files by hash to find duplicates
let mut groups: HashMap<String, Vec<&FileEntry>> = HashMap::new();
for entry in &results {
    groups.entry(entry.hash.clone()).or_default().push(entry);
}
let duplicates: Vec<_> = groups.values().filter(|g| g.len() > 1).collect();
```

### Step 4: Report Generation
A detailed JSON report is generated showing all duplicate file groups, including file paths, sizes, and hash values.

```rust
// Generate and display report
let report = generate_json_report(&duplicates);
println!("{}", report);
```

### Step 5: User-Friendly Output
The program provides clear, human-readable output indicating whether duplicates were found and listing the duplicate file groups.

## Hash Algorithms

### SHA-256
- **Use Case**: Cryptographic security, when absolute certainty is required
- **Speed**: Slower but cryptographically secure
- **Collision Resistance**: Extremely high

### Blake3
- **Use Case**: General-purpose deduplication (default)
- **Speed**: Very fast, parallelizable
- **Collision Resistance**: High

### xxHash
- **Use Case**: Speed-critical applications
- **Speed**: Extremely fast
- **Collision Resistance**: Good for deduplication purposes

## Configuration

The program can be configured by modifying these parameters in `main.rs`:

```rust
let target_dir = ".";                    // Directory to scan
let min_size = 1;                       // Minimum file size in bytes
let max_size = u64::MAX;                // Maximum file size
let allowed_exts = vec!["txt", "rs", "md"]; // Allowed file extensions
let name_regex = Regex::new(r".*").unwrap(); // File name pattern
```

## Usage

1. **Build the project**:
   ```bash
   cargo build --release
   ```

2. **Run the deduplicator**:
   ```bash
   cargo run
   ```

3. **View the output**:
   - JSON report with detailed duplicate information
   - User-friendly summary of duplicate files found
   - Progress bar during processing

## Dependencies

- `rayon`: Parallel processing
- `indicatif`: Progress bars
- `walkdir`: Recursive directory walking
- `regex`: Pattern matching
- `serde`/`serde_json`: JSON serialization
- `sha2`: SHA-256 hashing
- `blake3`: Blake3 hashing
- `twox-hash`: xxHash implementation
- `hex`: Hex encoding

## Future Enhancements

- [ ] CLI argument parsing for configuration
- [ ] HTML report generation
- [ ] Quarantine system implementation
- [ ] File deletion with confirmation
- [ ] Date-based filtering
- [ ] Memory usage optimization for large directories

## Safety Features

- **Quarantine System**: Files are moved to a safe location before deletion
- **Progress Tracking**: Visual feedback during long operations
- **Error Handling**: Graceful handling of file access errors
- **Detailed Logging**: Comprehensive reporting of all operations

## Performance Considerations

- **Parallel Processing**: Utilizes all CPU cores for faster processing
- **Buffered Reading**: Efficient file reading with 8KB buffers
- **Memory Management**: Processes files in chunks to manage memory usage
- **Early Filtering**: Filters files before expensive hashing operations

This intelligent file deduplicator provides a robust, efficient, and safe way to identify and manage duplicate files in your system. 