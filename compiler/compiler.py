import sys

from encoders import encode_operation, encode_branching
from keywords import OPERATION_CODES, BRANCHING_CODES


class Colours:
    SUCCESS = '\033[92m'
    INFO = '\033[93m'
    ERROR = '\033[91m'
    RESET = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'


class LoggingMessages:
    SUCCESS = f'{Colours.BOLD}{Colours.SUCCESS}[SUCCESS]{Colours.RESET}'
    INFO = f'{Colours.BOLD}{Colours.INFO}[INFO]{Colours.RESET}'
    ERROR = f'{Colours.BOLD}{Colours.ERROR}[ERROR]{Colours.RESET}'


if __name__ == '__main__':

    if len(sys.argv) != 2:
        print(f'{LoggingMessages.INFO} Usage: python3 compiler.py [INPUT FILE]')
        sys.exit(1)

    filename = sys.argv[1]
    try:
        with open(filename, 'r') as f:
            lines = f.readlines()
    except FileNotFoundError:
        print(f'{LoggingMessages.ERROR} Could not find {Colours.UNDERLINE}{filename}{Colours.RESET}.')
        sys.exit(1)

    if not filename.endswith('.s'):
        print(f'{LoggingMessages.ERROR} Input must be an assembly file.')
        sys.exit(1)

    # Read the assembly file line by line and encode each instruction
    instructions = []
    for index, line in enumerate(lines, start=1):
        line = line.strip()
        if line == '' or line.startswith(';'):  # Ignore comments and empty lines
            continue
        instruction = line.replace(',', '').split(' ')
        instruction[0] = instruction[0].upper()
        try:
            match instruction[0]:
                case x if x in OPERATION_CODES:
                    encoded_instruction = encode_operation(instruction)
                case x if x in BRANCHING_CODES:
                    encoded_instruction = encode_branching(instruction)
                case _:
                    raise Exception(f'Unknown operation [{instruction[0]}].')
        except Exception as e:
            print(f'{LoggingMessages.ERROR} On line {index}: {e.args[0]}')
            sys.exit(-1)
        instructions += encoded_instruction[::-1]  # Reverse order to match big endianness

    # Write binary file
    binary = bytearray([int(''.join(instructions[i:i + 8]), 2) for i in range(0, len(instructions), 8)])
    binary_filename = filename.replace('.s', '.bin')
    binary_size = len(binary)
    with open(binary_filename, 'wb') as f:
        f.write(binary)
    print(f"{LoggingMessages.SUCCESS} {Colours.UNDERLINE}{filename}{Colours.RESET} "
          f"compiled to {Colours.UNDERLINE}{binary_filename}{Colours.RESET} ({binary_size} bytes).")
