FT_CONTRACT_ID=token.meta.pool.testnet
VAUL_CONTRACT_ID=dev-1675782362358-40216332661399

near call $FT_CONTRACT_ID storage_deposit '{"account_id": "dev-1675782362358-40216332661399"}' --accountId alan_test.testnet --amount 0.01

near call token.meta.pool.testnet ft_transfer_call '{"receiver_id": "dev-1675782362358-40216332661399", "amount": "10000000000000000000000000", "msg": "{\"action_to_execute\": \"increase_deposit\"}"}' --accountId alan_test.testnet --depositYocto 1 --gas 200000000000000 

near view $VAUL_CONTRACT_ID get_ft_token_id

near view $VAUL_CONTRACT_ID get_time_last_deposit

near view $VAUL_CONTRACT_ID get_countdown_period 

near view $VAUL_CONTRACT_ID get_end_date

near view $VAUL_CONTRACT_ID get_vault_balance