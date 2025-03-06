#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Sha256, Digest};

pub fn main() {
    let input = sp1_zkvm::io::read::<String>();
	
    let mut hasher = Sha256::new();
	
    hasher.update(input);
	
    let result = hasher.finalize();
	
	let hash_hex = format!("{:x}", result);
	
    sp1_zkvm::io::commit(&hash_hex);
}
