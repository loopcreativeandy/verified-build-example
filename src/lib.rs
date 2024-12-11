use pinocchio::{
    account_info::AccountInfo, entrypoint, msg, pubkey::Pubkey, ProgramResult
  };
  
entrypoint!(process_instruction);

pub fn process_instruction(
_program_id: &Pubkey,
_accounts: &[AccountInfo],
_instruction_data: &[u8],
) -> ProgramResult {
    msg!("This program does exactly what the source code says!");

    Ok(())
}