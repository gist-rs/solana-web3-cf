use std::sync::Arc;

use solana_web3_wasm::{
    core::client::Web3WasmClient,
    solana_extra_wasm::program::{
        spl_token::state::AccountState,
        spl_token_2022::{id, native_mint, processor::Processor},
    },
    solana_sdk::{
        pubkey::Pubkey,
        signer::{keypair::Keypair, Signer},
    },
};

use super::token::{ExtensionInitializationParams, SendTransaction, Token};

#[derive(Debug, Clone, Copy, Default)]
pub struct ProgramRpcClientSendTransaction;

pub struct TokenContext {
    pub decimals: u8,
    pub mint_authority: Keypair,
    pub token: Token<ProgramRpcClientSendTransaction>,
    // pub token_unchecked: Token<ProgramBanksClientProcessTransaction>,
    pub alice: Keypair,
    pub bob: Keypair,
    pub freeze_authority: Option<Keypair>,
}

pub struct TestContext {
    // pub context: Arc<Mutex<ProgramTestContext>>,
    pub token_context: Option<TokenContext>,
}

pub type TokenResult<T> = anyhow::Result<T>;

impl TestContext {
    pub async fn new() -> Self {
        // let program_test = ProgramTest::new("spl_token_2022", id(), processor!(Processor::process));
        // let context = program_test.start_with_context().await;
        // let context = Arc::new(Mutex::new(context));

        Self {
            // context,
            token_context: None,
        }
    }

    pub async fn init_token_with_mint(
        &mut self,
        extension_init_params: Vec<ExtensionInitializationParams>,
    ) -> TokenResult<()> {
        self.init_token_with_mint_and_freeze_authority(extension_init_params, None)
            .await
    }

    pub async fn init_token_with_freezing_mint(
        &mut self,
        extension_init_params: Vec<ExtensionInitializationParams>,
    ) -> TokenResult<()> {
        let freeze_authority = Keypair::new();
        self.init_token_with_mint_and_freeze_authority(
            extension_init_params,
            Some(freeze_authority),
        )
        .await
    }

    pub async fn init_token_with_mint_and_freeze_authority(
        &mut self,
        extension_init_params: Vec<ExtensionInitializationParams>,
        freeze_authority: Option<Keypair>,
    ) -> TokenResult<()> {
        let mint_account = Keypair::new();
        self.init_token_with_mint_keypair_and_freeze_authority(
            mint_account,
            extension_init_params,
            freeze_authority,
        )
        .await
    }

    pub async fn init_token_with_mint_keypair_and_freeze_authority(
        &mut self,
        mint_account: Keypair,
        extension_init_params: Vec<ExtensionInitializationParams>,
        freeze_authority: Option<Keypair>,
    ) -> TokenResult<()> {
        let payer = Keypair::new();
        let client = Web3WasmClient::new_devnet();

        let decimals: u8 = 9;

        let mint_authority = Keypair::new();
        let mint_authority_pubkey = mint_authority.pubkey();
        let freeze_authority_pubkey = freeze_authority
            .as_ref()
            .map(|authority| authority.pubkey());

        let token = Token::new(
            client,
            &id(),
            &mint_account.pubkey(),
            Some(decimals),
            Arc::new(keypair_clone(&payer)),
        );

        // let token_unchecked =
        //     Token::new(client, &id(), &mint_account.pubkey(), None, Arc::new(payer));

        // token
        //     .create_mint(
        //         &mint_authority_pubkey,
        //         freeze_authority_pubkey.as_ref(),
        //         extension_init_params,
        //         &[&mint_account],
        //     )
        //     .await?;

        self.token_context = Some(TokenContext {
            decimals,
            mint_authority,
            token,
            // token_unchecked,
            alice: Keypair::new(),
            bob: Keypair::new(),
            freeze_authority,
        });

        Ok(())
    }
}

pub(crate) fn keypair_clone(kp: &Keypair) -> Keypair {
    Keypair::from_bytes(&kp.to_bytes()).expect("failed to copy keypair")
}
