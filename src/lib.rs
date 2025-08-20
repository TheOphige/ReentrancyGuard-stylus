use stylus_sdk::prelude::*;
use crate::reentrancy::{ReentrancyGuard, ReentrancyError};

#[entrypoint]
#[storage]
pub struct MyContract {
    guard: ReentrancyGuard,
    balance: u128,
}

impl MyContract {
    #[constructor]
    pub fn constructor(&mut self) {
        self.guard.init();
        self.balance = 0;
    }

    #[public]
    pub fn deposit(&mut self, amount: u128) {
        self.balance += amount;
    }

    #[public]
    pub fn safe_withdraw(&mut self, amount: u128) -> Result<(), ReentrancyError> {
        self.guard.non_reentrant(|| {
            if self.balance < amount {
                return Err(ReentrancyError::ReentrantCall);
            }
            self.balance -= amount;
            Ok(())
        })
    }

    #[public]
    pub fn unsafe_withdraw(&mut self, amount: u128) {
        if self.balance < amount {
            return;
        }
        self.balance -= amount;
        // Vulnerable to reentrancy
    }
}