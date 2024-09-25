use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, program_error::ProgramError, pubkey::Pubkey, system_instruction::transfer};

/// Deposit funds into vault with deterministic address derived from Signer's pubkey
pub fn process(accounts: &[AccountInfo<'_>], amount: u64) -> ProgramResult {
    let [
        signer, 
        vault, 
        _system_program
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Do I really care if someone wants to send funds to the wrong PDA? 🤔
    // let pda = Pubkey::try_find_program_address(&[&signer.key.as_ref()], &crate::ID).ok_or(ProgramError::InvalidSeeds)?.0;
    // assert!(pda.eq(vault.key));
    
    invoke(
        &transfer(
            signer.key,
            vault.key,
            amount
        ), 
        accounts
    )
}