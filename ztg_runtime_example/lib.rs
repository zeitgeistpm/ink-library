#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ztg_runtime_example {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct ZtgRuntimeExample {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl ZtgRuntimeExample {
        #[ink(constructor)]
        pub fn new(_value: bool) -> Self {
            ZtgRuntimeExample { value: _value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
        
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {

    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // // Given
            // let constructor = ZtgRuntimeExampleRef::default();

            // // When
            // let contract_account_id = client
            //     .instantiate("ztg_runtime_example", &ink_e2e::alice(), constructor, 0, None)
            //     .await
            //     .expect("instantiate failed")
            //     .account_id;

            // // Then
            // let get = build_message::<ZtgRuntimeExampleRef>(contract_account_id.clone())
            //     .call(|ztg_runtime_example| ztg_runtime_example.get());
            // let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            // assert!(matches!(get_result.return_value(), false));

            Ok(())
        }
    }
}