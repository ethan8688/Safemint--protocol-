use anchor_lang::prelude::*;

declare_id!("SaFeInsu111111111111111111111111111111111");

#[program]
pub mod insurance_protocol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, review_admin: Pubkey) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        cfg.review_admin = review_admin;
        Ok(())
    }

    pub fn pay_premium(_ctx: Context<PayPremium>, _coverage_days: u16, _amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn submit_claim(_ctx: Context<SubmitClaim>, _details: String) -> Result<()> {
        Ok(())
    }

    pub fn resolve_claim(_ctx: Context<ResolveClaim>, _approve: bool, _payout: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = 8 + 64)]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Config {
    pub review_admin: Pubkey,
}

#[derive(Accounts)]
pub struct PayPremium<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SubmitClaim<'info> {
    #[account(mut)]
    pub claimant: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResolveClaim<'info> {
    pub reviewer: Signer<'info>,
}