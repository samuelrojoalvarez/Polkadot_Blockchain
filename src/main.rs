mod balances;
mod system;
mod support;
mod proof_of_existence;

use balances::Call;
use support::Block;
use types::{Extrinsic, Header};

use crate::support::Dispatch;
use crate::support::DispatchResult;


mod types {
    use crate::support;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
    proof_of_existence: proof_of_existence::Pallet<Runtime>,
}

fn main() {
    let mut runtime = Runtime::new();
    let alice: String = "alice".to_string();
    let bob: String = "bob".to_string();
    let charli: String = "charli".to_string();

    runtime.balances.set_balance(alice.clone(), 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer { to: bob.clone(), amount: (30) }),
            },
        
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer { to: charli.clone(), amount: (20) }),
            },
        ],

    };

    runtime.execute_block(block_1).expect("wrong block execution");

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "my_document" }),
            },
        
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "bobs_document" }),
            },
        ],

    };

    runtime.execute_block(block_2).expect("wrong block execution");

    println!("{:#?}", runtime);
}
