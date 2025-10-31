use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{VaultState, error::ErrorCode};

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        constraint = vault_state.authority == user.key(),
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}


pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    };

    let state_key = vault_state.key();
    let seeds: &[&[u8]] = &[
        b"vault",
        state_key.as_ref(),
        &[vault_state.vault_bump],
    ];
    let signer = &[seeds];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );
    token::transfer(cpi_ctx, amount)?;

    vault_state.amount = vault_state.amount.checked_sub(amount).ok_or(ErrorCode::Overflow)?;
    Ok(())
}