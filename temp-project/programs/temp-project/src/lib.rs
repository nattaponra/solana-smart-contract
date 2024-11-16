use anchor_lang::prelude::*;

declare_id!("3EV2RvyHQt6i2fFsYCBLJ6ZLT9Bfmu9EqyWAodaexdkV");

#[program]
pub mod temp_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}