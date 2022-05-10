# Simple JobBoard Smart Contract on Near 

## Introduction
This project is a simple implementation of a job board using a Rust-based smart contract that can be deployed on the NEAR blockchain, which was built as a submission for the Near Certified Developer (NCD I) course. 

Ultimately, the purpose of this project was to build a simple contract to explore how persistent storage, unit tests, and contract calls interact when building on the NEAR ecosystem.

## Functionality 
The JobBoard smart contract has the following functionalities: 
1. Create a job posting
2. Reply to a job posting
3. Get job postings
4. Get replies to a job posting
5. Remove a job posting and all related replies

All related code for the Jobboard can be found in `src/lib.rs`. 

## Walkthrough 
All of the commands in this walkthrough will be listed below, but can also be foudn in the `commands.sh` file found atthe top level of this repositiory. 

### Logging in and Deploying


To begin, you will need to log into the NEAR testnet using the `near-cli`. Additionally, to make running the subsequent commands easier, you can export your testnet account to a variable:

```sh
near login

export ACCOUNT=<YOUR_TESTNET_ACCOUNT>
```

The web assembly has already been compiled and can be found in this repository; however, if you prefer to build it yourself, run the following command:

```sh
cargo build --target wasm32-unknown-unknown --release 
```

Finally, lets deploy the contract to the NEAR testnet (example output of the command will follow):

```sh
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/job_board_contract.wasm
```

#### Output:

```sh
Starting deployment. Account id: dev-1651855877935-92473091933102, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: target/wasm32-unknown-unknown/release/job_board_contract.wasm
Transaction Id HtUYtGDjZVt1h39drspVfGbmVfxwZK2frVcC56GYpQT4
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/HtUYtGDjZVt1h39drspVfGbmVfxwZK2frVcC56GYpQT4
Done deploying to dev-1651855877935-92473091933102
```

**NOTE**: Do not copy and paste from the output above; your output and contract account ID will be different. Please use the information in the output from your terminal. :) 

For ease of use, you can also export the contract account ID as a variable in your terminal: 

```sh
export CONTRACT=<YOUR_DEV_CONTRACT_ADDRESS>
```

### Initializing and Interacting with the Contract


Before we can begin posting job postings to our contract, we will need to call the new() function of the contract to initialize it:

```sh
near call $CONTRACT new --accountId $ACCOUNT
```

#### Output:

```sh
Scheduling a call: dev-1651855877935-92473091933102.new()
Doing account.functionCall()
Retrying request to broadcast_tx_commit as it has timed out [
  'EQAAAGFsaWdoaWVyaS50ZXN0bmV0ABVbEELRCAvF/Mn/L2brQcg28BlwJhXKikpqYIVMkFiGAcOEOytRAAAgAAAAZGV2LTE2NTE4NTU4Nzc5MzUtOTI0NzMwOTE5MzMxMDJZkVZm53Nps56Vbd0Cg2DV9joNYiBHdr1h5MLtwH88QAEAAAACAwAAAG5ldwIAAAB7fQDgV+tIGwAAAAAAAAAAAAAAAAAAAAAAAACRdz0OHbD1rQ0F6Ymkb7Nw1/fjIDi6P37vAEbmNLmm2nuxhn0FlnlpOZjGxSGj4wy/1KLK7OYYcUJ8FPq9WzwO'
]
Transaction Id Eb7gUb4pSSLYjcnzwJVGwPPzkqe5SZTp86wY5j3gYTkC
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/Eb7gUb4pSSLYjcnzwJVGwPPzkqe5SZTp86wY5j3gYTkC
```

Your output may be slightly different, as the example transaction timed out the first time it tried broadcasting. 

Now that the contract has been intialized, lets post our first job posting:

```sh
near call $CONTRACT add_posting '{"title":"A title", "description":"Hello there, please apply to my job.", "contact":"hello@hello.com"}' --accountId $ACCOUNT 
```

#### Output: 

```sh
Scheduling a call: dev-1651855877935-92473091933102.add_posting({"title":"A title", "description":"Hello there, please apply to my job.", "contact":"hello@hello.com"})
Doing account.functionCall()
Transaction Id CctaK5eaJdzZ1RvqwXMie566m7SkgfwP4Vu8T798rx8Q
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/CctaK5eaJdzZ1RvqwXMie566m7SkgfwP4Vu8T798rx8Q
{
  id: 0,
  title: 'A title',
  description: 'Hello there, please apply to my job.',
  contact: 'hello@hello.com'
}
```

Run this function call again with the same or slightly different values so that we get a second job posting registered on the smart contract. You should see in the output that the ID has now incremented by 1:

```sh
Scheduling a call: dev-1651855877935-92473091933102.add_posting({"title":"A title 2", "description":"Hello there, please apply to my job, PLEASE.", "contact":"hello@hello.com"})
Doing account.functionCall()
Transaction Id 3ja45YynGZnb67JH29v5Ay198piPEjWASbKQbNZHT8Bd
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/3ja45YynGZnb67JH29v5Ay198piPEjWASbKQbNZHT8Bd
{
  id: 1,  ## <-- increased to 1 from 0 
  title: 'A title 2',
  description: 'Hello there, please apply to my job, PLEASE.',
  contact: 'hello@hello.com'
}
```

Now that we have some postings registered with the smart contract, let's retrieve their data: 

```sh
near call $CONTRACT get_postings '{"from_index":0, "limit":5}' --accountId $ACCOUNT
```

#### Output

```sh
Scheduling a call: dev-1651855877935-92473091933102.get_postings({"from_index":0, "limit":5})
Doing account.functionCall()
Transaction Id 4PxRr8BzJ3VVC4tP1JgmpMDDg3b9N92zfCVPnVy5EzGS
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/4PxRr8BzJ3VVC4tP1JgmpMDDg3b9N92zfCVPnVy5EzGS
[
  [
    0,
    {
      id: 0,
      title: 'A title',
      description: 'Hello there, please apply to my job.',
      contact: 'hello@hello.com'
    }
  ],
  [
    1,
    {
      id: 1,
      title: 'A title 2',
      description: 'Hello there, please apply to my job, PLEASE.',
      contact: 'hello@hello.com'
    }
  ]
]
```

Cool! we can see the two job postings we just registered and see that they are returned in an array of objects. 

Now, let's add a reply to both our job postings (note the change in the `posting_id` key in the function calls below):

```sh
near call $CONTRACT add_reply '{"github":"github.com/me", "description":"I am a great Rust dev. I think you should hire me :)", "contact":"me@me.com", "posting_id":0}' --accountId $ACCOUNT

near call $CONTRACT add_reply '{"github":"github.com/me", "description":"I am a great Rust dev. I think you should hire me :)", "contact":"me@me.com", "posting_id":1}' --accountId $ACCOUNT
```

#### Output:

```sh
Scheduling a call: dev-1651855877935-92473091933102.add_reply({"github":"github.com/me", "description":"I am a great Rust dev. I think you should hire me :)", "contact":"me@me.com", "posting_id":0})
Doing account.functionCall()
Transaction Id CXCDaRoYLqukK9BZMBV1NUmGU3ij4b8W5v9np3ZYSDfT
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/CXCDaRoYLqukK9BZMBV1NUmGU3ij4b8W5v9np3ZYSDfT
{
  github: 'github.com/me',
  description: 'I am a great Rust dev. I think you should hire me :)',
  contact: 'me@me.com'
}

Scheduling a call: dev-1651855877935-92473091933102.add_reply({"github":"github.com/me", "description":"I am a great Rust dev. I think you should hire me :)", "contact":"me@me.com", "posting_id":1})
Doing account.functionCall()
Transaction Id DBUYRx7WkrRArXjYtfZ58fNg7HUngF3Z6Pwextr4et56
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/DBUYRx7WkrRArXjYtfZ58fNg7HUngF3Z6Pwextr4et56
{
  github: 'github.com/me',
  description: 'I am a great Rust dev. I think you should hire me :)',
  contact: 'me@me.com'
}

```

Now, lets retrieve the reply we submitted to the first job posting with `posting_id` of 0:

```sh
near call $CONTRACT get_postings_replies '{"posting_id":0}' --accountId $ACCOUNT
```

#### Output:

```sh
Scheduling a call: dev-1651855877935-92473091933102.get_postings_replies({"posting_id":0})
Doing account.functionCall()
Transaction Id 64mnArkZZD5F1v8pz5y1BXodBj1jKG3pqVQvgTNhQ5VN
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/64mnArkZZD5F1v8pz5y1BXodBj1jKG3pqVQvgTNhQ5VN
[
  {
    github: 'github.com/me',
    description: 'I am a great Rust dev. I think you should hire me :)',
    contact: 'me@me.com'
  }
]
```

Let's imagine that we have found an ideal candidate for our first job posting, and as such we want to remove it along with any related replies from the JobBoard smart contract:

```sh
❯ near call $CONTRACT remove_posting '{"posting_id":0}' --accountId $ACCOUNT

```

#### Output:

```sh
Scheduling a call: dev-1651855877935-92473091933102.remove_posting({"posting_id":0})
Doing account.functionCall()
Transaction Id 4ZvT1ZWuVGRoU8WfzFDKewKA2kAoQbk8Xvbu2yvioy82
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/4ZvT1ZWuVGRoU8WfzFDKewKA2kAoQbk8Xvbu2yvioy82
{
  id: 0,
  title: 'A title',
  description: 'Hello there, please apply to my job.',
  contact: 'hello@hello.com'
}
```

Awesome! Assuming you are logged into the same account that you deployed the contract with, you should be successful in removing the post and should see the below output when you run the `get_postings()` function call again:

#### Output

```sh
Scheduling a call: dev-1651855877935-92473091933102.get_postings({"from_index":0, "limit":5})
Doing account.functionCall()
Transaction Id AP6ou6pcdSPD2wBfmdG6MMLdP22DWShcbg9PCK7prKFk
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/AP6ou6pcdSPD2wBfmdG6MMLdP22DWShcbg9PCK7prKFk
[
  [
    1,
    {
      id: 1,
      title: 'A title 2',
      description: 'Hello there, please apply to my job, PLEASE.',
      contact: 'hello@hello.com'
    }
  ]
]
```

### Conclusion and Next Steps

We have tested out the entire functionality of the contract. Here are a few things you can try on your own:

  - Call the `get_postings_replies()` function on the `posting_id` of the job post we removed above. You should see that the output now returns an empty array, as the replies for that posting were also deleted when the posting was deleted.
  - After creating a job posting, log in to another account, and try to call the `remove_posting()` function on the freshly created job posting. the contract should panic, because only the account that creates the job posting is allowed to delete the job posting. 


An interesting iteration on this contract would be to add functionality such that only individual accounts that are NEAR Certified Developers can see or interact with a particular job posting. 