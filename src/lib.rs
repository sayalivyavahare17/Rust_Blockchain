use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct Token {
    pub symbol: String,
    pub balance: u64,
}

impl Token {
    pub fn new(symbol: String) -> Self {
        Token { symbol, balance: 0 }
    }

    pub fn credit(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn debit(&mut self, amount: u64) {
        if self.balance >= amount {
            self.balance -= amount;
        }
    }
}
