use std::time;
use sha2::Digest;

#[derive(Debug, Clone, PartialEq, Eq, Hash )]
pub struct Block {
    index: u32,
    timestamp: u64,
    previous_hash: String,
    data: String,
    hash: String,
    nonce: u32,
}

impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = get_current_timestamp();
        let nonce = 0; // Initial nonce valuelet
        let hash = calculate_hash(index, timestamp, &previous_hash, &data, nonce);
        Block {
            index, timestamp, previous_hash, data, hash, nonce
        }
    }
}

fn get_current_timestamp() -> u64 {
    let timestamp = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    timestamp
    
}

fn calculate_hash(index: u32, timestamp: u64, previous_hash: &str, data: &str, nonce: u32) -> String {
    let input = format!("{}{}{}{}{}", index, timestamp, previous_hash, data, nonce);
    let mut hasher = sha2::Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result) 
}