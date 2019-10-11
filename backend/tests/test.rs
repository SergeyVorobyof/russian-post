#[test]
fn test_rgs() {
    // Create 4 wallets.
    let (mut testkit, api, _) = create_testkit();
    let (tx_alice, key_alice) = api.create_wallet(ALICE_NAME, 0);
    let (tx_bob, key_bob) = api.create_wallet(BOB_NAME, 0);
    let (tx_michael, key_michael) = api.create_wallet(MICHAEL_NAME, 0);
    let (tx_alex, key_alex) = api.create_wallet(BOB_NAME, 0);
    
    testkit.create_block();
    api.assert_tx_status(tx_alice.hash(), &json!({ "type": "success" }));
    api.assert_tx_status(tx_bob.hash(), &json!({ "type": "success" }));

    let wallet = api.get_wallet(*tx_alice.pub_key()).unwrap();
    assert_eq!(wallet.balance(), 1);
    let wallet = api.get_wallet(*tx_alex.pub_key()).unwrap();
    assert_eq!(wallet.balance(), 1);

    let tx = Vote::new(
        tx_alice.pub_key(),
        tx_bob.pub_key(),
        1,  // amount
        0,  // seed
        &key_alice,
    );

    let rgs = RingSignature :: new(
        [tx_alice.pub_key(), tx_bob.pub_key()], // list of voters
        0, //seed
        &key_alice,
    );
    println!("RingSignature = {}", serde_json::to_string_pretty(&rgs).unwrap());
    api.vote(&rgs);
    testkit.create_block();
    api.assert_tx_status(rgs.hash(), &json!({ "type": "success" }));

    let wallet = api.get_wallet(*tx_alice.pub_key()).unwrap();
    assert_eq!(wallet.balance(), 0);
    let wallet = api.get_wallet(*tx_alex.pub_key()).unwrap();
    assert_eq!(wallet.balance(), 2);
}
