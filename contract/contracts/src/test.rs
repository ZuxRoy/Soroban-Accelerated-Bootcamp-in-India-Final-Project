//use super::*;
//use soroban_sdk::{testutils::Address, Env, String};
//
//#[test]
//fn test_share_and_get_files() {
//    let env = Env::default();
//    let contract_id = env.register_contract(None, FileShare);
//
//    let user = Address::generate(&env);
//
//    let name = String::from("My Document Name");
//    let description = String::from("This is a test document");
//    let details = String::from("Document contents go here");
//
//    let client = FileShareClient::new(&env, &contract_id);
//
//    client.share_files(&user, &name, &description, &details);
//
//    let retrieved_files = client.get_files(&user);
//
//    assert!(retrieved_files.is_some());
//    let retrieved_files = retrieved_files.unwrap();
//
//    assert_eq!(retrieved_files.name, name);
//    assert_eq!(retrieved_files.description, description);
//    assert_eq!(retrieved_files.details, details);
//}
