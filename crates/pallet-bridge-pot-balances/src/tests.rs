use crate::mock::*;

#[test]
fn genesis_build() {
    // Prepare some sample data and a config.
    let config = GenesisConfig {
        balances: pallet_balances::GenesisConfig {
            balances: vec![
                (42, EXISTENTIAL_DEPOSIT_NATIVE + 50),
                (43, EXISTENTIAL_DEPOSIT_NATIVE + 100),
                (
                    NativeToEvmSwapBridgePotAccountId::get(),
                    EXISTENTIAL_DEPOSIT_NATIVE,
                ),
            ],
        },
        evm: EVMConfig {
            accounts: vec![(
                EvmToNativeSwapBridgePotAccountId::get(),
                fp_evm::GenesisAccount {
                    balance: EXISTENTIAL_DEPOSIT_EVM.into(),
                    code: Default::default(),
                    nonce: Default::default(),
                    storage: Default::default(),
                },
            )]
            .into_iter()
            .collect(),
        },
        ..Default::default()
    };

    // Build the state from the config.
    new_test_ext_with(config).execute_with(move || {})
}
