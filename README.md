# Intelligent File Deduplicator
# This is a minor project which is a part of Silicon Summer Intership course(Backend with Rust)

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

### Step 2: Parallel Hashing
Files are processed in parallel using Rayon for improved performance. Each file is hashed using the selected algorithm (default: Blake3) with a progress bar showing completion status.

### Step 3: Duplicate Detection
Files are grouped by their hash values to identify duplicates. Files with identical hashes are considered duplicates.

### Step 4: Report Generation
A detailed JSON report is generated showing all duplicate file groups, including file paths, sizes, and hash values.

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
