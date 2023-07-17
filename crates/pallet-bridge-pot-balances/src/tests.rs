use crate::mock::*;

#[test]
fn genesis_build() {
    // Prepare some sample data and a config.
    let config = GenesisConfig {
        balances: pallet_balances::GenesisConfig {
            balances: vec![
                (42, 10),
                (43, 20),
                (
                    NativeToEvmSwapBridgePotAccountId::get(),
                    EXISTENTIAL_DEPOSIT_NATIVE,
                ),
            ],
        },
        ..Default::default()
    };

    // Build the state from the config.
    new_test_ext_with(config).execute_with(move || {})
}
