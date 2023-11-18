use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	TreeMetadata,
};


/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` tree: [TreeMetadata] 
/// 2. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - tree_seed_mint: [Pubkey] Auto-generated, from input tree of type [TreeMetadata] set the seed named mint, required by the type
pub fn reverse_age(
	program_id: &Pubkey,
	tree: &mut AccountPDA<TreeMetadata>,
) -> ProgramResult {
    tree.data.age -=1;
    Ok(())
}
