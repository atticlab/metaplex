mod utils;

use metaplex_nft_packs::{error::NFTPacksError, instruction::InitPackSetArgs};
use num_traits::FromPrimitive;
use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::{
    signature::Keypair, signer::Signer, transaction::TransactionError, transport::TransportError,
};
use utils::*;

async fn setup(
    mutable: bool,
) -> (
    ProgramTestContext,
    TestPackSet,
    TestMetadata,
    TestMasterEditionV2,
    User,
) {
    let mut context = nft_packs_program_test().start_with_context().await;

    let test_pack_set = TestPackSet::new();
    test_pack_set
        .init(
            &mut context,
            InitPackSetArgs {
                name: [7; 32],
                total_packs: 5,
                mutable: mutable,
            },
        )
        .await
        .unwrap();

    let test_metadata = TestMetadata::new();
    let test_master_edition = TestMasterEditionV2::new(&test_metadata);

    let user_token_acc = Keypair::new();
    let user = User {
        owner: Keypair::new(),
        token_account: user_token_acc.pubkey(),
    };

    test_metadata
        .create(
            &mut context,
            "Test".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
            &user_token_acc,
            &test_pack_set.authority.pubkey(),
        )
        .await
        .unwrap();

    test_master_edition
        .create(&mut context, Some(10))
        .await
        .unwrap();

    (
        context,
        test_pack_set,
        test_metadata,
        test_master_edition,
        user,
    )
}

#[tokio::test]
async fn success() {
    let (mut context, test_pack_set, _test_metadata, _test_master_edition, _user) =
        setup(true).await;

    assert_eq!(test_pack_set.get_data(&mut context).await.total_packs, 5);

    test_pack_set
        .edit(&mut context, None, None, Some(1337))
        .await
        .unwrap();

    assert_eq!(test_pack_set.get_data(&mut context).await.total_packs, 1337);
}

#[tokio::test]
async fn fail_immutable() {
    let (mut context, test_pack_set, _test_metadata, _test_master_edition, _user) =
        setup(false).await;

    let result = test_pack_set
        .edit(&mut context, None, None, Some(1337))
        .await;

    assert_custom_error!(result.unwrap_err(), NFTPacksError::ImmutablePackSet);
}
