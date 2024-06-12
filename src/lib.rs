use crate::instructions::CounterInstruction;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub mod instructions;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CounterAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entrypoint");

    let instruction = CounterInstruction::unpack(instructions_data)?;

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstruction::Increment(args) => {
            msg!("Instruction: Increment");
            counter_account.counter += args.value;
        }
        CounterInstruction::Decrement(args) => {
            msg!("Instruction: Decrement");
            if args.value > counter_account.counter {
                counter_account.counter = 0;
            } else {
                counter_account.counter -= args.value;
            }
        }
        CounterInstruction::Update(args) => {
            msg!("Instruction: Update");
            counter_account.counter = args.value;
        }
        CounterInstruction::Reset => {
            msg!("Instruction: Reset");
            counter_account.counter = 0;
        }
    };

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    // use solana_program_test::tokio::process;
    // use solana_sdk::address_lookup_table::program;
    use std::{mem, vec};

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let accounts = vec![account];

        let increment_instruction_data: Vec<u8> = vec![0];
        let decrement_instruction_data: Vec<u8> = vec![1];
        let mut update_instruction_data = vec![2];
        update_instruction_data.extend_from_slice(&33u32.to_le_bytes());
        let reset_instruction_data: Vec<u8> = vec![3];

        for (instruction, counter) in vec![
            (&increment_instruction_data, 1),
            (&decrement_instruction_data, 0),
            (&update_instruction_data, 33),
            (&reset_instruction_data, 0),
        ] {
            process_instruction(&program_id, &accounts, instruction).unwrap();
            assert_eq!(
                CounterAccount::try_from_slice(&accounts[0].data.borrow())
                    .unwrap()
                    .counter,
                counter
            );
        }
    }
}
