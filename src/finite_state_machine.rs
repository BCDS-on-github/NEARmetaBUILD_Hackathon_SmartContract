use crate::*;
/*
pub trait NonFungibleTokenCore {
    //transition token state to a valid state
    fn nft_transition_finite_state(&mut self, token_id: TokenId, transition: String);
}
*/
#[near_bindgen]
impl Contract { 
    #[payable]
    pub fn nft_transition_finite_state(
        &mut self,
        token_id: TokenId,
        transition: String,
        datetime: String,
    ) {
        //get the token object from the token ID
        let mut token = self.tokens_by_id.get(&token_id).expect("No token");

        //there is only one valid transition for each possible state
        if transition == "next" {
        	if token.finite_state.testing_complete_date == "_" {
        		token.finite_state.testing_complete_date = datetime;
        	} else if token.finite_state.inventoried_date == "_" {
        		token.finite_state.inventoried_date = datetime;
        	} else if token.finite_state.shipped_date == "_" {
        		token.finite_state.shipped_date = datetime;
        	} else if token.finite_state.transfused_date == "_" {
        		token.finite_state.transfused_date = datetime;
        	} else if token.finite_state.disposal_date == "_" {
        		token.finite_state.disposal_date = datetime;
        	}
        }

        //insert the token back into the tokens_by_id collection
        self.tokens_by_id.insert(&token_id, &token);
    }
}