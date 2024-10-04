use solana_program::{

    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey
};

entrypoint!(process_instruction);

fn process_instruction(    
    program_id: &Pubkey,
    account_info: &[AccountInfo],
    instruction_data: &[u8], 
) -> ProgramResult{
    msg!("this is message which i am getting ");
    msg!("process_instruction {} {} and data = {:?}", program_id,account_info.len(),instruction_data);
    Ok(())
} 