use std::fs;
use std::io::{self, Read};
use std::path::Path;
use sha2::{Sha256, Digest};
use blake3;
use twox_hash::XxHash64;
use std::hash::Hasher;

pub enum HashAlgo {
    Sha256,
    Blake3,
    XxHash,
}

pub fn calculate_hash(file_path: &Path, algo: HashAlgo) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(file_path)?;
    let mut buffer = [0; 8192];

    match algo {
        HashAlgo::Sha256 => {
            let mut hasher = Sha256::new();
            loop {
                let bytes_read = file.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }
            Ok(hasher.finalize().to_vec())
        }
        HashAlgo::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            loop {
                let bytes_read = file.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }
            Ok(hasher.finalize().as_bytes().to_vec())
        }
        HashAlgo::XxHash => {
            let mut hasher = XxHash64::with_seed(0);
            loop {
                let bytes_read = file.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.write(&buffer[..bytes_read]);
            }
            Ok(hasher.finish().to_le_bytes().to_vec())
        }
    }
} 