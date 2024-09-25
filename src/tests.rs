#[cfg(test)]
mod tests {
    use mollusk_svm::{program, Mollusk};
    use solana_program::{instruction::AccountMeta, pubkey};
    use solana_sdk::{account::AccountSharedData, instruction::Instruction, pubkey::Pubkey};

    #[test]
    fn test_deposit() {
        let program_id = pubkey!("9HFegTZnvebYjf9kSa6k3WBm93hRfogWB5B1goUrq1oL");

        let signer = Pubkey::new_unique();
        let vault = Pubkey::try_find_program_address(&[signer.as_ref()], &program_id)
            .unwrap()
            .0;
        let (system_program, system_program_account) = program::system_program();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[&[0x00], &100000u64.to_le_bytes()[..]].concat(),
            vec![
                AccountMeta::new(signer, true),
                AccountMeta::new(vault, false),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let mollusk = Mollusk::new(&program_id, "target/deploy/native_vault_2024");

        let _: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    signer,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (vault, AccountSharedData::new(0, 0, &Pubkey::default())),
                (system_program, system_program_account),
            ],
        );
    }

    #[test]
    fn test_withdraw() {
        let program_id = pubkey!("9HFegTZnvebYjf9kSa6k3WBm93hRfogWB5B1goUrq1oL");

        let signer = Pubkey::new_unique();
        let vault = Pubkey::try_find_program_address(&[signer.as_ref()], &program_id)
            .unwrap()
            .0;
        let (system_program, system_program_account) = program::system_program();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[&[0x01], &100000u64.to_le_bytes()[..]].concat(),
            vec![
                AccountMeta::new(signer, true),
                AccountMeta::new(vault, false),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let mollusk = Mollusk::new(&program_id, "target/deploy/native_vault_2024");

        let _: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    vault,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (signer, AccountSharedData::new(0, 0, &Pubkey::default())),
                (system_program, system_program_account),
            ],
        );
    }
}
