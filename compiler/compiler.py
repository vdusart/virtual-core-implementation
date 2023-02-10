from utils import *

file_name = '../programs/file.s'
f = open(file_name, 'r')
lines = f.readlines()
f.close()

def instruction(args):
    tmp = ['0'] * 32
    current_index = 0
    # Set IV Flag
    if True in [a.isdigit() for a in args]:
        tmp[24] = '1'
    
    # Set opcode
    tmp[20:24] = int_to_bit_array(OPCODES[args[current_index]])
    opcode = args[current_index]

    # Set destination
    if not opcode == "CMP":
        current_index += 1
        dest = args[current_index]
        check_register_validity(dest, "destination")
        tmp[8:12] = int_to_bit_array(int(dest[1:]))

    if opcode != "MOV":
        current_index += 1
        # Set first operand
        check_register_validity(args[current_index], "first operand")
        tmp[16:20] = int_to_bit_array(int(args[current_index][1:]))
    
    current_index += 1
    # Set second operand or immediate value
    if tmp[24] == '1':
        tmp[0:8] = int_to_bit_array(int(args[current_index]), 8)
    else:
        check_register_validity(args[current_index], "second operand")
        tmp[12:16] = int_to_bit_array(int(args[current_index][1:]))

    return tmp

def branch(args):
    tmp = ['0'] * 32

    tmp[28:32] = int_to_bit_array(BCC[args[0]])
    tmp[27] = '1' if int(args[1]) < 1 else '0'
    tmp[0:27] = int_to_bit_array(abs(int(args[1])), 27)
    return tmp

binary = []

for l in lines:
    l = l.strip()
    if l[0] == ';': continue
    args = l.replace(',', '').split(' ')
    if args[0] in OPCODES:
        tmp = instruction(args)
    elif args[0] in BCC:
        tmp = branch(args)
    else:
        raise Exception(f"Unknown operation: {l}")
    binary += tmp[::-1]

res = [int(''.join(binary[i:i+8]), 2) for i in range(0, len(binary), 8)]

f = open(file_name.replace(".s", ".bin"), 'wb')
f.write(bytearray(res))
f.close()