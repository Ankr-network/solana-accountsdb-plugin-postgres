use {log::*, std::collections::HashSet};

#[derive(Debug)]
pub(crate) struct HashSlots {
    pub from: u16,
    pub to: u16,
}

impl From<&[i64]> for HashSlots {
    fn from(slots: &[i64]) -> HashSlots {
        if slots.len() != 2 {
            panic!("Slots count should be 2")
        }
        let from = slots[0].try_into().unwrap();
        let to = slots[1].try_into().unwrap();
        HashSlots { from, to }
    }
}

impl HashSlots {
    fn contains(&self, account: &[u8]) -> bool {
        let account_slot = u16::from_le_bytes([account[0], account[1]]);
        self.from <= account_slot && account_slot < self.to
    }
}

#[derive(Debug)]
pub(crate) struct AccountsSelector {
    pub accounts: HashSet<Vec<u8>>,
    pub owners: HashSet<Vec<u8>>,
    pub select_all_accounts: bool,
    pub hash_slots: Option<HashSlots>,
}

impl AccountsSelector {
    pub fn default() -> Self {
        AccountsSelector {
            accounts: HashSet::default(),
            owners: HashSet::default(),
            select_all_accounts: true,
            hash_slots: None,
        }
    }

    pub fn new(accounts: &[String], owners: &[String], hash_slots: &[i64]) -> Self {
        info!(
            "Creating AccountsSelector from accounts: {:?}, owners: {:?}, hash slots: {:?}",
            accounts, owners, hash_slots
        );

        let select_all_accounts = accounts.iter().any(|key| key == "*");
        if select_all_accounts {
            return AccountsSelector {
                accounts: HashSet::default(),
                owners: HashSet::default(),
                select_all_accounts,
                hash_slots: None,
            };
        }
        let accounts = accounts
            .iter()
            .map(|key| bs58::decode(key).into_vec().unwrap())
            .collect();
        let owners = owners
            .iter()
            .map(|key| bs58::decode(key).into_vec().unwrap())
            .collect();
        let hash_slots = if hash_slots.len() > 0 {
            Some(HashSlots::from(hash_slots))
        } else {
            None
        };
        AccountsSelector {
            accounts,
            owners,
            select_all_accounts,
            hash_slots,
        }
    }

    pub fn is_account_selected(&self, account: &[u8], owner: &[u8]) -> bool {
        self.hash_slots.as_ref().map_or(false, |slots| slots.contains(account)) ||
        self.select_all_accounts || self.accounts.contains(account) || self.owners.contains(owner)
    }

    /// Check if any account is of interested at all
    pub fn is_enabled(&self) -> bool {
        self.hash_slots.is_some() ||
        self.select_all_accounts || !self.accounts.is_empty() || !self.owners.is_empty()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn test_create_accounts_selector() {
        AccountsSelector::new(
            &["9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin".to_string()],
            &[],
            &[],
        );

        AccountsSelector::new(
            &[],
            &["9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin".to_string()],
            &[],
        );

        AccountsSelector::new(
            &[],
            &[],
            &[42, 300],
        );
    }

    #[test]
    fn test_hash_slots_selector() {
        let empty_selector = AccountsSelector::new(
            &[],
            &[],
            &[],
        );
        assert!(!empty_selector.is_enabled());

        let hash_slot_selector = AccountsSelector::new(
            &[],
            &[],
            &[42, 300]
        );

        assert!(hash_slot_selector.is_enabled());

        let address1 = [12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let address2 = [45, 0, 56, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let address3 = [0, 1, 56, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let address4 = [0, 2, 56, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert!(!hash_slot_selector.is_account_selected(&address1, &[0]), "should not select address before the range");
        assert!(hash_slot_selector.is_account_selected(&address2, &[0]), "should select address inside the range");
        assert!(hash_slot_selector.is_account_selected(&address3, &[0]), "should select address inside the range");
        assert!(!hash_slot_selector.is_account_selected(&address4, &[0]), "should not select address after the range");
    }
}
