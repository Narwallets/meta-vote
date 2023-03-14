#CONTRACT_NAME="metavote.testnet"
#ACCOUNT_ID="test123512.testnet"
CONTRACT_NAME="token.meta.pool.testnet"
ACCOUNT_ID="dev-1678754191868-11103308478746"
AMOUNT="1000000000000000000000000"

#MESSAGE="30"
MESSAGE='{\"lock_period\":\"30\",\"receiver_id\":\"alotaco.testnet\"}'

#NEAR_ENV=testnet near call $CONTRACT_NAME ft_transfer_call '{"receiver_id": "'${ACCOUNT_ID}'", "amount": "'$AMOUNT'", "msg":  {"lock_period": "'$MESSAGE'","receiver_id": "alan_test.testnet"} }' --accountId $ACCOUNT_ID
NEAR_ENV=testnet near call $CONTRACT_NAME ft_transfer_call '{"receiver_id": "'${ACCOUNT_ID}'", "amount": "'$AMOUNT'",  "msg": "'$MESSAGE'"}' --accountId alan_test.testnet --depositYocto 1 --gas 300000000000000

# near call metavote.testnet ft_transfer_call '{"receiver_id": "test123512.testnet", "amount": "10", "msg": "30" }' --accountId test123512.testnet
