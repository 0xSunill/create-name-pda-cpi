use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::{ ProgramResult},
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction::create_account,
};
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let pda = next_account_info(iter)?;
    let user_acc = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;
    let seeds = &[user_acc.key.as_ref(), b"user"];
    let (pda_pub_key, bump) = Pubkey::find_program_address(seeds, program_id);
    let ix = create_account(user_acc.key, pda.key, 1000000000, 8, program_id);

    let signer_seeds: &[&[u8]] = &[
        user_acc.key.as_ref(),
        b"user",
        &[bump],
    ];
    
    invoke_signed(
        &ix,
        &[user_acc.clone(), pda.clone(), system_program.clone()],
        &[signer_seeds],
    )?;
    
    Ok(())
}
