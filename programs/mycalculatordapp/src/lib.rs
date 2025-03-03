use anchor_lang::prelude::*;

declare_id!("E31srdTDXcMJ88QKhqfF1KfeCjorDEqozEPbjBck6oTA");

#[program]
pub mod mycalculatordapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
