//! # Factom Entry System
//!
//! This is the Factom entry system. This is where entry data is stored,
//! paid for, and retrieved. It is important to note that external ids and
//! entry content are stored as `Vec<u8>`. For now, the client is responsible
//! for performing the Vec<u8> enoding; it is possible to accept a string
//! and perform the encoding here but that hasn't been tried in the PoC or here.
use crate::entry_credit;
use parity_codec::{Decode, Encode};
use rstd::vec::Vec;
use runtime_primitives::traits::Hash;
use support::{decl_module, decl_storage, dispatch::Result, ensure, StorageMap};
use system::ensure_signed;

pub trait Trait: system::Trait + entry_credit::Trait {}

/// Entry Data
///
/// This is the form of entry data as will be stored on chain. Each entry
/// will have its own hash which we can query. That query will also have
/// the chain id along with it.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct EntryStruct<Hash> {
    content: Vec<u8>,      // Content of arbitrary kind converted to Vec<u8>
    external_ids: Vec<u8>, // Collection of ext ids converted to Vec<u8>
    chain_id: Hash,        // Chain ID is a hash, stored along with entry
}

// Current design will store entry data as an association of the chain id, u64 to
// the entry struct. This association is useful for future new entries, as well as
// retreiving entries from storage later. We must manage, however, the total entries
// that a hash has accumulated separately.
decl_storage! {
    trait Store for Module<T: Trait> as Entry {
        EntryData get(entry_data): map (T::Hash, u64) => EntryStruct<T::Hash>;
        TotalEntries get(total_entries): map T::Hash => u64;
    }
}

/// Validate Entry Data
///
/// 1. Combined sizes do no exceed 1kb. We can get more specific or add
/// additional fees for number of ext_ids, for example, but for now a
/// check of the total combined lengths should be in the same neighborhood.
fn validate_entry_data(content: &Vec<u8>, ext_ids: &Vec<u8>) -> Result {
    if content.len() + ext_ids.len() <= 1024 {
        Ok(())
    } else {
        Err("EntryTooLarge")
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Put Entry
        ///
        /// The user has a chain id to use and will be providing a new entry. Will spend
        /// from free balance of entry credits.
        ///
        /// While entry and ext id validation steps occur in functions outside decl_module,
        /// the chain id validation will occur here. This is because right now we will check
        /// that that chain id is indeed a Blake2 hash (we get that for free) and we will
        /// also check that the hash already exists in storage.
        fn put_entry (origin, content: Vec<u8>, external_ids: Vec<u8>, chain_id: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;
            let _ = validate_entry_data(&content, &external_ids)?;

            ensure!(<EntryData<T>>::exists((chain_id, 1)), "This chain does not exist.");

            let entries_total = Self::total_entries(chain_id);
            let incr_entries_total = entries_total.checked_add(1).ok_or("Overflow entries total!")?;

            let new_entry = EntryStruct {
                content: content,
                external_ids: external_ids,
                chain_id: chain_id
            };

            <entry_credit::Module<T>>::spend_entry_credits(sender.clone(), 1)?;
            <TotalEntries<T>>::insert(chain_id, incr_entries_total);
            <EntryData<T>>::insert((chain_id, incr_entries_total), new_entry);

            Ok(())
        }

        /// Put Chain
        ///
        /// The user does not have a chain id and wants to add an entry. This function
        /// will provide a new chain id for this entry. Will spend from free balance of
        /// entry credits.
        fn put_chain (origin, content: Vec<u8>, external_ids: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;
            let _ = validate_entry_data(&content, &external_ids)?;
            let chain_id = <T as system::Trait>::Hashing::hash(&external_ids);

            ensure!(!<EntryData<T>>::exists((chain_id, 1)), "This chain already exists.");

            let new_entry = EntryStruct {
                content: content,
                external_ids: external_ids,
                chain_id: chain_id
            };

            <entry_credit::Module<T>>::spend_entry_credits(sender.clone(), 2)?;
            <TotalEntries<T>>::insert(chain_id, 1);
            <EntryData<T>>::insert((chain_id, 1), new_entry);
            Ok(())
        }

    }
}
