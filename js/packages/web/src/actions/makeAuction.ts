import { Keypair, PublicKey, TransactionInstruction } from '@solana/web3.js';
import {
  utils,
  actions,
  findProgramAddress,
  IPartialCreateAuctionArgs,
  IPartialCreateAuctionArgsV2,
  CreateAuctionArgs,
  CreateAuctionArgsV2,
} from '@oyster/common';

const { AUCTION_PREFIX, createAuction } = actions;

// This command makes an auction
export async function makeAuction(
  wallet: any,
  vault: PublicKey,
  auctionSettings: IPartialCreateAuctionArgsV2,
): Promise<{
  auction: PublicKey;
  instructions: TransactionInstruction[];
  signers: Keypair[];
}> {
  const PROGRAM_IDS = utils.programIds();

  let signers: Keypair[] = [];
  let instructions: TransactionInstruction[] = [];
  const auctionKey: PublicKey = (
    await findProgramAddress(
      [
        Buffer.from(AUCTION_PREFIX),
        PROGRAM_IDS.auction.toBuffer(),
        vault.toBuffer(),
      ],
      PROGRAM_IDS.auction,
    )
  )[0];

  const fullSettings = new CreateAuctionArgsV2({
    ...auctionSettings,
    authority: wallet.publicKey,
    resource: vault,
  });

  createAuction(fullSettings, wallet.publicKey, instructions);

  return { instructions, signers, auction: auctionKey };
}
