from keywords import OPERATION_CODES, BRANCHING_CODES


def check_register_validity(register, position):
    if register[0] != 'r' or int(register[1:]) >= 16:
        raise Exception(f'Unknown {position} [{register}]. Must be a register ri with i in [0;15].')


def int_to_bit_array(number, padding=4):
    return [x for x in bin(number)[2:].rjust(padding, '0')][::-1]


def encode_operation(instruction):
    encoding = ['0'] * 32  # Initialise empty encoding
    current_index = 0  # Cursor to iterate over the statement

    # Set Immediate Value Flag
    if True in [operand.isdigit() for operand in instruction]:
        encoding[24] = '1'

    # Encode opcode
    opcode = instruction[current_index]
    encoding[20:24] = int_to_bit_array(OPERATION_CODES[opcode])

    # Encode destination
    if not opcode == 'CMP':
        current_index += 1
        destination = instruction[current_index]
        check_register_validity(destination, 'destination')
        encoding[8:12] = int_to_bit_array(int(destination[1:]))

    # Set first operand
    if opcode != 'MOV':
        current_index += 1
        check_register_validity(instruction[current_index], 'first operand')
        encoding[16:20] = int_to_bit_array(int(instruction[current_index][1:]))

    # Encode second operand or immediate value
    current_index += 1
    if encoding[24] == '1':
        immediate_value = int(instruction[current_index])
        if immediate_value < 0 or immediate_value > 255:
            raise Exception(f'Wrong Immediate Value [{immediate_value}]. Must be in the range [0;255].')
        encoding[0:8] = int_to_bit_array(immediate_value, 8)
    else:
        check_register_validity(instruction[current_index], 'second operand')
        encoding[12:16] = int_to_bit_array(int(instruction[current_index][1:]))

    return encoding


def encode_branching(instruction):
    encoding = ['0'] * 32  # Initialise empty encoding
    encoding[28:32] = int_to_bit_array(BRANCHING_CODES[instruction[0]])  # Encode branching code
    encoding[27] = '1' if int(instruction[1]) < 0 else '0'  # Sign of the offset
    encoding[0:27] = int_to_bit_array(abs(int(instruction[1])), 27)  # Encode offset
    return encoding
