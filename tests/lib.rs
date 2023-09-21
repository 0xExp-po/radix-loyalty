use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;
use general_kit::member::Tasks;

#[test]
fn test_hello() {
    // Setup the environment
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    // Create an account
    let (public_key, _private_key, account) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    // Test the `instantiate_hello` function.
    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "Member",
            "instantiate_member",
            manifest_args!(),
        )
        .build();
        
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    println!("{:?}\n", receipt);
    let component = receipt.expect_commit(true).new_component_addresses()[0];
    let member_nft_resource_address = receipt.expect_commit(true).new_resource_addresses()[2];
    
    println!("{:?}\n", member_nft_resource_address);
    receipt.expect_commit_success();

    // Test the `free_token` method.
    // let manifest = ManifestBuilder::new()
    //     .call_method(component, "free_token", manifest_args!())
    //     .call_method(
    //         account,
    //         "deposit_batch",
    //         manifest_args!(ManifestExpression::EntireWorktop),
    //     )
    //     .build();
    // let receipt = test_runner.execute_manifest_ignoring_fee(
    //     manifest,
    //     vec![NonFungibleGlobalId::from_public_key(&public_key)],
    // );

    // println!("{:?}\n", receipt);
    // receipt.expect_commit_success();

    let manifest2 = ManifestBuilder::new()
    .create_proof_from_account_of_amount(
        account,
        member_nft_resource_address,
        dec!(1)
    );

    let object_names  = manifest2.object_names();

    dump_manifest_to_file_system( 
        object_names,
        &manifest2.build(), 
        "./transaction-manifest", 
        Some("create_proof_from_account_of_amount"),
        &NetworkDefinition::simulator()
    ).err(); 


}

// #[test]
// fn test_mint_member_card() {
//     let mut test_runner = TestRunner::builder().without_trace().build();

//     // Create an account
//     let (public_key, _private_key, account) = test_runner.new_allocated_account();

//     // Publish package
//     let package_address = test_runner.compile_and_publish(this_package!());

//     // Test the `instantiate_hello` function.
//     let manifest = ManifestBuilder::new()
//         .call_function(
//             package_address,
//             "Member",
//             "instantiate_member",
//             manifest_args!(),
//         )
//         .build();

//     let receipt = test_runner.execute_manifest_ignoring_fee(
//         manifest,
//         vec![NonFungibleGlobalId::from_public_key(&public_key)],
//     );

//     println!("{:?}\n", receipt);
//     let component = receipt.expect_commit(true).new_component_addresses()[0];
//     println!("{:?}\n", component);

//     // Test the `mint_member_card` method.
//     let manifest = ManifestBuilder::new()
//         .call_method(component, "mint_member_card", (Tasks::AttendEvent("my_nice_event".to_owned(), 413u32),))
//         .try_deposit_batch_or_abort(account);

//     let object_names  = manifest.object_names();

//     // let receipt = test_runner.execute_manifest_ignoring_fee(
//     //     manifest,
//     //     vec![NonFungibleGlobalId::from_public_key(&public_key)],
//     // );
//     // println!("{:?}\n", receipt);
//     // receipt.expect_commit_success();
// }

// #[test]
// fn get_reward_for_task_2() {
//     let mut test_runner = TestRunner::builder().without_trace().build();

//     // Create an account
//     let (public_key, _private_key, account) = test_runner.new_allocated_account();

//     // Publish package
//     let package_address = test_runner.compile_and_publish(this_package!());

//     // Test the `instantiate_hello` function.
//     let manifest = ManifestBuilder::new()
//         .call_function(
//             package_address,
//             "Member",
//             "instantiate_member",
//             manifest_args!(),
//         )
//         .build();

//     let receipt = test_runner.execute_manifest_ignoring_fee(
//         manifest,
//         vec![NonFungibleGlobalId::from_public_key(&public_key)],
//     );

//     println!("{:?}\n", receipt);
//     let component = receipt.expect_commit(true).new_component_addresses()[0];
//     println!("{:?}\n", component);

//     // Test the `mint_member_card` method.
//     let manifest = ManifestBuilder::new()
//         .call_method(component, "get_reward_for_task_2", manifest_args!())
//         .try_deposit_batch_or_abort(account, false, None)
//         .build();

//     let receipt = test_runner.execute_manifest_ignoring_fee(
//         manifest,
//         vec![NonFungibleGlobalId::from_public_key(&public_key)],
//     );
//     println!("{:?}\n", receipt);
//     receipt.expect_commit_success();
// }