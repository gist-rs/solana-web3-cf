use std::convert::TryInto;

use solana_web3_wasm::{
    solana_client_wasm::WasmClient,
    solana_extra_wasm::program::{
        spl_associated_token_account::{self, instruction::create_associated_token_account},
        spl_token,
        spl_token_2022::extension::transfer_fee::{TransferFee, TransferFeeConfig},
    },
    solana_sdk::{
        program_option::COption, pubkey::Pubkey, signature::Keypair, signer::Signer,
        transaction::Transaction,
    },
};

use super::{
    program_test::{TestContext, TokenContext},
    token::{ExtensionInitializationParams, Token},
};

/// Send transaction to validator using `BanksClient::process_transaction`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ProgramBanksClientProcessTransaction;

const TEST_MAXIMUM_FEE: u64 = 10_000_000;
const TEST_FEE_BASIS_POINTS: u16 = 250;

fn test_transfer_fee() -> TransferFee {
    TransferFee {
        epoch: 0.into(),
        transfer_fee_basis_points: TEST_FEE_BASIS_POINTS.into(),
        maximum_fee: TEST_MAXIMUM_FEE.into(),
    }
}

fn test_transfer_fee_config() -> TransferFeeConfig {
    let transfer_fee = test_transfer_fee();
    TransferFeeConfig {
        transfer_fee_config_authority: COption::Some(Pubkey::new_unique()).try_into().unwrap(),
        withdraw_withheld_authority: COption::Some(Pubkey::new_unique()).try_into().unwrap(),
        withheld_amount: 0.into(),
        older_transfer_fee: transfer_fee,
        newer_transfer_fee: transfer_fee,
    }
}

struct TransferFeeConfigWithKeypairs {
    transfer_fee_config: TransferFeeConfig,
    transfer_fee_config_authority: Keypair,
    withdraw_withheld_authority: Keypair,
}

fn test_transfer_fee_config_with_keypairs() -> TransferFeeConfigWithKeypairs {
    let transfer_fee = test_transfer_fee();
    let transfer_fee_config_authority = Keypair::new();
    let withdraw_withheld_authority = Keypair::new();
    let transfer_fee_config = TransferFeeConfig {
        transfer_fee_config_authority: COption::Some(transfer_fee_config_authority.pubkey())
            .try_into()
            .unwrap(),
        withdraw_withheld_authority: COption::Some(withdraw_withheld_authority.pubkey())
            .try_into()
            .unwrap(),
        withheld_amount: 0.into(),
        older_transfer_fee: transfer_fee,
        newer_transfer_fee: transfer_fee,
    };
    TransferFeeConfigWithKeypairs {
        transfer_fee_config,
        transfer_fee_config_authority,
        withdraw_withheld_authority,
    }
}

struct TokenWithAccounts {
    context: TestContext,
    // token: Token<ProgramBanksClientProcessTransaction>,
    // token_unchecked: Token<ProgramBanksClientProcessTransaction>,
    transfer_fee_config: TransferFeeConfig,
    withdraw_withheld_authority: Keypair,
    freeze_authority: Keypair,
    alice: Keypair,
    alice_account: Pubkey,
    bob_account: Pubkey,
}

async fn create_mint_with_accounts(alice_amount: u64) -> TokenWithAccounts {
    let TransferFeeConfigWithKeypairs {
        transfer_fee_config_authority,
        withdraw_withheld_authority,
        transfer_fee_config,
        ..
    } = test_transfer_fee_config_with_keypairs();
    let mut context = TestContext::new().await;
    let transfer_fee_basis_points = u16::from(
        transfer_fee_config
            .newer_transfer_fee
            .transfer_fee_basis_points,
    );
    let maximum_fee = u64::from(transfer_fee_config.newer_transfer_fee.maximum_fee);
    context
        .init_token_with_freezing_mint(vec![ExtensionInitializationParams::TransferFeeConfig {
            transfer_fee_config_authority: transfer_fee_config_authority.pubkey().into(),
            withdraw_withheld_authority: withdraw_withheld_authority.pubkey().into(),
            transfer_fee_basis_points,
            maximum_fee,
        }])
        .await
        .unwrap();
    let TokenContext {
        mint_authority,
        freeze_authority,
        token,
        // token_unchecked,
        alice,
        bob,
        ..
    } = context.token_context.take().unwrap();

    // token account is self-owned just to test another case
    token
        .create_auxiliary_token_account(&alice, &alice.pubkey())
        .await
        .unwrap();
    let alice_account = alice.pubkey();
    let bob_account = Keypair::new();
    // token
    //     .create_auxiliary_token_account(&bob_account, &bob.pubkey())
    //     .await
    //     .unwrap();
    let bob_account = bob_account.pubkey();

    // // mint tokens
    // token
    //     .mint_to(
    //         &alice_account,
    //         &mint_authority.pubkey(),
    //         alice_amount,
    //         &[&mint_authority],
    //     )
    //     .await
    //     .unwrap();
    TokenWithAccounts {
        context,
        // token,
        // token_unchecked,
        transfer_fee_config,
        withdraw_withheld_authority,
        freeze_authority: freeze_authority.unwrap(),
        alice,
        alice_account,
        bob_account,
    }
}

#[cfg(test)]
mod test {
    use solana_web3_wasm::{core::client::Web3WasmClient, solana_client_wasm::WasmClient};

    #[tokio::test]
    async fn test_create_mint_token22_memo_fee() {
        let client: WasmClient = Web3WasmClient::new_devnet();

        todo!();
    }
}
