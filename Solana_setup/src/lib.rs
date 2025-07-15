use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // The public key of the account where this program is deployed
    accounts: &[AccountInfo], // The accounts that will be accessed by the program
    instruction_data: &[u8], // The instruction data
) -> ProgramResult {
    // Log a message to the blockchain
    msg!("Hello, World! This is my first Solana program!");
    msg!("Program ID: {}", program_id);
    msg!("Number of accounts: {}", accounts.len());
    msg!("Instruction data length: {}", instruction_data.len());

    // Gracefully exit the program
    Ok(())
} 