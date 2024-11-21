use near_sdk::{near_bindgen, env, collections::LookupMap, borsh::{self, BorshDeserialize, BorshSerialize}};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ISStorage {
    data: LookupMap<u64, u64>,
}

// Implement Default manually to initialize `LookupMap`
impl Default for ISStorage {
    fn default() -> Self {
        Self {
            data: LookupMap::new(b"d"), // Use a unique storage prefix
        }
    }
}

#[near_bindgen]
impl ISStorage {
    #[init]
    pub fn new() -> Self {
        assert!(
            !env::state_exists(),
            "The contract is already initialized"
        );
        Self::default()
    }

    pub fn insert_data(&mut self, key: u64, value: u64) {
        self.data.insert(&key, &value);
    }

    pub fn get_data(&self, key: u64) -> Option<u64> {
        self.data.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get_data() {
        let mut contract = ISStorage::default();
        assert_eq!(contract.insert_data(1, 100), None);
        assert_eq!(contract.get_data(1), Some(100));
    }
}
