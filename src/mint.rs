use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        finite_state: TokenFiniteState
    ) {
        //measure the initial storage being used on the contract
        //let initial_storage_usage = env::storage_usage();

        //specify the token struct that contains the owner ID 
        let mut token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //set the finite state to the passed finite state
            finite_state: finite_state,
        };
        
        token.finite_state.testing_complete_date = "_".to_string();
        token.finite_state.inventoried_date = "_".to_string();
        token.finite_state.shipped_date = "_".to_string();
        token.finite_state.transfused_date = "_".to_string();
        token.finite_state.disposal_date = "_".to_string();

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        //calculate the required storage which was the used - initial
        //let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        //refund_deposit(required_storage_in_bytes);
    }
}