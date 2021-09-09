mod utils;

use metaplex_nft_packs::{
    instruction::{AddVoucherToPackArgs, InitPackSetArgs},
    state::ActionOnProve,
};
use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::{
    signature::Keypair, signer::Signer, transaction::TransactionError, transport::TransportError,
};
use utils::*;

async fn setup() -> (
    ProgramTestContext,
    TestPackSet,
    TestMetadata,
    TestMetadata,
    TestPackVoucher,
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
                mutable: true,
            },
        )
        .await
        .unwrap();

    let test_metadata = TestMetadata::new();
    let test_master_edition = TestMasterEditionV2::new(&test_metadata);

    // Create only instance to obtain generated PDA later
    let test_new_metadata = TestMetadata::new();

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

    let test_pack_voucher = TestPackVoucher::new(&test_pack_set, 1);
    test_pack_set
        .add_voucher(
            &mut context,
            &test_pack_voucher,
            &test_master_edition,
            &test_metadata,
            &user,
            AddVoucherToPackArgs {
                max_supply: Some(5),
                number_to_open: 4,
                action_on_prove: ActionOnProve::Burn,
            },
        )
        .await
        .unwrap();

    (
        context,
        test_pack_set,
        test_metadata,
        test_new_metadata,
        test_pack_voucher,
        test_master_edition,
        user,
    )
}

#[tokio::test]
async fn success() {
    let (
        mut context,
        test_pack_set,
        test_metadata,
        test_new_metadata,
        test_pack_voucher,
        test_master_edition,
        _user,
    ) = setup().await;

    let payer_pubkey = context.payer.pubkey();

    test_pack_set
        .mint_edition_with_voucher(
            &mut context,
            &test_metadata,
            &test_pack_voucher,
            &test_new_metadata,
            &test_master_edition,
            &payer_pubkey,
            &payer_pubkey,
            1,
        )
        .await
        .unwrap();

    assert!(!is_empty_account(&mut context, &test_new_metadata.pubkey).await);
}

#[tokio::test]
async fn fail_invalid_token_account() {
    let (
        mut context,
        test_pack_set,
        test_metadata,
        test_new_metadata,
        mut test_pack_voucher,
        test_master_edition,
        _user,
    ) = setup().await;

    let payer_pubkey = context.payer.pubkey();

    // Change PackVoucher token account to fake one
    test_pack_voucher.token_account = Keypair::new();

    let result = test_pack_set
        .mint_edition_with_voucher(
            &mut context,
            &test_metadata,
            &test_pack_voucher,
            &test_new_metadata,
            &test_master_edition,
            &payer_pubkey,
            &payer_pubkey,
            0,
        )
        .await;

    assert_transport_error!(
        result.unwrap_err(),
        TransportError::TransactionError(TransactionError::InstructionError(
            0,
            InstructionError::InvalidArgument
        ))
    );
}
