use anchor_lang::prelude::*;



#[account]
#[derive(InitSpace)] // - it doesn't take in a consideration the Discriminator size
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,

    pub authority: Pubkey,
    pub amount: u32,
    pub mint: Pubkey,
}

// impl Space for VaultState {
//     const INIT_SPACE: usize = 8 + 1 + 1;
// }