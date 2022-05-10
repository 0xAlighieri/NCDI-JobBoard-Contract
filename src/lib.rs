use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Posting {
    id: u32,
    title: String,
    description: String,
    contact: String,
}

impl Default for Posting {
    fn default() -> Self {
        env::panic(b"Posting should be fully defined.")
    }
}
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Reply {
    github: String,
    description: String,
    contact: String,
}

impl Default for Reply {
    fn default() -> Self {
        env::panic(b"Posting should be fully defined.")
    }
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct JobBoard {
    posting_id_to_account_id_mapping: UnorderedMap<u32, AccountId>,
    postings: UnorderedMap<u32, Posting>,
    posting_replies: UnorderedMap<u64, Reply>,
    reply_id_to_posting_id: UnorderedMap<u64, u32>,
    posting_id: u32,
    reply_id: u64,
}

impl Default for JobBoard {
    fn default() -> Self {
        env::panic(b"JobBoard should be initialized before usage.")
    }
}

#[near_bindgen]
impl JobBoard {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            posting_id_to_account_id_mapping: UnorderedMap::new(b"m".to_vec()),
            postings: UnorderedMap::new(b"p".to_vec()),
            posting_replies: UnorderedMap::new(b"r".to_vec()),
            reply_id_to_posting_id: UnorderedMap::new(b"a".to_vec()),
            posting_id: 0,
            reply_id: 0,
        }
    }

    // Function to add a new posting
    pub fn add_posting(&mut self, title: String, description: String, contact: String) -> Posting {
        let account_id = env::predecessor_account_id();
        let posting = Posting {
            id: self.posting_id,
            title,
            description,
            contact,
        };
        self.posting_id_to_account_id_mapping
            .insert(&self.posting_id, &account_id);
        self.postings.insert(&self.posting_id, &posting);
        self.posting_id += 1;
        posting
    }
    // Function to remove a posting and related replies
    pub fn remove_posting(&mut self, posting_id: u32) -> Posting {
        assert!(self
            .posting_id_to_account_id_mapping
            .get(&posting_id)
            .is_some());
        let posting_account_id = self.posting_id_to_account_id_mapping.get(&posting_id);
        assert!(
            posting_account_id.unwrap() == env::predecessor_account_id(),
            "Addresses do not match; you do not have permission to remove this posting."
        );
        let removed_post = self.postings.remove(&posting_id).unwrap();
        self.posting_id_to_account_id_mapping.remove(&posting_id);
        let vector_of_removal_keys = self.posting_replies_filter(posting_id);
        for id in vector_of_removal_keys {
            self.posting_replies.remove(&id);
            self.reply_id_to_posting_id.remove(&id);
        }
        removed_post
    }
    // Function to get postings [PAGINATED] (example from https://www.near-sdk.io/contract-structure/collections)
    pub fn get_postings(&self, from_index: u64, limit: u64) -> Vec<(u32, Posting)> {
        assert!(
            self.postings.len() > 0,
            "ERROR: There are no postings to retrieve yet."
        );
        let keys = self.postings.keys_as_vector();
        let values = self.postings.values_as_vector();
        (from_index..std::cmp::min(from_index + limit, self.postings.len()))
            .map(|index| (keys.get(index).unwrap(), values.get(index).unwrap()))
            .collect()
    }

    // Function to get replies for a posting
    pub fn get_postings_replies(&self, posting_id: u32) -> Vec<Reply> {
        let vec_of_reply_ids = self.posting_replies_filter(posting_id);
        let mut vec_replies: Vec<Reply> = Vec::new();
        if vec_of_reply_ids.len() > 0 {
            for id in vec_of_reply_ids {
                let reply = self.posting_replies.get(&id);
                vec_replies.push(reply.unwrap());
            }
        }
        vec_replies
    }
    //Function to add a reply to a posting
    pub fn add_reply(
        &mut self,
        github: String,
        description: String,
        contact: String,
        posting_id: u32,
    ) -> Reply {
        let reply = Reply {
            github,
            description,
            contact,
        };
        self.posting_replies.insert(&self.reply_id, &reply);
        self.reply_id_to_posting_id
            .insert(&self.reply_id, &posting_id);
        self.reply_id += 1;
        reply
    }

    // HELPERS - PRIVATE FUNCTIONS

    // Data wrangling for posting replies getter
    fn posting_replies_filter(&self, posting_id: u32) -> Vec<u64> {
        let keys = self.reply_id_to_posting_id.keys_as_vector();
        let values = self.reply_id_to_posting_id.values_as_vector();
        let mut vector_of_filtered_keys: Vec<u64> = Vec::new();
        values.iter().enumerate().for_each(|(i, v)| {
            if v == posting_id {
                let removal_key = keys.get(i as u64);
                vector_of_filtered_keys.push(removal_key.unwrap());
            }
        });

        vector_of_filtered_keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn robert() -> AccountId {
        "robert.testnet".to_string()
    }
    fn mike() -> AccountId {
        "mike.testnet".to_string()
    }

    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // Tests
    #[test]
    fn create_posting() {
        let context = get_context(robert(), 0);
        testing_env!(context);
        let mut contract = JobBoard::new();
        let mut number_of_posts = contract.postings.len();
        let mut current_posting_id = contract.posting_id;
        assert_eq!(
            0, number_of_posts,
            "Expected posts to be empty on initialize."
        );
        assert_eq!(
            0, current_posting_id,
            "Posting ID should be 0 on initialize."
        );
        contract.add_posting(
            "title".to_string(),
            "description".to_string(),
            "contact".to_string(),
        );
        number_of_posts = contract.postings.len();
        current_posting_id = contract.posting_id;
        assert_eq!(1, number_of_posts, "Expected there to be a post.");
        assert_eq!(
            1, current_posting_id,
            "Posting ID should have iterated by 1."
        );
    }
    #[test]
    fn get_postings() {
        let context = get_context(robert(), 0);
        testing_env!(context);
        let mut contract = JobBoard::new();

        contract.add_posting(
            "title".to_string(),
            "description".to_string(),
            "contact".to_string(),
        );
        contract.add_posting(
            "title".to_string(),
            "description".to_string(),
            "contact".to_string(),
        );
        assert!(
            contract.get_postings(0, 10).len() == 2,
            "There should be 2 postings"
        )
    }

    #[test]
    fn create_posting_reply() {
        let context = get_context(robert(), 0);
        testing_env!(context);
        let mut contract = JobBoard::new();
        contract.add_posting(
            "title".to_string(),
            "description".to_string(),
            "contact".to_string(),
        );
        contract.add_reply(
            "github".to_string(),
            "description".to_string(),
            "contact".to_string(),
            0,
        );
        assert!(contract.posting_replies.len() > 0, "No replies were found");
        assert!(
            contract.get_postings_replies(0).len() > 0,
            "No replies were found for post with ID {}",
            0
        )
    }

    #[test]
    fn delete_posting() {
        let context = get_context(robert(), 0);
        testing_env!(context);
        let mut contract = JobBoard::new();
        contract.add_posting(
            "title".to_string(),
            "description".to_string(),
            "contact".to_string(),
        );
        contract.add_reply(
            "github".to_string(),
            "description".to_string(),
            "contact".to_string(),
            0,
        );
        contract.remove_posting(0);
        assert!(
            contract.postings.len() == 0,
            "There should be no postings after the only one was deleted."
        );
        assert!(
            contract.get_postings_replies(0).len() == 0,
            "replies should have been deleted."
        );
    }
    #[test]
    #[should_panic(
        expected = "Addresses do not match; you do not have permission to remove this posting."
    )]
    fn delete_posting_as_other_account() {
        let context = get_context(robert(), 0);
        testing_env!(context);
        let mut contract = JobBoard::new();
        contract.add_posting(
            "title".to_string(),
            "description".to_string(),
            "contact".to_string(),
        );
        let context2 = get_context(mike(), 0);
        testing_env!(context2);
        contract.remove_posting(0);
    }
}
