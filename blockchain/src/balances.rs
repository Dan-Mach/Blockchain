use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};
pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,

}


#[macros::call]
impl <T: Config> Pallet<T>{
    pub fn transfer(&mut self, caller: T::AccountId, to: T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
    
        let to_balance = self.balance(&to);
        let new_caller_balance = caller_balance 
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow in recipient balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}


impl <T: Config> Pallet<T>
    {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);   
    }
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
    
}

#[cfg(test)]
mod tests {
    use crate::system;
    struct TestConfig;
    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
        
    }
    impl super::Config for TestConfig {
        type Balance = u128;
    }
    #[test]
    fn init_balances() {
        let mut balaances: super::Pallet<TestConfig> = super::Pallet::new();

        assert_eq!(balaances.balance(&"Alice".to_string()), 0);

        balaances.set_balance(&"Alice".to_string(), 100);
        assert_eq!(balaances.balance(&"Alice".to_string()), 100);
        assert_eq!(balaances.balance(&"Bob".to_string()), 0);
    }
    #[test]
    fn transfer_balance() {
        let Alice = "Alice".to_string();
        let Bob = "Bob".to_string();

        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);

        let _ = balances.transfer(Alice.clone(), Bob.clone(), 90);
        
        assert_eq!(balances.balance(&Alice), 10);
        assert_eq!(balances.balance(&Bob), 90);

    }
    #[test]
    fn transfer_balance_insufficient(){
        let Alice = "Alice".to_string();
        let Bob = "Bob".to_string();

        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);

        let result = balances.transfer(Alice.clone(), Bob.clone(), 190);
        
        assert_eq!(result, Err("Insufficient balance"));
        assert_eq!(balances.balance(&Alice), 100);
        assert_eq!(balances.balance(&Bob), 0);
    }
    #[test]
    fn transfer_balance_overflow(){
        let Alice = "Alice".to_string();
        let Bob = "Bob".to_string();
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), u128::MAX);

        let result = balances.transfer(Alice.clone(), Bob.clone(), 1);
        assert_eq!(result, Err("Overflow in recipient balance"));
        assert_eq!(balances.balance(&Alice), 100);
        assert_eq!(balances.balance(&Bob), u128::MAX);   
    
    }
}