#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, vec, Env};

#[test]
fn test_check_auth() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let user = Address::generate(&env);
    env.mock_all_auths();
    
    assert!(client.check_auth(&user));
}

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    env.mock_all_auths();
    
    client.initialize(&admin);
    assert_eq!(client.get_admin(), Some(admin.clone()));
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_initialize_twice_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    env.mock_all_auths();
    
    client.initialize(&admin);
    client.initialize(&admin);
}

#[test]
fn test_admin_action() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    env.mock_all_auths();
    
    client.initialize(&admin);
    let result = client.admin_action(&admin, &10);
    assert_eq!(result, 20);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_admin_action_unauthorized() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    env.mock_all_auths();
    
    client.initialize(&admin);
    client.admin_action(&non_admin, &10);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    env.mock_all_auths();
    
    client.initialize(&admin);
    client.set_balance(&admin, &user1, &1000);
    
    client.transfer(&user1, &user2, &300);
    
    assert_eq!(client.get_balance(&user1), 700);
    assert_eq!(client.get_balance(&user2), 300);
}

#[test]
fn test_approve_and_transfer_from() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let recipient = Address::generate(&env);
    env.mock_all_auths();
    
    client.initialize(&admin);
    client.set_balance(&admin, &owner, &1000);
    client.approve(&owner, &spender, &500);
    
    client.transfer_from(&spender, &owner, &recipient, &200);
    
    assert_eq!(client.get_balance(&owner), 800);
    assert_eq!(client.get_balance(&recipient), 200);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_transfer_from_insufficient_allowance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let recipient = Address::generate(&env);
    env.mock_all_auths();
    
    client.initialize(&admin);
    client.set_balance(&admin, &owner, &1000);
    client.approve(&owner, &spender, &100);
    
    client.transfer_from(&spender, &owner, &recipient, &200);
}

#[test]
fn test_multi_sig_action() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let signer1 = Address::generate(&env);
    let signer2 = Address::generate(&env);
    let signer3 = Address::generate(&env);
    env.mock_all_auths();
    
    let signers = vec![&env, signer1, signer2, signer3];
    let result = client.multi_sig_action(&signers, &10);
    assert_eq!(result, 13);
}

#[test]
fn test_emit_event() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let user = Address::generate(&env);
    env.mock_all_auths();
    
    client.emit_event(&user, &symbol_short!("hello"));
}

#[test]
fn test_set_balance_admin_only() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    env.mock_all_auths();
    
    client.initialize(&admin);
    client.set_balance(&admin, &user, &5000);
    
    assert_eq!(client.get_balance(&user), 5000);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_set_balance_non_admin_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let user = Address::generate(&env);
    env.mock_all_auths();
    
    client.initialize(&admin);
    client.set_balance(&non_admin, &user, &5000);
}

// ---------------------------------------------------------------------------
// 8. Multi-party authorization tests
// ---------------------------------------------------------------------------

#[test]
fn test_multi_party_role_hierarchy() {
    let (env, _contract_id, admin, client) = setup_initialized_contract();
    let moderator = Address::generate(&env);
    let user = Address::generate(&env);

    client.grant_role(&admin, &moderator, &Role::Moderator);
    client.grant_role(&admin, &user, &Role::User);

    assert!(client.has_role(&admin, &Role::Admin));
    assert!(client.has_role(&moderator, &Role::Moderator));
    assert!(client.has_role(&user, &Role::User));

    let admin_result = client.admin_action(&admin, &10);
    assert_eq!(admin_result, 20);

    let mod_result = client.moderator_action(&moderator, &10);
    assert_eq!(mod_result, 110);
}

#[test]
fn test_multi_party_cooldown_isolation() {
    let (env, _contract_id, admin, client) = setup_initialized_contract();
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    client.set_cooldown(&admin, &100);

    env.ledger().with_mut(|li| li.timestamp = 200);
    client.cooldown_action(&user1);

    env.ledger().with_mut(|li| li.timestamp = 210);
    let result = client.cooldown_action(&user2);
    assert_eq!(result, 210);
}

#[test]
#[should_panic(expected = "Not admin")]
fn test_non_admin_cannot_grant_roles() {
    let (env, _contract_id, admin, client) = setup_initialized_contract();
    let attacker = Address::generate(&env);
    let victim = Address::generate(&env);

    client.grant_role(&admin, &attacker, &Role::User);
    client.grant_role(&attacker, &victim, &Role::Admin);
}

// ---------------------------------------------------------------------------
// 9. Edge case tests
// ---------------------------------------------------------------------------

#[test]
fn test_role_overwrite() {
    let (env, _contract_id, admin, client) = setup_initialized_contract();
    let user = Address::generate(&env);

    client.grant_role(&admin, &user, &Role::User);
    assert!(client.has_role(&user, &Role::User));

    client.grant_role(&admin, &user, &Role::Moderator);
    assert!(client.has_role(&user, &Role::Moderator));
    assert!(!client.has_role(&user, &Role::User));
}

#[test]
#[should_panic(expected = "No role assigned")]
fn test_get_role_unassigned_panics() {
    let (env, _contract_id, _admin, client) = setup_initialized_contract();
    let unassigned = Address::generate(&env);
    client.get_role(&unassigned);
}

#[test]
fn test_cooldown_zero_period() {
    let (env, _contract_id, admin, client) = setup_initialized_contract();

    client.set_cooldown(&admin, &0);

    env.ledger().with_mut(|li| li.timestamp = 100);
    client.cooldown_action(&admin);

    env.ledger().with_mut(|li| li.timestamp = 100);
    let result = client.cooldown_action(&admin);
    assert_eq!(result, 100);
}

#[test]
fn test_time_lock_zero_allows_immediate() {
    let (env, _contract_id, admin, client) = setup_initialized_contract();

    client.set_time_lock(&admin, &0);

    env.ledger().with_mut(|li| li.timestamp = 1);
    let result = client.time_locked_action(&admin);
    assert_eq!(result, 1);
}

#[test]
fn test_state_default_is_active() {
    let (_env, _contract_id, _admin, client) = setup_initialized_contract();
    assert_eq!(client.get_state(), 0);
}

#[test]
fn test_revoke_nonexistent_role() {
    let (env, _contract_id, admin, client) = setup_initialized_contract();
    let user = Address::generate(&env);
    client.revoke_role(&admin, &user);
}

#[test]
#[should_panic(expected = "Not admin")]
fn test_non_admin_cannot_set_state() {
    let (env, _contract_id, admin, client) = setup_initialized_contract();
    let user = Address::generate(&env);
    client.grant_role(&admin, &user, &Role::User);
    client.set_state(&user, &ContractState::Paused);
}

#[test]
#[should_panic(expected = "Not admin")]
fn test_non_admin_cannot_set_time_lock() {
    let (env, _contract_id, admin, client) = setup_initialized_contract();
    let user = Address::generate(&env);
    client.grant_role(&admin, &user, &Role::User);
    client.set_time_lock(&user, &1000);
}

#[test]
#[should_panic(expected = "Not admin")]
fn test_non_admin_cannot_set_cooldown() {
    let (env, _contract_id, admin, client) = setup_initialized_contract();
    let user = Address::generate(&env);
    client.grant_role(&admin, &user, &Role::User);
    client.set_cooldown(&user, &100);
}
