//! ## Entry Credits
//!
//! Factom's entry credits allow users to add entries to Factom. These credits
//! are purchased with Factoids and are burned when used to pay for an entry.
//!
//! The conversion for FCT -> EC is determined by an oracle. This is to ensure that
//! ECs are consistently priced.
//!
use support::{decl_module, dispatch::Result, traits::Currency, ensure};
use runtime_primitives::traits::{As};

pub trait Trait: balances::Trait<balances::Instance0> {}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
  }}

impl<T: Trait> Module<T> {
        /// Set new entry credit balance
        ///
        /// This is a privileged function that will increase the balance of an account's
        /// entry credits. Imbalance can be ignored, balances module will update total supply
        /// automatically.
        pub fn increase_ec_balance(who: T::AccountId, increase: T::Balance) -> Result {
            let _imbalance = <balances::Module<T, balances::Instance0> as Currency<_>>::deposit_creating(&who, increase);
            Ok(())
        }

        /// Spend entry credits
        ///
        /// This will be called when an entry is purchased by an account.
        pub fn spend_entry_credits(who: T::AccountId, value: u64) -> Result {
            let bal = T::Balance::sa(value);
            // Check for sufficent balance
            ensure!(<balances::Module<T, balances::Instance0> as Currency<_>>::can_slash(&who, bal), "Insufficient Balance");

            let _imbalance = <balances::Module<T, balances::Instance0> as Currency<_>>::slash(&who, bal);
            Ok(())
        }
}
