use scrypto::prelude::*;


#[blueprint]
mod organization {
    struct Organization {
        reward_token_address: ResourceAddress,
    }
    impl Organization {

        // Call instantiate_member to create a new member (will have 1 membership card and 0 rewards)
        // pub fn instantiate_organization() -> Global<Organization> {
        //     // Create resource representing membership card, to be minted by user. Maximum 1 can be minted per account.
        //     let reward_token: Bucket = ResourceBuilder::new_fungible()
        //     // .divisibility(DIVISIBILITY_MAXIMUM)
        //     // .metadata("name", format!("{} LP Tracking Token", pair_name))
        //     // .metadata("symbol", "TT")
        //     // .metadata("description", "A tracking token used to track the percentage ownership of liquidity providers over the liquidity pool")
        //     // .metadata("lp_id", format!("{}", lp_id))
        //     // .mintable(rule!(require(tracking_token_admin_badge.resource_address())), LOCKED)
        //     // .burnable(rule!(require(tracking_token_admin_badge.resource_address())), LOCKED)
        //     // .c;


        //     // // Instantiate a Member component with member card and empty rewards bucket
        //     // Self {
        //     //     card_vault: Vault::with_bucket(my_bucket),
        //     // }
        //     // .instantiate()
        //     // .prepare_to_globalize(OwnerRole::None)
        //     // .globalize()
        // }
    }
}