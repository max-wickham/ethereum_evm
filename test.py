# # pip install py-evm


# from eth_keys import keys
# from eth_utils import decode_hex
# from eth import constants
# from eth.chains.mainnet import MainnetChain
# from eth.db.atomic import AtomicDB
# from eth_utils import to_wei, encode_hex

# import json
# from eth.vm.forks.berlin.headers import configure_berlin_header


# with open('tests/official_tests/tests/GeneralStateTests/VMTests/vmArithmeticTest/add.json') as f:
#     test = json.load(f)['add']
# #
# # print(test['pre'])
# for state in test['pre'].values():
#     print(state['code'])
#     print(bytes.fromhex(state['code'][2:]) if state['code'] != '0x' else b'')
# GENESIS_STATE = {
#     bytes.fromhex(address[2:]): {
#         "balance": int(state['balance'], base=16),
#         "nonce": int(state['nonce'], base=16),
#         "code": decode_hex(state['code']),
#         "storage": state['storage'],
#         "is_cold":True
#     } for address,state in test['pre'].items()
# }


# GENESIS_PARAMS = {

#     'difficulty': int(test['env']['currentDifficulty'], base=16),

# }

# chain = MainnetChain.from_genesis(AtomicDB(), GENESIS_PARAMS, GENESIS_STATE)

# current_vm = chain.get_vm()
# state = current_vm.state
# print(state.get_balance(bytes.fromhex('0000000000000000000000000000000000000100')))


# post = test['post']['Berlin'][0]
# tx = current_vm.create_unsigned_transaction(
#     nonce = int.from_bytes(decode_hex(test['transaction']['nonce'])),
#     gas_price = int.from_bytes(decode_hex(test['transaction']['gasPrice'])),
#     gas = int.from_bytes(decode_hex(test['transaction']['gasLimit'][post['indexes']['gas']])),
#     to = bytes.fromhex(test['transaction']['to'][2:]),
#     value = int.from_bytes(decode_hex(test['transaction']['value'][post['indexes']['value']])),
#     data = bytes.fromhex(test['transaction']['data'][post['indexes']['data']][2:]),
# )
# key = keys.PrivateKey(decode_hex(test['transaction']['secretKey']))
# signed_tx = tx.as_signed_transaction(key)

# current_balances = []
# print('=========================')
# for index, key in enumerate(test['pre'].keys()):
#     print(state.get_balance(bytes.fromhex(key[2:])))
#     current_balances.append(state.get_balance(bytes.fromhex(key[2:])))

# receipt, compute = current_vm.apply_transaction(configure_berlin_header(current_vm,
#                                                      gas_limit = int.from_bytes(decode_hex(test['env']['currentGasLimit']))), signed_tx)
# print(receipt)
# print(compute)

# def print_gas_breakdown(computation):
#     print("Gas Breakdown:")
#     print(f"    Gas Used: {computation.get_gas_used()}")
#     # print(f"    Base Gas: {computation.ge()}")
#     # print(f"    Gas Paid: {computation.get_gas_paid()}")
#     print(f"    Gas Refunded: {computation.get_gas_refund()}")
# print_gas_breakdown(compute)

# print('=========================')

# balances = [
# 838137708091124174,
# 838137708091124174,
# 838137708091124174,
# 838137708091124174,
# 838137708091124174,
# 838137708090685793,
# 838137708091124175,
# # 838137708091124175,
# ]
# for index, key in enumerate(test['pre'].keys()):
#     print()
#     print(state.get_balance(bytes.fromhex(key[2:])))
#     # assert(balances[index] == state.get_balance(bytes.fromhex(key[2:])))
#     print("Difference",balances[index] - state.get_balance(bytes.fromhex(key[2:])))
#     print("Actual Change", state.get_balance(bytes.fromhex(key[2:])) - current_balances[index])
#     print("Measured Change", balances[index] - current_balances[index])

# # 838137708091124174
# # 838137708091124174
# # 838137708091124174
# # 838137708091124174
# # 838137708091124174
# # 838137708091124173
# # 838137708091122755



# # PUSH_1
# #### Gas Usage: 3
# # PUSH_1
# #### Gas Usage: 3
# # PUSH_1
# #### Gas Usage: 3
# # PUSH_1
# #### Gas Usage: 3
# # PUSH_1
# #### Gas Usage: 3
# # PUSH_1
# #### Gas Usage: 3
# # CALLDATALOAD
# #### Gas Usage: 3
# #### Gas Usage: 3
# # ADD
# #### Gas Usage: 3
# #### Gas Usage: 3
# # Make Call
# # PUSH_32
# #### Gas Usage: 3
# # PUSH_32
# #### Gas Usage: 3
# # ADD
# #### Gas Usage: 3
# # PUSH_1
# #### Gas Usage: 3
# # SSTORE
# #### Gas Usage: 20000
# # STOP
# # Gas Usage: 22612
# # STOP
# # Gas Usage: 43642
# # Eth Usage: 436420


# #### GAS CONSUMPTION: 3 -> 79978597 (PUSH1)
# #### GAS CONSUMPTION: 3 -> 79978594 (PUSH1)
# #### GAS CONSUMPTION: 3 -> 79978591 (PUSH1)
# #### GAS CONSUMPTION: 3 -> 79978588 (PUSH1)
# #### GAS CONSUMPTION: 3 -> 79978585 (PUSH1)
# #### GAS CONSUMPTION: 3 -> 79978582 (PUSH1)
# #### GAS CONSUMPTION: 3 -> 79978579 (CALLDATALOAD)
# #### GAS CONSUMPTION: 3 -> 79978576 (PUSH2)
# #### GAS CONSUMPTION: 3 -> 79978573 (ADD)
# #### GAS CONSUMPTION: 3 -> 79978570 (PUSH3)
# # GAS CONSUMPTION: 40 -> 79978530 (CALL)
# # GAS CONSUMPTION: 16777215 -> 63201315 (CALL)
# #### GAS CONSUMPTION: 3 -> 16777212 (PUSH32)
# #### GAS CONSUMPTION: 3 -> 16777209 (PUSH32)
# #### GAS CONSUMPTION: 3 -> 16777206 (ADD)
# #### GAS CONSUMPTION: 3 -> 16777203 (PUSH1)
# # GAS CONSUMPTION: 0 -> 16777203 (SSTORE)
# #### GAS CONSUMPTION: 20000 -> 16757203 (SSTORE: 0x0000000000000000000000000000000000000100[0] -> 115792089237316195423570985008687907853269984665640564039457584007913129639934 (0))
# # GAS CONSUMPTION: 0 -> 16757203 (STOP)
# # GAS CONSUMPTION: 0 -> 79958518 (STOP)


# # Gas Usage: 3
# # Gas Usage: 3
# # Gas Usage: 3
# # Gas Usage: 3
# # Gas Usage: 3
# # Gas Usage: 3
# # Gas Usage: 3
# # Gas Usage: 3
# # Gas Usage: 3
# # Gas Usage: 3
# # Gas Usage: 20012
# # 410390
# # missing 403 gas and 40 gas from call?

# # GAS CONSUMPTION: 3 -> (PUSH1)
# # GAS CONSUMPTION: 3 -> (PUSH1)
# # GAS CONSUMPTION: 3 -> (PUSH1)
# # GAS CONSUMPTION: 3 -> (PUSH1)
# # GAS CONSUMPTION: 3 -> (PUSH1)
# # GAS CONSUMPTION: 3 -> (PUSH1)
# # GAS CONSUMPTION: 3 -> (CALLDATALOAD)
# # GAS CONSUMPTION: 3 -> (PUSH2)
# # GAS CONSUMPTION: 3 -> (ADD)
# # GAS CONSUMPTION: 3 -> (PUSH3)
# # GAS CONSUMPTION: 20012 -> (STORE)
# # GAS CONSUMPTION: 40 -> (CALL)
# # 410760

# # missing 406 gas?




# x = '''
# [(0x0000000000000000000000000000000000000100, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xce\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0\x15\xb8\x1c\xad\x95\xd9\xa1\xbc@p\x87&!\x1e\xb3\xd60#\xf8\xe6\xe1O\xd7E\x9dL8?\xc7].\xef"), (0x0000000000000000000000000000000000000101, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xce\xa0\xad\x83\"\x17\xa1\xc3\xac\xdd+\xc7\xebu8\x08^3\xe4\x17-\x14\x90\xa0\xb6\xdd_+\x9a\"f\x1b/L\xa0\x88\xc3o$\xe3D\xd4\t\xbc\xc4*Yh\x9aq\x86v\xdc\xba\x10\xb9\xaf./\xa5\xe87\xae\xb3;\x8c\xd0"), (0x0000000000000000000000000000000000000102, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xce\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0wc\xc5\xc3v\xc7\x17\xfdn\xe3\xb1;e\xd4YK\"=>\x1d1J\xd7\xb5\x9c\0>\xca\n\x16\xf6\xef"), (0x0000000000000000000000000000000000000103, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xce\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0E\x02\x8f\x8d\x8dj\xeaO\x0c\xd7\xd5\x7f\xc3\xe7\xbf\x94S\\l\x02)j\x8dc\x07\x19\x8ar\xaaU\xf9V"), (0x0000000000000000000000000000000000000104, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xce\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0\x81\xc6\x83\xa1\xea\xe1\x16\xd0\x83\xc3\xbd\xb8fCj\x08\xd8tF\x01\x05[\x85\xa2\x849[\x8ec\xa0x\xb8"), (0x2adc25665018aa1fe0e6bc666dac8fc2697ff9ba, b"\xf8G\x80\x83\x07\x02\xc4\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0\xc5\xd2F\x01\x86\xf7#<\x92~}\xb2\xdc\xc7\x03\xc0\xe5\0\xb6S\xca\x82';{\xfa\xd8\x04]\x85\xa4p"), (0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b, b"\xf8L\x01\x88\x0b\xa1\xa9\xce\x0b\x9a\xa7\t\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0\xc5\xd2F\x01\x86\xf7#<\x92~}\xb2\xdc\xc7\x03\xc0\xe5\0\xb6S\xca\x82';{\xfa\xd8\x04]\x85\xa4p"), (0xcccccccccccccccccccccccccccccccccccccccc, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xcf\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0W\x9bP\xb0F\x0f\x1e\x19A\xf1\xd5\xcc\x83&H6c\xa6\x80\xb0\x90\xb9\x12yib}tr\x93e8")]
# '''

# y = '''
# [(0x0000000000000000000000000000000000000100, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xce\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0\x15\xb8\x1c\xad\x95\xd9\xa1\xbc@p\x87&!\x1e\xb3\xd60#\xf8\xe6\xe1O\xd7E\x9dL8?\xc7].\xef"), (0x0000000000000000000000000000000000000101, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xce\xa0\xdb\x915(\xa1\xcc\xbcD\x03\xae\x18\xeb\x828c\xb1\x07\xa1\xc0\x1c\0\x8f\x0e\xbc\xcb\xd8\x96\xe0<_\x97\x92\xa0\x88\xc3o$\xe3D\xd4\t\xbc\xc4*Yh\x9aq\x86v\xdc\xba\x10\xb9\xaf./\xa5\xe87\xae\xb3;\x8c\xd0"), (0x0000000000000000000000000000000000000102, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xce\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0wc\xc5\xc3v\xc7\x17\xfdn\xe3\xb1;e\xd4YK\"=>\x1d1J\xd7\xb5\x9c\0>\xca\n\x16\xf6\xef"), (0x0000000000000000000000000000000000000103, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xce\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0E\x02\x8f\x8d\x8dj\xeaO\x0c\xd7\xd5\x7f\xc3\xe7\xbf\x94S\\l\x02)j\x8dc\x07\x19\x8ar\xaaU\xf9V"), (0x0000000000000000000000000000000000000104, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xce\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0\x81\xc6\x83\xa1\xea\xe1\x16\xd0\x83\xc3\xbd\xb8fCj\x08\xd8tF\x01\x05[\x85\xa2\x849[\x8ec\xa0x\xb8"), (0x2adc25665018aa1fe0e6bc666dac8fc2697ff9ba, b"\xf8G\x80\x83\x07\x02\xc4\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0\xc5\xd2F\x01\x86\xf7#<\x92~}\xb2\xdc\xc7\x03\xc0\xe5\0\xb6S\xca\x82';{\xfa\xd8\x04]\x85\xa4p"), (0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b, b"\xf8L\x01\x88\x0b\xa1\xa9\xce\x0b\x9a\xa7\t\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0\xc5\xd2F\x01\x86\xf7#<\x92~}\xb2\xdc\xc7\x03\xc0\xe5\0\xb6S\xca\x82';{\xfa\xd8\x04]\x85\xa4p"), (0xcccccccccccccccccccccccccccccccccccccccc, b"\xf8L\x80\x88\x0b\xa1\xa9\xce\x0b\xa1\xa9\xcf\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0W\x9bP\xb0F\x0f\x1e\x19A\xf1\xd5\xcc\x83&H6c\xa6\x80\xb0\x90\xb9\x12yib}tr\x93e8")]
# '''

# print(x == y)




print("0x0000000000000000000000000000000000000000000000000000000000000000" == "0x0000000000000000000000000000000000000000000000000000000000000000")
