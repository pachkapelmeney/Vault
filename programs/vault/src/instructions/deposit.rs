use anchor_lang::{prelude::*};
use anchor_spl::{associated_token::{self, AssociatedToken}, token::{Token, TokenAccount, Transfer}};

use crate::{VaultState, program::Vault};

#[derive(Accounts)]
pub struct Deposit<'info>{
    //transfer tokens from user wallet to token-account of the wallet
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, //do ineed to init it second itme?
        payer=user,
        seeds=[b"state", user.key().as_ref()],
    bump,
    space= 8 + VaultState::INIT_SPACE,)]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds=[b"vault", user.key().as_ref(),
        ], bump
    )]
    pub vault: SystemAccount<'info>,

    pub vault_token_account: Program<'info, AssociatedToken>,

    #[account()]
    pub user_token_accont: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    // pub amount: 
}


pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()>
{
    let vault_state = &mut ctx.accounts.vault_state;
    let vault = &mut ctx.accounts.vault;
    let user_token_account = &mut ctx.accounts.user_token_accont;
    let vault_token_account = &mut ctx.accounts.vault_token_account;

    // ctx.user_token_accont::transfer(user_token_account.to_account_info(), vault, lamports);
    
    let cpi_accounts = Transfer {
        from: user_token_account.to_account_info(),
        to: vault_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info()
    };

    let cpi_program = &mut ctx.accounts.token_program;


    let cpi_ctx = CpiContext::new(cpi_program.to_account_info(), cpi_accounts);
    anchor_spl::token::transfer(cpi_ctx, amount);
    

    Ok(())
}