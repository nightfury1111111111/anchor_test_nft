use anchor_lang::prelude::*;

declare_id!("CK6CX3Pk6K6rYDMqZPifsL279QisiqNh2SLLyAXShFEb");

#[program]
pub mod topdev {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
