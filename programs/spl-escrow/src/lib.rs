use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, mint_to, MintTo, Mint}, associated_token::AssociatedToken};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod spl_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_supply: u64) -> Result<()> {
        Ok(())
    }

    pub fn register(ctx: Context<Register>) -> Result<()> {
        let mut escrow = &mut ctx.accounts.escrow;
        escrow.withdraw_available_at = 5 + Clock::get().unwrap().unix_timestamp;
        escrow.amount_accrued = 0;
        Ok(())
    }

    pub fn accrue(ctx: Context<Accrue>, amount: u64) -> Result<()> {
        let mut escrow = &mut ctx.accounts.escrow;
        escrow.amount_accrued += amount;
        escrow.withdraw_available_at += 5;
        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(init, payer = authority, mint::decimals = 6, mint::authority = authority)]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(init, payer = authority, space = 8 + 8 + 8)]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Accrue<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
}

// impl <'info> Initialize<'info> {

//     pub fn build_cpi_mint_to(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
//         let program = self.token_program.to_account_info();
//         let accounts = MintTo{
//             mint: self.mint.to_account_info(),
//             to: self.admin_account.to_account_info(),
//             authority: self.authority.to_account_info(),
//         };
//         return CpiContext::new(program, accounts);
//     }
// }

#[account]
pub struct Escrow {
    withdraw_available_at: i64,
    amount_accrued: u64,
}
