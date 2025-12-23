use anchor_lang::prelude::*;

declare_id!("BhJXdCKNpLkRf2qWPwFYgYZFFE1WLngKNjMMhJBE7oku");

#[program]
pub mod ticket_machine {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
