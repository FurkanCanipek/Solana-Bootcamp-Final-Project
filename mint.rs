use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	AccountPDA,
	TreeMetadata,
};


/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable, signer]` mint: [Mint] 
/// 2. `[writable]` tree: [TreeMetadata] 
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 4. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 5. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 6. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 7. `[]` token_program: [AccountInfo] SPL Token program
/// 8. `[signer]` owner: [AccountInfo] The mint's minting authority.
/// 9. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
/// 10. `[]` csl_spl_assoc_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
///
/// Data:
/// - leaf_color: [String] 
/// - lifespan: [u32] 
/// - age: [u32] 
/// - leaf_pattern: [String] 
/// - wood_color: [String] 
/// - short_description: [String] 
pub fn mint(
	program_id: &Pubkey,
	for_initialize_mint_2: &[&AccountInfo],
	for_create: &[&AccountInfo],
	for_mint_to: &[&AccountInfo],
	for_set_authority: &[&AccountInfo],
	mint: &Account<spl_token::state::Mint>,
	tree: &mut AccountPDA<TreeMetadata>,
	funding: &AccountInfo,
	assoc_token_account: &AccountInfo,
	wallet: &AccountInfo,
	owner: &AccountInfo,
	leafColor: String,
	lifespan: u32,
	//age: u32,
	leafPattern: String,
	woodColor: String,
	shortDescription: String,
) -> ProgramResult {
    tree.data.leaf_color = leafColor;
    tree.data.lifespan = lifespan;
    tree.data.age = 0;
    tree.data.leaf_pattern= leafPattern;
    tree.data.wood_color= woodColor;
    tree.data.short_description= shortDescription;
    tree.data.mint = *mint.info.key;
    tree.data.assoc_account = Some(*assoc_token_account.key);
    
    csl_spl_token::src::cpi::initialize_mint_2(for_initialize_mint_2, 0, *wallet.key, None)?;
    csl_spl_assoc_token::src::cpi::create(for_create)?;
    csl_spl_token::src::cpi::mint_to(for_mint_to, 1)?;
    csl_spl_token::src::cpi::set_authority(for_set_authority, 0, None)?;


    Ok(())
}
