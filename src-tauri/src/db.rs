use serde::{Deserialize, Serialize};
use sled::Db;
use std::error::Error;

// 1. THE SCHEMA
// These structs define exactly what we save to the disk.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MoveStats {
    pub move_uci: String, // The move string, e.g., "e2e4"
    pub white_wins: u32,
    pub black_wins: u32,
    pub draws: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PositionStats {
    pub total_games: u32,
    pub moves: Vec<MoveStats>, // A list of all moves played in this position
}

// 2. THE DATABASE WRAPPER
pub struct Database {
    // The actual connection to the Sled database file
    tree: Db,
}

impl Database {
    // Open (or create) the database file at a specific path
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let tree = sled::open(path)?;
        Ok(Self { tree })
    }

    // Write statistics for a specific position (Hash -> Data)
    pub fn insert_stats(&self, hash: u64, stats: &PositionStats) -> Result<(), Box<dyn Error>> {
        // 1. Convert our Rust struct into raw bytes (binary)
        let encoded_bytes = bincode::serialize(stats)?;

        // 2. Convert the u64 hash into 8 bytes (Sled only understands bytes)
        let key = hash.to_be_bytes();

        // 3. Save it
        self.tree.insert(key, encoded_bytes)?;
        Ok(())
    }

    // Read statistics for a specific position
    pub fn get_stats(&self, hash: u64) -> Result<Option<PositionStats>, Box<dyn Error>> {
        let key = hash.to_be_bytes();

        match self.tree.get(key)? {
            Some(bytes) => {
                // We found data! Convert bytes back into our Rust Struct
                let stats = bincode::deserialize(&bytes)?;
                Ok(Some(stats))
            },
            None => Ok(None), // No data found for this hash
        }
    }
}

// 3. THE TESTS
// This is the "Base line for implementing test" we discussed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_write_read_flow() {
        // 1. Setup: Use a temporary database that deletes itself after the test
        let config = sled::Config::new().temporary(true);
        let db_tree = config.open().unwrap();
        let database = Database { tree: db_tree };

        // 2. Create dummy data
        let dummy_hash: u64 = 123456789;
        let dummy_stats = PositionStats {
            total_games: 10,
            moves: vec![
                MoveStats {
                    move_uci: "e2e4".to_string(),
                    white_wins: 4,
                    black_wins: 1,
                    draws: 5,
                }
            ]
        };

        // 3. Action: Write to DB
        database.insert_stats(dummy_hash, &dummy_stats).expect("Failed to write");

        // 4. Assert: Read it back and check it's identical
        let result = database.get_stats(dummy_hash).expect("Failed to read");
        assert_eq!(result, Some(dummy_stats));

        // 5. Assert: Check a hash that doesn't exist
        let missing = database.get_stats(99999).expect("Failed to read missing");
        assert!(missing.is_none());
    }
}