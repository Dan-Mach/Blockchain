#[derive(Debug, Clone, PartialEq, Eq, Hash )]
pub struct Block {
    index: u32,
    timestamp: u64,
    previous_hash: String,
    data: String,
    hash: String,
    nonce: u32,
}