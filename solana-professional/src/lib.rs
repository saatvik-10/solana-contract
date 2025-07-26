pub mod instruction;
pub mod state;
use borsh::{BorshDeserialize, BorshSerialize};
use instruction::CounterInstruction;
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use state::Counter;

//declaring entry point
entrypoint!(process_instruction);

//function to handle all instructions to the program
pub fn process_instruction(
    program_id: &Pubkey,      //public key of my program
    accounts: &[AccountInfo], //accounts involved in transaction
    instruction_data: &[u8],  //data passed by caller to program
) -> ProgramResult {
    // msg!("Increment Counter");

    // let account_iter = &mut accounts.iter(); //getting the account iterator
    // let counter_account = next_account_info(account_iter)?; // The first account is the one where we store the counter
    // let payer = next_account_info(account_iter)?; // The second account is the "payer" (who pays for the operation)
    // let system_program = next_account_info(account_iter)?; // The third account is the system program

    // //counter account is owned by our program
    // if counter_account.owner != program_id {
    //     msg!("Counter account is not the owner");
    //     return Err(ProgramError::IncorrectProgramId);
    // }

    // //Deserialize the Counter from the account data
    // let mut counter = Counter::try_from_slice(&counter_account.data.borrow())?;
    // msg!("Current Counter: {}", counter.value);

    // //Increment the counter
    // counter.value += 1;
    // msg!("New counter: {}", counter.value);

    // //Serialize the Counter back to the account data
    // counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;

    // Ok(())

    let instruction = CounterInstruction::try_from_slice(instruction_data)?; //decodes the instruction bytes
    let account_iter = &mut accounts.iter(); //getting the account iterator
    let counter_account = next_account_info(account_iter)?; // The first account is the one where we store the counter

    match instruction {
        CounterInstruction::Initialize => {
            msg!("Instruction: Initialize");

            //checking ownership
            if counter_account.owner != program_id {
                msg!("Counter account is now owned by the program");
                return Err(ProgramError::IncorrectProgramId);
            }

            //initializing the counter with zero
            let counter = Counter { value: 0 };
            counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?; //provide a mutable reference (&mut) to that mutable buffer so the serializer can write into it

            msg!("Counter account has been initialized with value 0");
        }

        CounterInstruction::Increment => {
            msg!("Instruction: Increment");

            if counter_account.owner != program_id {
                msg!("Counter account not owned by the program");
                return Err(ProgramError::IncorrectProgramId);
            }

            let mut counter = Counter::try_from_slice(&counter_account.data.borrow())?;
            msg!("Current Counter: {}", counter.value);

            counter.value += 1;
            msg!("New Counter: {}", counter.value);

            counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;
        }
    }
    Ok(())
}
