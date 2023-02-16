use anchor_lang::prelude::*;
use storage::cpi::set_data;
use storage::program::Storage;
use storage::Data;
use storage::cpi::accounts::SetData;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod storage_manager {
    use super::*;

    pub fn interact(ctx: Context<Interact>, data: u64) -> Result<()> {
        set_data(ctx.accounts.build_ctx(), data)
    }
}

#[derive(Accounts)]
pub struct Interact<'info> {
    #[account(mut)]
    pub data: Account<'info, Data>,
    pub storage_program: Program<'info, Storage>,
}

impl<'info> Interact<'info> {
    pub fn build_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let program = self.storage_program.to_account_info();
        let accounts = SetData {
            data: self.data.to_account_info()
        };
        CpiContext::new(program, accounts)
    }
}
