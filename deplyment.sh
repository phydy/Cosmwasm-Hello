source <(curl -sSL https://raw.githubusercontent.com/CosmWasm/testnets/master/malaga-420/defaults.env)

export NODE="--node $RPC"

export TXFLAG="${NODE} --chain-id ${CHAIN_ID} --gas-prices 0.25${FEE_DENOM} --gas auto --gas-adjustment 1.3"

RUSTFLAGS='-C link-arg=-s' cargo wasm

RES=$(~/go/bin/wasmd tx wasm store ./target/wasm32-unknown-unknown/release/ixo_hello_world.wasm --from wallet $TXFLAG -y --output json -b block)

CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')

first_message='{"first_message": {"Hello IXO"}}'

~/go/bin/wasmd tx wasm instantiate $CODE_ID "$first_message" --from wallet --label "hello world" $TXFLAG --no-admin

CONTRACT=$(~/go/bin/wasmd query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')

#change the Hello World
NEW_GREETING='{"changegreeting":{"new_greeting": "Hello World from IXO"}}'

~/go/bin/wasmd tx wasm execute $CONTRACT '$NEW_GREETING' --amount 100umlg --from wallet $TXFLAG -y

GREETING_QUERY='{"getgreeting":{}}'

~go/bin/wasmd query contract-state smart $CONTRACT "$GREETING_QUERY" $NODE --output json 
