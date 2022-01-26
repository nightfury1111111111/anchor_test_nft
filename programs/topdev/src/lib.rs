use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, 
    Token, 
    Mint, 
    TokenAccount, 
    SetAuthority, 
    MintTo, 
    Transfer, 
};
use spl_token::instruction::AuthorityType;
use std::time::{ Duration };

declare_id!("3MzZ4DoRKjH8ar7VELx7KRg4KB31sayjQi81RYucUvpB");

#[program]
pub mod topdev {
    use super::*;

    pub fn init_vault_token(ctx: Context<PlatformToken>) -> ProgramResult {
        token::mint_to(
            ctx.accounts.mint_to(), 
            1_000_000_0000000000)?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlatformToken<'info> {
    #[account(mut)]
    pub vault_mint: Account<'info, Mint>,
    #[account(mut)]
    pub vault_token_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> PlatformToken<'info> {
    pub fn mint_to(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let account = MintTo {
            mint: self.vault_mint.to_account_info().clone(),
            to: self.vault_token_account.to_account_info().clone(),
            authority: self.user.clone(),
        };
        let program = self.token_program.to_account_info();
        CpiContext::new(program,account)
    }
}

#[derive(Accounts)]
pub struct MintInfo<'info> {
    #[account(init, payer = owner, space = 512)]
    pub bid_info: Account<'info, BidInfo>,
    #[account(mut)] // Needed for isWriteable
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)] // Needed for isWriteable
    pub owner_platform_token: Account<'info, TokenAccount>,
    #[account(mut)] // Needed for isWriteable
    pub mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    #[account(mut, signer)]
    pub owner: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintInfo<'info> {
    pub fn mint_to(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let account = MintTo {
            mint: self.mint.to_account_info().clone(),
            to: self.token_account.to_account_info().clone(),
            authority: self.owner.clone(),
        };
        let program = self.token_program.to_account_info();
        CpiContext::new(program,account)
    }
    pub fn null_authority(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let account = SetAuthority {
            current_authority: self.owner.clone(),
            account_or_mint: self.mint.to_account_info().clone(),
        };
        let program = self.token_program.to_account_info();
        CpiContext::new(program, account)
    }
}