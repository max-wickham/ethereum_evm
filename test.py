# pip install py-evm


from eth_keys import keys
from eth_utils import decode_hex
from eth import constants
from eth.chains.mainnet import MainnetChain
from eth.db.atomic import AtomicDB
from eth_utils import to_wei, encode_hex

import json
from eth.vm.forks.berlin.headers import configure_berlin_header


with open('tests/official_tests/tests/GeneralStateTests/VMTests/vmArithmeticTest/add.json') as f:
    test = json.load(f)['add']
#
# print(test['pre'])
for state in test['pre'].values():
    print(state['code'])
    print(bytes.fromhex(state['code'][2:]) if state['code'] != '0x' else b'')
GENESIS_STATE = {
    bytes.fromhex(address[2:]): {
        "balance": int(state['balance'], base=16),
        "nonce": int(state['nonce'], base=16),
        "code": decode_hex(state['code']),
        "storage": state['storage'],
        "is_cold":True
    } for address,state in test['pre'].items()
}


GENESIS_PARAMS = {

    'difficulty': int(test['env']['currentDifficulty'], base=16),

}

chain = MainnetChain.from_genesis(AtomicDB(), GENESIS_PARAMS, GENESIS_STATE)

current_vm = chain.get_vm()
state = current_vm.state
print(state.get_balance(bytes.fromhex('0000000000000000000000000000000000000100')))


post = test['post']['Berlin'][0]
tx = current_vm.create_unsigned_transaction(
    nonce = int.from_bytes(decode_hex(test['transaction']['nonce'])),
    gas_price = int.from_bytes(decode_hex(test['transaction']['gasPrice'])),
    gas = int.from_bytes(decode_hex(test['transaction']['gasLimit'][post['indexes']['gas']])),
    to = bytes.fromhex(test['transaction']['to'][2:]),
    value = int.from_bytes(decode_hex(test['transaction']['value'][post['indexes']['value']])),
    data = bytes.fromhex(test['transaction']['data'][post['indexes']['data']][2:]),
)
key = keys.PrivateKey(decode_hex(test['transaction']['secretKey']))
signed_tx = tx.as_signed_transaction(key)

current_balances = []
print('=========================')
for index, key in enumerate(test['pre'].keys()):
    print(state.get_balance(bytes.fromhex(key[2:])))
    current_balances.append(state.get_balance(bytes.fromhex(key[2:])))

receipt, compute = current_vm.apply_transaction(configure_berlin_header(current_vm,
                                                     gas_limit = int.from_bytes(decode_hex(test['env']['currentGasLimit']))), signed_tx)
print(receipt)
print(compute)

def print_gas_breakdown(computation):
    print("Gas Breakdown:")
    print(f"    Gas Used: {computation.get_gas_used()}")
    # print(f"    Base Gas: {computation.ge()}")
    # print(f"    Gas Paid: {computation.get_gas_paid()}")
    print(f"    Gas Refunded: {computation.get_gas_refund()}")
print_gas_breakdown(compute)

print('=========================')

balances = [
838137708091124174,
838137708091124174,
838137708091124174,
838137708091124174,
838137708091124174,
838137708090685793,
838137708091124175,
# 838137708091124175,
]
for index, key in enumerate(test['pre'].keys()):
    print()
    print(state.get_balance(bytes.fromhex(key[2:])))
    # assert(balances[index] == state.get_balance(bytes.fromhex(key[2:])))
    print("Difference",balances[index] - state.get_balance(bytes.fromhex(key[2:])))
    print("Actual Change", state.get_balance(bytes.fromhex(key[2:])) - current_balances[index])
    print("Measured Change", balances[index] - current_balances[index])

# 838137708091124174
# 838137708091124174
# 838137708091124174
# 838137708091124174
# 838137708091124174
# 838137708091124173
# 838137708091122755



# PUSH_1
#### Gas Usage: 3
# PUSH_1
#### Gas Usage: 3
# PUSH_1
#### Gas Usage: 3
# PUSH_1
#### Gas Usage: 3
# PUSH_1
#### Gas Usage: 3
# PUSH_1
#### Gas Usage: 3
# CALLDATALOAD
#### Gas Usage: 3
#### Gas Usage: 3
# ADD
#### Gas Usage: 3
#### Gas Usage: 3
# Make Call
# PUSH_32
#### Gas Usage: 3
# PUSH_32
#### Gas Usage: 3
# ADD
#### Gas Usage: 3
# PUSH_1
#### Gas Usage: 3
# SSTORE
#### Gas Usage: 20000
# STOP
# Gas Usage: 22612
# STOP
# Gas Usage: 43642
# Eth Usage: 436420


#### GAS CONSUMPTION: 3 -> 79978597 (PUSH1)
#### GAS CONSUMPTION: 3 -> 79978594 (PUSH1)
#### GAS CONSUMPTION: 3 -> 79978591 (PUSH1)
#### GAS CONSUMPTION: 3 -> 79978588 (PUSH1)
#### GAS CONSUMPTION: 3 -> 79978585 (PUSH1)
#### GAS CONSUMPTION: 3 -> 79978582 (PUSH1)
#### GAS CONSUMPTION: 3 -> 79978579 (CALLDATALOAD)
#### GAS CONSUMPTION: 3 -> 79978576 (PUSH2)
#### GAS CONSUMPTION: 3 -> 79978573 (ADD)
#### GAS CONSUMPTION: 3 -> 79978570 (PUSH3)
# GAS CONSUMPTION: 40 -> 79978530 (CALL)
# GAS CONSUMPTION: 16777215 -> 63201315 (CALL)
#### GAS CONSUMPTION: 3 -> 16777212 (PUSH32)
#### GAS CONSUMPTION: 3 -> 16777209 (PUSH32)
#### GAS CONSUMPTION: 3 -> 16777206 (ADD)
#### GAS CONSUMPTION: 3 -> 16777203 (PUSH1)
# GAS CONSUMPTION: 0 -> 16777203 (SSTORE)
#### GAS CONSUMPTION: 20000 -> 16757203 (SSTORE: 0x0000000000000000000000000000000000000100[0] -> 115792089237316195423570985008687907853269984665640564039457584007913129639934 (0))
# GAS CONSUMPTION: 0 -> 16757203 (STOP)
# GAS CONSUMPTION: 0 -> 79958518 (STOP)


# Gas Usage: 3
# Gas Usage: 3
# Gas Usage: 3
# Gas Usage: 3
# Gas Usage: 3
# Gas Usage: 3
# Gas Usage: 3
# Gas Usage: 3
# Gas Usage: 3
# Gas Usage: 3
# Gas Usage: 20012
# 410390
# missing 403 gas and 40 gas from call?

# GAS CONSUMPTION: 3 -> (PUSH1)
# GAS CONSUMPTION: 3 -> (PUSH1)
# GAS CONSUMPTION: 3 -> (PUSH1)
# GAS CONSUMPTION: 3 -> (PUSH1)
# GAS CONSUMPTION: 3 -> (PUSH1)
# GAS CONSUMPTION: 3 -> (PUSH1)
# GAS CONSUMPTION: 3 -> (CALLDATALOAD)
# GAS CONSUMPTION: 3 -> (PUSH2)
# GAS CONSUMPTION: 3 -> (ADD)
# GAS CONSUMPTION: 3 -> (PUSH3)
# GAS CONSUMPTION: 20012 -> (STORE)
# GAS CONSUMPTION: 40 -> (CALL)
# 410760

# missing 406 gas?
