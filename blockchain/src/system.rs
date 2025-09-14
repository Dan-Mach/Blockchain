use std::{collections::BTreeMap, ops::AddAssign};

use num::{CheckedAdd, CheckedSub, One, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber , Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,

}
impl <AccountId, BlockNumber , Nonce> Pallet<AccountId, BlockNumber , Nonce>
where
    AccountId: Ord + Clone,
    BlockNumber: Zero + One + CheckedAdd + CheckedSub + Copy + AddAssign,
    Nonce: Ord + Clone + Copy + Zero + One,
    {    
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }
    pub fn inc_block_number(&mut self) {
        self.block_number += BlockNumber::one();
    }
    pub fn inc_nonce(&mut self, who: &AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
        self.nonce.insert(who.clone(), nonce + Nonce::one());
    }
    pub fn get_nonce(&self, who: &AccountId) -> Nonce {
        *self.nonce.get(who).unwrap_or(&Nonce::zero())
    }

}
#[cfg(test)]
mod test {
    use crate::system::Pallet;

    #[test]
    fn init_system() {
        let mut system:Pallet<String, u32, u32> = super::Pallet::new();         
        assert_eq!(system.block_number(), 0);
    }  
    #[test]
    fn inc_block_number(){
        let mut system:Pallet<String, u32, u32> = super::Pallet::new();           
        system.inc_block_number();
        assert_eq!(system.block_number(), 1);
    }
    #[test]
    fn inc_nonce(){
        let Alice = &"Alice".to_string();
        let mut system:Pallet<String, u32, u32> = super::Pallet::new();           
        system.inc_nonce(&Alice.clone());
        system.inc_nonce(&Alice.clone());
        assert_eq!(system.get_nonce(Alice), 2);
    }
}


