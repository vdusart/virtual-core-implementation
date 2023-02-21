import sys

from keywords import OPERATION_CODES, BRANCHING_CODES
from encoders import encode_operation, encode_branching

if __name__ == '__main__':

    if len(sys.argv) != 2:
        print('[+] Usage: python3 compiler.py [INPUT FILE]')
        sys.exit(1)

    filename = sys.argv[1]
    try:
        with open(filename, 'r') as f:
            lines = f.readlines()
    except FileNotFoundError:
        print(f'[-] Could not find: {filename}')
        sys.exit(1)

    if not filename.endswith('.s'):
        print('[-] Input must be an assembly file.')
        sys.exit(1)

    # Read the assembly file line by line and encode each instruction
    instructions = []
    for line in lines:
        instruction = line.strip().replace(',', '').split(' ')
        match instruction[0]:
            case x if str(x).startswith(';'):  # Allow comments
                continue
            case x if x in OPERATION_CODES:
                encoded_instruction = encode_operation(instruction)
            case x if x in BRANCHING_CODES:
                encoded_instruction = encode_branching(instruction)
            case _:
                raise Exception(f'Unknown operation: {line}')
        instructions += encoded_instruction[::-1]  # Reverse order to match big endianness

    # Write binary file
    binary = [int(''.join(instructions[i:i + 8]), 2) for i in range(0, len(instructions), 8)]
    binary_filename = filename.replace('.s', '.bin')
    with open(binary_filename, 'wb') as f:
        f.write(bytearray(binary))
