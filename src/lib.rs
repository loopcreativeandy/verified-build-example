use pinocchio::{
    lazy_entrypoint::{ InstructionContext}, ProgramResult
  };
  use pinocchio::lazy_entrypoint;

lazy_entrypoint!(process_instruction);

pub fn process_instruction(
_context: InstructionContext,
) -> ProgramResult {
    Ok(())
}