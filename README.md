# Virtual core implementation - Report

Group members:
- Lucas DRAESCHER
- Victor DUSART

## Explanation of the implementation

TODO

## How to use

### Requirements
To use the project, Python (>= 3.10) and Rust (with cargo) are required.

### Using the compiler
The compiler can be found under `compiler/compiler.py`. To use it, simply: `python3 compiler.py [INPUT FILE]`.

### Using the virtual core
- Compiling the core: `cargo build --release`
- Running the core: `cargo run --release [INPUT CODE] [INITIAL STATE FILE] (optional: verbose)`

## Explanation of the tested programs

TODO

## Answers to the questions

> Which parts of a 64-bit processor are 64 bits wide?

In a 64-bit processor, the registers are 64 bits wide.

> Which instructions can potentially create a carry?

Addition, subtraction and bit-shifting operations create carries.

> What is the purpose of the add carry (ADC) instruction?

The add carry instruction allows the addition of numbers that are greater than 64 bits in length.

> What are the checks to realise during a branch instruction?

Here are some checks the CPU needs to realise during a branch instruction:
- Check if the branch should be taken or not if it is a conditional branch.
- Calculating the target address and checking if it is a valid address in the program's memory space.

> Is it possible to pipeline the virtual core?

It would be possible to pipeline the virtual core, but it would require parallel programming in order to fetch, 
decode and execute instructions simultaneously. 

Pipelining would also introduce a lot of complexity in the handling of instructions, especially for branches that would 
require optimisations such as branch prediction algorithms or pipeline flushing.