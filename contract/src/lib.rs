
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log_str;
use near_sdk::near_bindgen;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    greeting: String,
}


impl Default for Contract {
    fn default() -> Self {
        Self { greeting: "Hello".to_string() }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {

    pub fn get_greeting(&self) -> String {
        return self.greeting.clone();
    }

    pub fn set_greeting(&mut self, greeting: String) {
        log_str(&format!("Saving greeting: {greeting}"));
        self.greeting = greeting;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
