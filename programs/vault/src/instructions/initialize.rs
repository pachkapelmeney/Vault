use anchor_lang::prelude::*;
use crate::VaultState;
 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init,
    payer = user,
    seeds = [b"state", user.key().as_ref()],
    bump,
    space = 8 + VaultState::INIT_SPACE,)]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
        // PDA authority account without data; no serialization required
        )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, init_bump: InitializeBumps) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    vault_state.amount = 0;
    vault_state.authority = *ctx.accounts.user.key;
    vault_state.state_bump = init_bump.vault;
    vault_state.state_bump = init_bump.vault_state;

    let vault = &mut 

    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
//why would you do this