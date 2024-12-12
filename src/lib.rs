use pinocchio::{
    lazy_entrypoint::{ InstructionContext}, ProgramResult
  };
  use pinocchio::lazy_entrypoint;
use solana_program::msg;

lazy_entrypoint!(process_instruction);

pub fn process_instruction(
_context: InstructionContext,
) -> ProgramResult {
    msg!("give me a different hash please!");
    Ok(())
}