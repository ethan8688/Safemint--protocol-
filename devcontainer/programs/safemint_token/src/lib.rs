use anchor_lang::prelude::*;

declare_id!("SaFeM1nT111111111111111111111111111111111");

#[program]
pub mod safemint_token {
    use super::*;

    /// Initialize mint & config PDAs (to be implemented).
    pub fn initialize(_ctx: Context<Initialize>, _decimals: u8) -> Result<()> {
        Ok(())
    }

    /// Custom transfer: 1% burn + 0.5% fee to insurance vault (to be implemented).
    pub fn transfer_with_tax(_ctx: Context<TransferWithTax>, _amount: u64) -> Result<()> {
        // burn_amount = amount / 100;      // 1%
        // fee_amount  = amount / 200;      // 0.5%
        // send_amount = amount - burn_amount - fee_amount
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferWithTax<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}