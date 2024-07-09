use ic_cdk::api::call::call;
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;

mod lib;
use crate::lib::Token;

type Address = String;
type Balance = u64;

static mut TOKENS: Option<HashMap<Address, Token>> = None;

#[init]
fn init() {
    unsafe {
        TOKENS = Some(HashMap::new());
    }
}

#[update]
async fn send_token(to: Address, amount: Balance) {
    let from = ic_cdk::caller();
    unsafe {
        let tokens = TOKENS.as_mut().unwrap();
        if let Some(sender) = tokens.get_mut(&from.to_string()) {
            if sender.balance >= amount {
                sender.debit(amount);
                let receiver = tokens.entry(to).or_insert(Token::new("IRCRC2".to_string()));
                receiver.credit(amount);
            }
        }
    }
}

#[query]
fn get_balance(address: Address) -> Balance {
    unsafe {
        let tokens = TOKENS.as_ref().unwrap();
        if let Some(token) = tokens.get(&address) {
            return token.balance;
        }
        0
    }
}

#[update]
async fn receive_token(from: Address, amount: Balance) {
    let to = ic_cdk::caller();
    unsafe {
        let tokens = TOKENS.as_mut().unwrap();
        let sender = tokens.entry(from.clone()).or_insert(Token::new("IRCRC2".to_string()));
        if sender.balance >= amount {
            sender.debit(amount);
            let receiver = tokens.entry(to).or_insert(Token::new("IRCRC2".to_string()));
            receiver.credit(amount);
        }
    }
}
