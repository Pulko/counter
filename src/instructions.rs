use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateInstructionArgs {
    pub value: u32,
}

pub enum CounterInstruction {
    Increment(UpdateInstructionArgs),
    Decrement(UpdateInstructionArgs),
    Update(UpdateInstructionArgs),
    Reset,
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => Self::Increment(UpdateInstructionArgs::try_from_slice(rest)?),
            1 => Self::Decrement(UpdateInstructionArgs::try_from_slice(rest)?),
            2 => Self::Update(UpdateInstructionArgs::try_from_slice(rest)?),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
