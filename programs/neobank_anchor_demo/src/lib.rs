use anchor_lang::{
    prelude::*,
    system_program::{self, Transfer as SystemTransfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, TransferChecked},
};

declare_id!("DvTZxzMYaBKTTHCuaR9QQoZXxeqWuxqwgDoZmTt9kFd7");

const BANK_ACCOUNT_SEED: &[u8] = b"bank-account";
const SOL_VAULT_SEED: &[u8] = b"sol-vault";
const VAULT_AUTHORITY_SEED: &[u8] = b"vault-authority";

#[program]
pub mod neobank_anchor_demo {
    use super::*;

    pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
        let bank_account = &mut ctx.accounts.bank_account;
        bank_account.owner = ctx.accounts.owner.key();
        bank_account.sol_vault = ctx.accounts.sol_vault.key();
        bank_account.vault_authority = ctx.accounts.vault_authority.key();
        bank_account.sol_balance = 0;
        bank_account.token_vault_count = 0;
        bank_account.bank_bump = ctx.bumps.bank_account;
        bank_account.sol_vault_bump = ctx.bumps.sol_vault;
        bank_account.vault_authority_bump = ctx.bumps.vault_authority;
        Ok(())
    }

    pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        require!(amount > 0, NeobankError::InvalidAmount);

        ctx.accounts.transfer_sol_to_vault(amount)?;

        let bank_account = &mut ctx.accounts.bank_account;
        bank_account.sol_balance = bank_account
            .sol_balance
            .checked_add(amount)
            .ok_or(NeobankError::MathOverflow)?;

        Ok(())
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        require!(amount > 0, NeobankError::InvalidAmount);
        require!(
            ctx.accounts.bank_account.sol_balance >= amount,
            NeobankError::InsufficientSolBalance
        );

        ctx.accounts.transfer_sol_to_owner(amount)?;

        let bank_account = &mut ctx.accounts.bank_account;
        bank_account.sol_balance = bank_account
            .sol_balance
            .checked_sub(amount)
            .ok_or(NeobankError::MathOverflow)?;

        Ok(())
    }

    pub fn initialize_token_vault(ctx: Context<InitializeTokenVault>) -> Result<()> {
        let bank_account = &mut ctx.accounts.bank_account;
        bank_account.token_vault_count = bank_account
            .token_vault_count
            .checked_add(1)
            .ok_or(NeobankError::MathOverflow)?;
        Ok(())
    }

    pub fn deposit_spl(ctx: Context<DepositSpl>, amount: u64) -> Result<()> {
        require!(amount > 0, NeobankError::InvalidAmount);
        require!(
            ctx.accounts.owner_token_account.amount >= amount,
            NeobankError::InsufficientUserTokenBalance
        );

        ctx.accounts.transfer_tokens_to_vault(amount)
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {
        require!(amount > 0, NeobankError::InvalidAmount);
        require!(
            ctx.accounts.token_vault.amount >= amount,
            NeobankError::InsufficientTokenBalance
        );

        ctx.accounts.transfer_tokens_to_owner(amount)
    }
}

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + BankAccount::INIT_SPACE,
        seeds = [BANK_ACCOUNT_SEED, owner.key().as_ref()],
        bump
    )]
    pub bank_account: Account<'info, BankAccount>,
    #[account(
        init,
        payer = owner,
        space = 0,
        owner = system_program.key(),
        seeds = [SOL_VAULT_SEED, owner.key().as_ref()],
        bump
    )]
    /// CHECK: The vault is a PDA owned by the System Program and only holds lamports.
    pub sol_vault: UncheckedAccount<'info>,
    /// CHECK: This PDA never stores data. It only signs CPIs and owns token vaults.
    #[account(
        seeds = [VAULT_AUTHORITY_SEED, owner.key().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner @ NeobankError::Unauthorized,
        constraint = bank_account.sol_vault == sol_vault.key() @ NeobankError::InvalidSolVault,
        seeds = [BANK_ACCOUNT_SEED, owner.key().as_ref()],
        bump = bank_account.bank_bump
    )]
    pub bank_account: Account<'info, BankAccount>,
    #[account(
        mut,
        owner = system_program.key(),
        seeds = [SOL_VAULT_SEED, owner.key().as_ref()],
        bump = bank_account.sol_vault_bump
    )]
    /// CHECK: The vault is a PDA owned by the System Program and validated with seeds + owner.
    pub sol_vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner @ NeobankError::Unauthorized,
        constraint = bank_account.sol_vault == sol_vault.key() @ NeobankError::InvalidSolVault,
        seeds = [BANK_ACCOUNT_SEED, owner.key().as_ref()],
        bump = bank_account.bank_bump
    )]
    pub bank_account: Account<'info, BankAccount>,
    #[account(
        mut,
        owner = system_program.key(),
        seeds = [SOL_VAULT_SEED, owner.key().as_ref()],
        bump = bank_account.sol_vault_bump
    )]
    /// CHECK: The vault is a PDA owned by the System Program and validated with seeds + owner.
    pub sol_vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeTokenVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner @ NeobankError::Unauthorized,
        constraint = bank_account.vault_authority == vault_authority.key() @ NeobankError::InvalidVaultAuthority,
        seeds = [BANK_ACCOUNT_SEED, owner.key().as_ref()],
        bump = bank_account.bank_bump
    )]
    pub bank_account: Account<'info, BankAccount>,
    /// CHECK: PDA signer validated by seeds and stored address.
    #[account(
        seeds = [VAULT_AUTHORITY_SEED, owner.key().as_ref()],
        bump = bank_account.vault_authority_bump
    )]
    pub vault_authority: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
        associated_token::token_program = token_program
    )]
    pub token_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSpl<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner @ NeobankError::Unauthorized,
        constraint = bank_account.vault_authority == vault_authority.key() @ NeobankError::InvalidVaultAuthority,
        seeds = [BANK_ACCOUNT_SEED, owner.key().as_ref()],
        bump = bank_account.bank_bump
    )]
    pub bank_account: Account<'info, BankAccount>,
    /// CHECK: PDA signer validated by seeds and stored address.
    #[account(
        seeds = [VAULT_AUTHORITY_SEED, owner.key().as_ref()],
        bump = bank_account.vault_authority_bump
    )]
    pub vault_authority: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program
    )]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
        associated_token::token_program = token_program
    )]
    pub token_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawSpl<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner @ NeobankError::Unauthorized,
        constraint = bank_account.vault_authority == vault_authority.key() @ NeobankError::InvalidVaultAuthority,
        seeds = [BANK_ACCOUNT_SEED, owner.key().as_ref()],
        bump = bank_account.bank_bump
    )]
    pub bank_account: Account<'info, BankAccount>,
    /// CHECK: PDA signer validated by seeds and stored address.
    #[account(
        seeds = [VAULT_AUTHORITY_SEED, owner.key().as_ref()],
        bump = bank_account.vault_authority_bump
    )]
    pub vault_authority: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program
    )]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
        associated_token::token_program = token_program
    )]
    pub token_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> DepositSol<'info> {
    fn transfer_sol_to_vault(&self, amount: u64) -> Result<()> {
        let cpi_accounts = SystemTransfer {
            from: self.owner.to_account_info(),
            to: self.sol_vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);
        system_program::transfer(cpi_ctx, amount)
    }
}

impl<'info> WithdrawSol<'info> {
    fn transfer_sol_to_owner(&self, amount: u64) -> Result<()> {
        let owner_key = self.owner.key();
        let bump = [self.bank_account.sol_vault_bump];
        let signer_seeds: &[&[u8]] = &[SOL_VAULT_SEED, owner_key.as_ref(), &bump];

        let cpi_accounts = SystemTransfer {
            from: self.sol_vault.to_account_info(),
            to: self.owner.to_account_info(),
        };
        let signer = [signer_seeds];
        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            &signer,
        );
        system_program::transfer(cpi_ctx, amount)
    }
}

impl<'info> DepositSpl<'info> {
    fn transfer_tokens_to_vault(&self, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.owner_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.token_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token::transfer_checked(cpi_ctx, amount, self.mint.decimals)
    }
}

impl<'info> WithdrawSpl<'info> {
    fn transfer_tokens_to_owner(&self, amount: u64) -> Result<()> {
        let owner_key = self.owner.key();
        let bump = [self.bank_account.vault_authority_bump];
        let signer_seeds: &[&[u8]] = &[VAULT_AUTHORITY_SEED, owner_key.as_ref(), &bump];

        let cpi_accounts = TransferChecked {
            from: self.token_vault.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.owner_token_account.to_account_info(),
            authority: self.vault_authority.to_account_info(),
        };
        let signer = [signer_seeds];
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            &signer,
        );
        token::transfer_checked(cpi_ctx, amount, self.mint.decimals)
    }
}

#[account]
pub struct BankAccount {
    pub owner: Pubkey,
    pub sol_vault: Pubkey,
    pub vault_authority: Pubkey,
    pub sol_balance: u64,
    pub token_vault_count: u16,
    pub bank_bump: u8,
    pub sol_vault_bump: u8,
    pub vault_authority_bump: u8,
}

impl BankAccount {
    pub const INIT_SPACE: usize = 32 + 32 + 32 + 8 + 2 + 1 + 1 + 1;
}

#[error_code]
pub enum NeobankError {
    #[msg("Amount must be greater than zero.")]
    InvalidAmount,
    #[msg("Only the owner can operate this bank account.")]
    Unauthorized,
    #[msg("The provided SOL vault does not match the bank account state.")]
    InvalidSolVault,
    #[msg("The provided vault authority does not match the bank account state.")]
    InvalidVaultAuthority,
    #[msg("The bank account does not have enough tracked SOL.")]
    InsufficientSolBalance,
    #[msg("The token vault does not have enough tokens.")]
    InsufficientTokenBalance,
    #[msg("The owner token account does not have enough tokens.")]
    InsufficientUserTokenBalance,
    #[msg("Math overflow.")]
    MathOverflow,
}
