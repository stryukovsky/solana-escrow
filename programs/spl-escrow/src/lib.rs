use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Transfer, Token, TokenAccount},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[error_code]
enum ErrorCodes {
    TooEarly,
    NoSufficientSupply,
    AmountTooBig,
    TransferFailed,
}

#[program]
pub mod spl_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_supply: u64) -> Result<()> {
        let cpi_mint_to = ctx.accounts.cpi_mint_to();
        mint_to(cpi_mint_to, initial_supply)
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
        escrow.withdraw_available_at += 50000;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        require!(ctx.accounts.admin_account.amount >= amount, ErrorCodes::NoSufficientSupply);
        let escrow = &mut ctx.accounts.escrow;
        require!(escrow.amount_accrued >= amount, ErrorCodes::AmountTooBig);
        let current_time = Clock::get().unwrap().unix_timestamp;
        require!(
            current_time >= escrow.withdraw_available_at,
            ErrorCodes::TooEarly
        );
        escrow.amount_accrued -= amount;
        escrow.withdraw_available_at += 5;
        let cpi_context = ctx.accounts.cpi_transfer();
        transfer(cpi_context, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, mint::decimals = 6, mint::authority = authority)]
    pub mint: Account<'info, Mint>,

    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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
    pub escrow: Account<'info, Escrow>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    /// CHECK: used only for transfer verification
    pub mint_authority: AccountInfo<'info>, 

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub escrow: Account<'info, Escrow>,

    pub token_program: Program<'info, Token>,
}

impl<'info> Initialize<'info> {
    pub fn cpi_mint_to(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = MintTo {
            to: self.token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        return CpiContext::new(program, accounts);
    }
}

impl <'info> Withdraw<'info> {
    pub fn cpi_transfer(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = Transfer{
            from: self.admin_account.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

#[account]
pub struct Escrow {
    withdraw_available_at: i64,
    amount_accrued: u64,
}
