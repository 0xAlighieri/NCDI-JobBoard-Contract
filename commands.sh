near login
export ACCOUNT=<YOUR_TESTNET_ACCOUNT>

cargo build --target wasm32-unknown-unknown --release 

near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/job_board_contract.wasm
export CONTRACT=<YOUR_DEV_CONTRACT_ADDRESS>

near call $CONTRACT new --accountId $ACCOUNT

near call $CONTRACT add_posting '{"title":"A title", "description":"Hello there, please apply to my job.", "contact":"hello@hello.com"}' --accountId $ACCOUNT 
near call $CONTRACT add_reply '{"github":"github.com/me", "description":"I am a great Rust dev. I think you should hire me :)", "contact":"me@me.com", "posting_id":0}' --accountId $ACCOUNT
near call $CONTRACT add_reply '{"github":"github.com/me", "description":"I am a great Rust dev. I think you should hire me :)", "contact":"me@me.com", "posting_id":1}' --accountId $ACCOUNT

near call $CONTRACT get_postings '{"from_index":0, "limit":5}' --accountId $ACCOUNT

near call $CONTRACT get_postings_replies '{"posting_id":0}' --accountId $ACCOUNT

near call $CONTRACT remove_posting '{"posting_id":0}' --accountId $ACCOUNT

