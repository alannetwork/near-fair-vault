FT_CONTRACT_ID=token.meta.pool.testnet
VAULT_CONTRACT_ID=dev-1675880707627-59536617850341

#near call token.meta.pool.testnet ft_transfer_call '{"receiver_id": "dev-1675782362358-40216332661399", "amount": "10000000000000000000000000", "msg": "{\"action_to_execute\": \"increase_deposit\"}"}' --accountId alan_test.testnet --depositYocto 1 --gas 200000000000000 
#near call token.meta.pool.testnet ft_transfer_call '{"receiver_id": "dev-1675880707627-59536617850341", "amount": "1000000000000000000000000000", "msg": "{\"action_to_execute\": \"increase_deposit\"}"}' --accountId alan_test.testnet --depositYocto 1 --gas 200000000000000 

near view $VAULT_CONTRACT_ID get_ft_token_id

near view $VAULT_CONTRACT_ID get_time_last_deposit

near view $VAULT_CONTRACT_ID get_countdown_period 

near view $VAULT_CONTRACT_ID get_end_date

near view $VAULT_CONTRACT_ID get_vault_balance

near view $VAULT_CONTRACT_ID get_list_deposits '{"from_index":0, "limit":10}'