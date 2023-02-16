use anchor_lang::prelude::*;

declare_id!("Tg6PaFpoGXkYsidMpWTK62BeZ7FEfcYkg476zPFsLnT");

#[program]
pub mod storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.data.value = data;
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        ctx.accounts.data.value = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space=2000)]
    pub data: Account<'info, Data>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub data: Account<'info, Data>,
}


#[account]
pub struct Data {
    value: u64,
}