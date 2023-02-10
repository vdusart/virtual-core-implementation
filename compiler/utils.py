BCC = {
    'B': 0x8,
    'BEQ': 0x9,
    'BNE': 0xa,
    'BLE': 0xb,
    'BGE': 0xc,
    'BL': 0xd,
    'BG': 0xe
}

OPCODES = {
    'AND': 0x0,
    'ORR': 0x1,
    'EOR': 0x2,
    'ADD': 0x3,
    'ADC': 0x4,
    'CMP': 0x5,
    'SUB': 0x6,
    'SBC': 0x7,
    'MOV': 0x8,
    'LSH': 0x9,
    'RSH': 0xa,
}

def int_to_bit_array(number, padding = 4):
    return [x for x in bin(number)[2:].rjust(padding, '0')][::-1]

def check_register_validity(register, msg):
    if register[0] != "r" or int(register[1:]) >= 16:
        raise SyntaxError(f"The {msg} need to be a register between r0 and r16: {register}")