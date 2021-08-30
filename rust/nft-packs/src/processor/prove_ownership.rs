//! ProveOwnership instruction processing

use crate::{
    error::NFTPacksError,
    math::SafeMath,
    state::{
        ActionOnProve, InitProvingProcessParams, PackSet, PackSetState, PackVoucher, ProvingProcess,
    },
    utils::*,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::state::Account;
use spl_token_metadata::{
    state::{Edition, EDITION, PREFIX as EDITION_PREFIX},
    utils::assert_derivation,
};

/// Process ProveOwnership instruction
pub fn prove_ownership(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pack_set_account = next_account_info(account_info_iter)?;
    let edition_data_account = next_account_info(account_info_iter)?;
    let edition_mint_account = next_account_info(account_info_iter)?;
    let voucher_account = next_account_info(account_info_iter)?;
    let proving_process_account = next_account_info(account_info_iter)?;
    let user_wallet_account = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(rent_info)?;

    assert_owned_by(pack_set_account, program_id)?;
    assert_owned_by(edition_data_account, &spl_token_metadata::id())?;
    assert_owned_by(voucher_account, program_id)?;

    let pack_set = PackSet::unpack(&pack_set_account.data.borrow_mut())?;

    if pack_set.pack_state != PackSetState::Activated {
        return Err(NFTPacksError::PackSetNotActivated.into());
    }

    assert_derivation(
        program_id,
        proving_process_account,
        &[
            pack_set_account.key.as_ref(),
            ProvingProcess::PREFIX.as_bytes(),
            user_wallet_account.key.as_ref(),
        ],
    )?;

    let mut proving_process = get_proving_process_data(
        proving_process_account,
        user_wallet_account.key,
        pack_set_account.key,
        rent,
    )?;

    assert_derivation(
        program_id,
        voucher_account,
        &[
            pack_set_account.key.as_ref(),
            PackVoucher::PREFIX.as_bytes(),
            &proving_process
                .proved_vouchers
                .error_increment()?
                .to_le_bytes(),
        ],
    )?;

    let voucher = PackVoucher::unpack(&voucher_account.data.borrow_mut())?;

    if proving_process.proved_vouchers == pack_set.pack_vouchers {
        return Err(NFTPacksError::ProvingPackProcessCompleted.into());
    }

    if proving_process.proved_voucher_editions == voucher.number_to_open {
        return Err(NFTPacksError::ProvingVoucherProcessCompleted.into());
    }

    assert_derivation(
        &spl_token_metadata::id(),
        edition_data_account,
        &[
            EDITION_PREFIX.as_bytes(),
            spl_token_metadata::id().as_ref(),
            edition_mint_account.key.as_ref(),
            EDITION.as_bytes(),
        ],
    )?;

    let edition = Edition::from_account_info(edition_data_account)?;

    if edition.parent != voucher.master {
        return Err(NFTPacksError::WrongEdition.into());
    }

    let user_token_acc = Account::unpack(&user_token_account.data.borrow_mut())?;
    if user_token_acc.mint != *edition_mint_account.key {
        return Err(NFTPacksError::WrongEditionMint.into());
    }

    match voucher.action_on_prove {
        ActionOnProve::Burn => {
            burn_tokens(
                user_token_account.clone(),
                edition_mint_account.clone(),
                user_wallet_account.clone(),
                ProvingProcess::TOKEN_AMOUNT,
            )?;
            close_token_account(
                user_token_account.clone(),
                user_wallet_account.clone(),
                user_wallet_account.clone(),
            )?;
        }
        ActionOnProve::Redeem => {
            msg!("Redeem action is not implemented in current stage.");
            msg!("Do nothing with token.");
        }
    }

    if proving_process.proved_voucher_editions.error_increment()? == voucher.number_to_open {
        proving_process.proved_voucher_editions = 0;
        proving_process.proved_vouchers = proving_process.proved_vouchers.error_increment()?;
    } else {
        proving_process.proved_voucher_editions =
            proving_process.proved_voucher_editions.error_increment()?;
    }

    ProvingProcess::pack(proving_process, *proving_process_account.data.borrow_mut())?;

    Ok(())
}

/// Returns deserialized proving process data or initialized if it wasn't initialized yet
pub fn get_proving_process_data(
    account_info: &AccountInfo,
    user_wallet: &Pubkey,
    pack_set: &Pubkey,
    rent: &Rent,
) -> Result<ProvingProcess, ProgramError> {
    let unpack = ProvingProcess::unpack(&account_info.data.borrow_mut());

    let proving_process = match unpack {
        Ok(data) => Ok(data),
        Err(_) => {
            assert_rent_exempt(rent, account_info)?;
            let mut data = ProvingProcess::unpack_unchecked(&account_info.data.borrow_mut())?;

            data.init(InitProvingProcessParams {
                user_wallet: *user_wallet,
                pack_set: *pack_set,
            });
            Ok(data)
        }
    };

    proving_process
}
