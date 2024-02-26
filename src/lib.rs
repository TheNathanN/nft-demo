// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env::{self, log_str};
use near_sdk::serde::Serialize;
use near_sdk::{near_bindgen, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone)]
pub struct Token {
    token_id: u128,
    owner_id: AccountId,
    name: String,
    description: String,
    media_uri: String,
    level: u128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTContract {
    owner_by_id: UnorderedMap<u128, AccountId>,
    token_id: u128,
    token_by_id: UnorderedMap<u128, Token>,
}

impl Default for NFTContract {
    fn default() -> Self {
        Self {
            owner_by_id: UnorderedMap::new(b'm'),
            token_id: 0,
            token_by_id: UnorderedMap::new(b'n'),
        }
    }
}

#[near_bindgen]
impl NFTContract {
    pub fn mint(
        &mut self,
        token_owner_id: AccountId,
        name: String,
        description: String,
        media_uri: String,
        level: u128,
    ) -> Token {
        self.owner_by_id.insert(&self.token_id, &token_owner_id);

        let token = Token {
            token_id: self.token_id,
            owner_id: token_owner_id,
            name,
            description,
            media_uri,
            level,
        };

        self.token_by_id.insert(&self.token_id, &token);
        self.token_id += 1;

        log_str("Token created successfully");

        token
    }

    pub fn get_total_minted(&self) -> u128 {
        self.token_id
    }

    pub fn get_token_by_id(&self, token_id: u128) -> Token {
        self.token_by_id.get(&token_id).unwrap_or(Token {
            token_id: 0,
            owner_id: env::signer_account_id(),
            name: "Token not found".to_string(),
            description: "Token not found".to_string(),
            media_uri: "Token not found".to_string(),
            level: 0,
        })
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use near_sdk::env;

    use super::*;

    #[test]
    fn mint_token() {
        let mut contract = NFTContract::default();
        let token = contract.mint(
            env::signer_account_id(),
            "Test Name".to_string(),
            "Test description for token".to_string(),
            "media.png".to_string(),
            10,
        );
        assert_eq!(token.name, "Test Name".to_string());
        assert_eq!(token.description, "Test description for token".to_string());
    }

    #[test]
    fn get_total_minted() {
        let mut contract = NFTContract::default();

        contract.mint(
            env::signer_account_id(),
            "Test Name".to_string(),
            "Test description for token".to_string(),
            "media.png".to_string(),
            10,
        );
        contract.mint(
            env::signer_account_id(),
            "Test Name2".to_string(),
            "Test description for token2".to_string(),
            "media.png".to_string(),
            20,
        );
        contract.mint(
            env::signer_account_id(),
            "Test Name3".to_string(),
            "Test description for token3".to_string(),
            "media.png".to_string(),
            20,
        );

        assert_eq!(contract.get_total_minted(), 3);
    }

    #[test]
    fn get_token_by_id() {
        let mut contract = NFTContract::default();

        contract.mint(
            env::signer_account_id(),
            "Test Name".to_string(),
            "Test description for token".to_string(),
            "media.png".to_string(),
            10,
        );
        contract.mint(
            env::signer_account_id(),
            "Test Name2".to_string(),
            "Test description for token2".to_string(),
            "media.png".to_string(),
            20,
        );
        contract.mint(
            env::signer_account_id(),
            "Test Name3".to_string(),
            "Test description for token3".to_string(),
            "media.png".to_string(),
            20,
        );

        let found_token = contract.get_token_by_id(1);
        let not_found_token = contract.get_token_by_id(3);

        assert_eq!(found_token.name, "Test Name2".to_string());
        assert_eq!(not_found_token.name, "Token not found".to_string());
    }
}
