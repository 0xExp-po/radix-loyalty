use scrypto::prelude::*;

// data within membership cards, can get updated
#[derive(ScryptoSbor, NonFungibleData)]
struct MembershipData{
    // user level, great for gamification and advancing to the next level
    level: String,
    join_date_time: Instant
}

// define task and reward
#[derive(ScryptoSbor,ManifestEncode)]
pub enum Tasks{
    Vote(String, u32),
    AttendEvent(String, u32),
    SayHi(u32)
}

#[blueprint]
mod member {
    enable_method_auth! {
        roles {
            member => updatable_by: [OWNER];
        },
        methods {
            mint_member_card => PUBLIC;
            get_reward_for_task => restrict_to: [member];
            get_reward_for_task_2 => restrict_to: [member];
            get_reward_with_reason => restrict_to: [member];
            deposit_xrd => PUBLIC;
        }
    }

    struct Member {
        // rewards token
        rewards_token_resource_manager: ResourceManager,
        // dream token
        dream_token_resource_manager: ResourceManager,
        // where member's card is held
        member_card_resource_manager: ResourceManager,
        // membership ID counter
        member_id_counter: u64,
        // fee payer
        xrd_fee_vault: Vault,
    }
    impl Member {

        // Call instantiate_member to create a new member (will have 1 membership card and 0 rewards)
        pub fn instantiate_member(owner_badge: ResourceAddress) -> Global<Member> {

            let (address_reservation, component_address) =
            Runtime::allocate_component_address(Member::blueprint_id()); 

            let rewards_token_resource_manager: ResourceManager = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(DIVISIBILITY_NONE)
            .metadata(metadata!(
                init {
                    "name" => "Reward_Token".to_owned(), locked;
                    "symbol" => "REW".to_owned(), locked;
                    "description" => "Rewards for activity".to_owned(), locked;
                }
            ))
            .mint_roles(mint_roles! {
                minter => rule!(require(global_caller(component_address))); 
                minter_updater => rule!(deny_all);
            })
            .create_with_no_initial_supply();

            let dream_token_resource_manager: ResourceManager = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(DIVISIBILITY_NONE)
            .metadata(metadata!(
                init {
                    "name" => "Dream_Token".to_owned(), locked;
                    "symbol" => "DRM".to_owned(), locked;
                    "description" => "Dream tokens are for big dreams".to_owned(), locked;
                }
            ))
            .mint_roles(mint_roles! {
                minter => rule!(require(global_caller(component_address))); 
                minter_updater => rule!(deny_all);
            })
            .create_with_no_initial_supply();

            // Create resource representing membership card, to be minted by user. Maximum 1 can be minted per account.
            // TODO: chagne RUID after wallet is fixed
            // let member_card_resource_manager: ResourceManager = ResourceBuilder::new_ruid_non_fungible::<MembershipData>(OwnerRole::None)
            let member_card_resource_manager: ResourceManager = ResourceBuilder::new_integer_non_fungible::<MembershipData>(OwnerRole::None)
                 .metadata(metadata! {
                    init {
                        "name" => "Member Card", locked;
                        "symbol" => "MEMBER_CARD", locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(require(global_caller(component_address))); 
                    minter_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            // Instantiate a Member component with member card and empty rewards bucket
            Self {
                rewards_token_resource_manager,
                dream_token_resource_manager,
                member_card_resource_manager,
                member_id_counter: 0u64,
                xrd_fee_vault: Vault::new(XRD)
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed( 
                    rule!(require(owner_badge))
                )
            )
            .roles(
                roles!(
                    member => rule!(require(member_card_resource_manager.address())); 
                )
            )
            .with_address(address_reservation) 
            .globalize()
        }

        /**
         * TODO: to use in place of ruid based NFTs (see below)
         */
        pub fn mint_member_card(&mut self) -> Bucket {
            self.member_id_counter += 1;
            // todo: if account already has member card error out
            let new_card_data = MembershipData { level: "0".to_owned(), join_date_time: Clock::current_time_rounded_to_minutes()};
            let member_card = self.member_card_resource_manager
                                        .mint_non_fungible(&NonFungibleLocalId::integer(self.member_id_counter),new_card_data);

            member_card
        }

        /**
         * // TODO: when RUID is fixed with wallet use this
         * Only an account without an existing membership card can mint a new one
         */
        // pub fn mint_member_card(&self) -> Bucket {
        //     // todo: if account already has member card error out
        //     let new_card_data = MembershipData { level: "Silver".to_owned() };
        //     let member_card = self.member_card_resource_manager.mint_ruid_non_fungible(new_card_data);

        //     member_card
        // }

        /**
         * Complete task and get the associated award
         * Deprecated
         */
        pub fn get_reward_for_task(&self, task: Tasks) -> Bucket{
            let rewards: Bucket;
            match task {
                Tasks::Vote(poll_name, reward_amount) => {
                    println!("Member voted for poll: {}, collect {} pts.", poll_name, reward_amount);
                    rewards = self.rewards_token_resource_manager.mint(reward_amount);
                }
                Tasks::AttendEvent(event_name, reward_amount) => {
                    println!("Member attended event: {}, collect {} pts.", event_name, reward_amount);
                    rewards = self.rewards_token_resource_manager.mint(reward_amount);
                }
                Tasks::SayHi(reward_amount) => {
                    println!("Member said hi!, collect {} pts.", reward_amount);
                    rewards = self.rewards_token_resource_manager.mint(reward_amount)
                }
            }

            rewards
        }

        pub fn get_reward_for_task_2(&self) -> (Bucket, Bucket){
            let rewards: Bucket = self.rewards_token_resource_manager.mint(13);
            let dream: Bucket = self.dream_token_resource_manager.mint(7);
            (rewards, dream)
        }

        /**
         * get reward on task based on reward amount and description
         * gas sponsorship available if user has not consumed more than gas amount threshold
         * @Returns rewards bucket
         */
        pub fn get_reward_with_reason(&mut self, reward_amount: u64, reason: String) -> Bucket{
            // self.xrd_fee_vault.lock;
            println!("Member said hi!, collect reward_amount: {} for reason: {}.", reward_amount, reason);
            let rewards: Bucket = self.rewards_token_resource_manager.mint(reward_amount);
            rewards
        }

        pub fn deposit_xrd(&mut self, xrd_bucket: Bucket) -> () {
            self.xrd_fee_vault.put(xrd_bucket);
        }
    }
}