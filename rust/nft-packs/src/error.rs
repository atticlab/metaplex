//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the Metaplex NFT packs program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum NFTPacksError {
    /// Total packs amount should be more then 0
    #[error("Total packs amount should be more then 0")]
    WrongTotalPacksAmount,

    /// Proved vouchers mismatch pack vourchers
    #[error("Proved vouchers mismatch pack vourchers")]
    ProvedVouchersMismatchPackVouchers,

    /// Pack is already open
    #[error("Pack is already open")]
    PackIsAlreadyOpen,

    /// NFT pack set not fully configured
    #[error("NFT pack set not fully configured")]
    PackSetNotConfigured,

    /// NFT pack set is already activated
    #[error("NFT pack set already activated")]
    PackAlreadyActivated,

    /// NFT pack set is already deactivated
    #[error("NFT pack set already deactivated")]
    PackAlreadyDeactivated,

    /// Pack set should be activated
    #[error("Pack set should be activated")]
    PackSetNotActivated,

    /// Proving process for this pack is completed
    #[error("Proving process for this pack is completed")]
    ProvingPackProcessCompleted,

    /// Proving process for this voucher is completed
    #[error("Proving process for this voucher is completed")]
    ProvingVoucherProcessCompleted,

    /// Received edition from wrong master
    #[error("Received edition from wrong master")]
    WrongEdition,

    /// Received wrong edition mint
    #[error("Received wrong edition mint")]
    WrongEditionMint,

    /// Overflow
    #[error("Overflow")]
    Overflow,

    /// Underflow
    #[error("Underflow")]
    Underflow,

    /// Pack set should be empty to delete it
    #[error("Pack set should be empty to delete it")]
    NotEmptyPackSet,

    /// Wrong pack state to change data
    #[error("Wrong pack state to change data")]
    WrongPackState,

    /// Pack set is immutable
    #[error("Pack set is immutable")]
    ImmutablePackSet,

    /// Total packs can't be less then pack cards amount
    #[error("Total packs can't be less then pack cards amount")]
    SmallTotalPacksAmount,

    /// Can't set the same value
    #[error("Can't set the same value")]
    CantSetTheSameValue,

    /// Wrong pack card received
    #[error("Wrong pack card received")]
    WrongPackCard,

    /// Wrong pack voucher received
    #[error("Wrong pack voucher received")]
    WrongPackVoucher,

    /// Max supply can't be less then current supply
    #[error("Max supply can't be less then current supply")]
    SmallMaxSupply,

    /// Number NFTs to open pack should be greater then zero
    #[error("Number NFTs to open pack should be greater then zero")]
    WrongNumberToOpen,
}

impl From<NFTPacksError> for ProgramError {
    fn from(e: NFTPacksError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for NFTPacksError {
    fn type_of() -> &'static str {
        "NFTPacksError"
    }
}

impl PrintProgramError for NFTPacksError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        msg!(&self.to_string())
    }
}
