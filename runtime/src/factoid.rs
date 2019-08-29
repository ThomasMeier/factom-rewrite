//! ## Factoids
//!
//! Factom's tradeable currency. This is used to reward validators and
//! converted to Entry Credits to, in turn, buy entries on Factom.
//!
use crate::EntryCredits;
use primitives::sr25519::Public;
use runtime_primitives::traits::As;
use support::{decl_module, dispatch::Result, traits::Currency};
use system::ensure_signed;

/// Module config
/// TODO Include events, tom
pub trait Trait: balances::Trait {}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    /// Transfer factoids
    ///
    /// Send factoids from one address to another address. Right now there is not
    /// an imposed transfer fee.
    fn transfer_factoids(origin, to: T::AccountId, value: T::Balance) -> Result {
        let sender = ensure_signed(origin)?;

        // Simple transfer
        <balances::Module<T> as Currency<_>>::transfer(&sender, &to, value)?;

        Ok(())
    }

    /// Convert factoids to entry credits
    ///
    /// In order to write entries to Factom, one must have entry credits. To obtain
    /// entry credits, you must convert some of your Factoid balance to Entry Credits.
    ///
    /// The price oracle will be added later. For now, 1 FCT = 1 EC.
    ///
    /// Factoids are slashed from the account upon purchase. Imbalance is ignored.
    /// Presently, the result of the increase is also ignored until we implement
    /// Events for these modules.
    fn buy_entry_credits(origin, to_ec_addr: Public, value: T::Balance) -> Result {
        let sender = ensure_signed(origin)?;

        // Check if there is enough free balance for requested exchange
        let can_slash = <balances::Module<T> as Currency<_>>::can_slash(&sender, value);

        if can_slash {
            // remove requested amounmt
            let _imbalance = <balances::Module<T> as Currency<_>>::slash(&sender, value);
            // Conversion necessary from Balance to rust primitive
            let value_as = value.as_();
            let _increased_ec = <EntryCredits>::increase_ec_balance(to_ec_addr, value_as.into());
        }

        Ok(())
    }

}
}
