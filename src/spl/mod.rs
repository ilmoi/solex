use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::{instruction::transfer_checked, solana_program::program_pack::Pack};

pub fn get_token_mint_accounts() -> Vec<Pubkey> {
    vec![Pubkey::from_str("CepEnf1tjkjh7gdHf89uyXUqm5mPQb2ihfv4ygYjgPyF").unwrap()]
}

pub fn get_associated_token_account(token_mint_addr: &Pubkey, owner_addr: &Pubkey) -> Pubkey {
    get_associated_token_address(owner_addr, token_mint_addr)
}

pub fn get_token_balance(client: &RpcClient, token_assoc_addr: &Pubkey) -> u64 {
    match client.get_account(token_assoc_addr) {
        Ok(account) => {
            spl_token::state::Account::unpack(&account.data)
                .unwrap()
                .amount
        }
        Err(_) => 0, //if the assoc account isn't created, this will error and so we reutrn 0 manually
    }
}

pub fn transfer_tokens(
    client: &RpcClient,
    token_mint_addr: &Pubkey,
    source: &Keypair,
    dest_addr: &Pubkey,
    amount: u64,
) {
    let source_addr = source.pubkey();
    let token_source_addr = get_associated_token_account(&token_mint_addr, &source_addr);
    let token_dest_addr = get_associated_token_account(&token_mint_addr, &dest_addr);

    let mint_account = client.get_account(token_mint_addr).unwrap();
    let decimals = spl_token::state::Mint::unpack(&mint_account.data)
        .unwrap()
        .decimals;

    let transfer_ix = transfer_checked(
        &spl_token::id(),
        &token_source_addr,
        &token_mint_addr,
        &token_dest_addr,
        &source_addr,
        &[], //not a multisig account that's why this is empty
        amount,
        decimals,
    )
    .unwrap();

    // if destination assoc token acc doesn't exist, we need to create it
    // (!) NOTE: this assumes that the address passed is always the SOL address, NOT the associated program address!
    let instructions = match client.get_account(&token_dest_addr) {
        Ok(_) => {
            vec![transfer_ix]
        }
        Err(_) => {
            let create_dest_token_acc_ix =
                spl_associated_token_account::create_associated_token_account(
                    &source_addr,
                    &dest_addr,
                    &token_mint_addr,
                );
            vec![create_dest_token_acc_ix, transfer_ix]
        }
    };

    let tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&source_addr),
        &[source],
        client.get_recent_blockhash().unwrap().0,
    );

    let tx_hash = client.send_and_confirm_transaction(&tx).unwrap();
    println!(
        "transfer of {} units of TOKEN:{} from {} to {} successfully executed, {}",
        amount,
        token_mint_addr,
        source.pubkey(),
        dest_addr,
        tx_hash
    );
}
