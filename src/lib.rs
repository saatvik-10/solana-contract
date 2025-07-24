use solana_program::{account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey
};

//declaring entry point
entrypoint!(process_instruction);

//function to handle all instructions to the program
pub fn process_instruction(
    program_id: &Pubkey, //public key of my program
    accounts: &[AccountInfo], //accounts involved in transaction
    instruction_data: &[u8], //data passed by caller to program
) -> ProgramResult {
    msg!("Hello Solana");

    Ok(())
}

