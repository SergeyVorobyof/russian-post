#[test]
fn test_vote() {
    // Create 2 wallets.
    let (mut testkit, api, _) = create_testkit();
    let (tx_alice, key_alice) = api.create_wallet(ALICE_NAME, 0);
    let (tx_bob, _) = api.create_wallet(BOB_NAME, 0);
    testkit.create_block();
    api.assert_tx_status(tx_alice.hash(), &json!({ "type": "success" }));
    api.assert_tx_status(tx_bob.hash(), &json!({ "type": "success" }));

    let wallet = api.get_wallet(*tx_alice.pub_key()).unwrap();
    assert_eq!(wallet.balance(), 1);
    let wallet = api.get_wallet(*tx_bob.pub_key()).unwrap();
    assert_eq!(wallet.balance(), 1);

    let tx = Vote::new(
        tx_alice.pub_key(),
        tx_bob.pub_key(),
        1,  // amount
        0,  // seed
        &key_alice,
    );
    println!("vote = {}", serde_json::to_string_pretty(&tx).unwrap());
    api.vote(&tx);
    testkit.create_block();
    api.assert_tx_status(tx.hash(), &json!({ "type": "success" }));

    let wallet = api.get_wallet(*tx_alice.pub_key()).unwrap();
    assert_eq!(wallet.balance(), 0);
    let wallet = api.get_wallet(*tx_bob.pub_key()).unwrap();
    assert_eq!(wallet.balance(), 2);
}
