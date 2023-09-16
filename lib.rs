#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]
        

#[openbrush::contract]
pub mod my_psp34 {
    /** access-control */
	use openbrush::contracts::access_control::only_role;
	use openbrush::contracts::access_control::extensions::enumerable::*;
    /** psp34 :: extensions */
	use openbrush::contracts::psp34::extensions::burnable::*;
	use openbrush::contracts::psp34::extensions::mintable::*;
	use openbrush::contracts::psp34::extensions::enumerable::*;
	use openbrush::contracts::psp34::extensions::metadata::*;
    /** extra things that willbe needed */
	use openbrush::traits::{DefaultEnv, Storage, String};
    use ink::codegen::{EmitEvent, Env};
    use ink::prelude::vec::Vec;
    use openbrush::contracts::psp34::balances::BalancesManager;

    
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
    	#[storage_field]
		psp34: psp34::Data<Balances>,
		#[storage_field]
		access: access_control::Data<Members>,
		#[storage_field]
		metadata: metadata::Data,
        #[storage_field]
        _data : metaData
    }

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(metaData);

	#[openbrush::upgradeable_storage(STORAGE_KEY)]
	#[derive(Default, Debug)]
	pub struct metaData {
		//for upgradable storage
	}

	const MANAGER: RoleType = ink::selector_id!("MANAGER");
    
    // Section contains default implementation without any modifications
	
	
	impl AccessControlEnumerable for Contract {
        //get_role_member
        //get_role_member_count
    }

    impl PSP34Enumerable for Contract {
        // balance_of
        // increase_balance
        // decrease_balance
        // total_supply
    }

	impl PSP34Metadata for Contract {
        // get_attribute
    }

	impl PSP34Burnable for Contract {
		#[ink(message)]
		#[openbrush::modifiers(only_role(MANAGER))]
		fn burn(
            &mut self,
            account: AccountId,
			id: Id
        ) -> Result<(), PSP34Error> {
			self._burn_from(account, id)
		}
	}

	impl PSP34Mintable for Contract {
		#[ink(message)]
		#[openbrush::modifiers(only_role(MANAGER))]
		fn mint(
            &mut self,
            account: AccountId,
			id: Id
        ) -> Result<(), PSP34Error> {
			self._mint_to(account, id)
		}
	}

	impl access_control::Internal for Contract {
        //implement events
        // fn _emit_role_admin_changed(&mut self, _role: RoleType, _previous: RoleType, _new: RoleType);
        // fn _emit_role_granted(&mut self, _role: RoleType, _grantee: AccountId, _grantor: Option<AccountId>);
        // fn _emit_role_revoked(&mut self, _role: RoleType, _account: AccountId, _sender: AccountId);
        /*
         *  _default_admin,
         * _init_with_caller,
         * _init_with_admin,
         * _setup_role,
         * _do_revoke_role,
         * _set_role_admin 
         * */
    }

    impl AccessControl for Contract {
        /*
         * has_role,
         * get_role_admin,
         * grant_role
         * revoke_role
         * renounce_role
         */
    }

    impl psp34::Internal for Contract {
        // fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id);
        // fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Option<Id>, _approved: bool);
        /*
         * _approve_for,
         * _owner_of
         * _transfer_token
         * _do_safe_transfer_check
         * _mint_to
         * _burn_from
         * _allowance
         * _check_token_exists
         */
    }

    impl PSP34 for Contract {
        /*
         * collection_id,
         * balance_of,
         * owner_of,
         * allowance,
         * approve,
         * transfer,
         * total_supply
         */
    }

    impl metadata :: Internal for Contract {
        // fn _emit_attribute_set_event(&self, _id: Id, _key: Vec<u8>, _data: Vec<u8>);

        // fn _set_attribute(&mut self, id: Id, key: Vec<u8>, value: Vec<u8>);
    }
    
    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			_instance.grant_role(MANAGER, _instance.env().caller()).expect("Should grant MANAGER role");
			_instance.grant_role(MANAGER, _instance.env().caller()).expect("Should grant MANAGER role");
			let collection_id = _instance.collection_id();
			_instance._set_attribute(collection_id.clone(), String::from("name"), String::from("MyPSP34"));
			_instance._set_attribute(collection_id, String::from("symbol"), String::from("MPSP"));
            _instance
        }

        #[ink(message)]
        #[openbrush::modifiers(only_role(MANAGER))]
        pub fn set_code(&mut self, code_hash: [u8; 32]) -> Result<(), PSP34Error> {
            ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });
            Ok(())
        }
    }
}