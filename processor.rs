use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
    system_instruction,
};

use crate::instruction::SolanaInstruction;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SolanaInstruction::unpack(instruction_data)?;

    match instruction {
        SolanaInstruction::InitializeAccount => {
            msg!("Instruction: InitializeAccount");
            let accounts_iter = &mut accounts.iter();
            let initializer = next_account_info(accounts_iter)?;
            let account_to_initialize = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;

            let rent_exemption = Rent::get()?.minimum_balance(0);

            // Create the account with a balance sufficient to be rent-exempt
            invoke_signed(
                &system_instruction::create_account(
                    initializer.key,
                    account_to_initialize.key,
                    rent_exemption,
                    0, // No data storage, just storing SOL
                    program_id,
                ),
                &[initializer.clone(), account_to_initialize.clone(), system_program.clone()],
                &[],
            )?;
        },
        SolanaInstruction::Deposit { amount } => {
            if amount == 0 {
                return Err(ProgramError::InvalidInstructionData);
            }
            msg!("Instruction: Deposit");
            let accounts_iter = &mut accounts.iter();
            let depositor = next_account_info(accounts_iter)?;
            let account_to_deposit = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;

            invoke(
                &system_instruction::transfer(depositor.key, account_to_deposit.key, amount),
                &[depositor.clone(), account_to_deposit.clone(), system_program.clone()],
            )?;
        },
        SolanaInstruction::WithdrawTenPercent => {
            msg!("Instruction: WithdrawTenPercent");
            let accounts_iter = &mut accounts.iter();
            let withdrawing_account = next_account_info(accounts_iter)?;
            let destination_account = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;

            let balance = **withdrawing_account.lamports.borrow();
            let amount_to_withdraw = balance / 10;
            if amount_to_withdraw == 0 {
                return Err(ProgramError::InsufficientFunds);
            }

            invoke(
                &system_instruction::transfer(withdrawing_account.key, destination_account.key, amount_to_withdraw),
                &[withdrawing_account.clone(), destination_account.clone(), system_program.clone()],
            )?;
        }
    }

    Ok(())
}
