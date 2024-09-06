use solana_program::program_error::ProgramError;

pub enum SolanaInstruction {
    InitializeAccount,
    Deposit { amount: u64 },
    WithdrawTenPercent,
}

impl SolanaInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => Self::InitializeAccount,
            1 => {
                let amount = rest.get(..8).and_then(|slice| slice.try_into().ok()).map(u64::from_le_bytes).ok_or(ProgramError::InvalidInstructionData)?;
                Self::Deposit { amount }
            },
            2 => Self::WithdrawTenPercent,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
