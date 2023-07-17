use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};
use spl_token::state::Account;

entrypoint!(process_instruction);
fn process_instruction(_program_id: &Pubkey, accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
    // Создаем новый аккаунт для токена TEST
    let account = Account {
        mint: accounts[0].key.to_bytes(),
        owner: accounts[1].key.to_bytes(),
        ..Account::default()
    };

    // Инициализируем новый аккаунт
    Account::pack(account, &mut accounts[2].data.borrow_mut())?;

    Ok(())
}
