use anchor_lang::prelude::*;



#[account]
#[derive(InitSpace)] // - it doesn't take in a consideration the Discriminator size
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,

}

// impl Space for VaultState {
//     const INIT_SPACE: usize = 8 + 1 + 1;
// }