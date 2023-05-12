# Virtual core implementation - Report
Group members:
- Lucas DRAESCHER
- Victor DUSART

## Explanation of the implementation
We chose a combination of Python and Rust for this project. The compiler is written in Python because it is essentially just array manipulation and Python's powerful list manipulation allowed us to get the compiler done in just a few hours. The core had to be written in a compiled language, per the subjects requirements. We both wanted to learn Rust, so we decided to write the core with it.

The compiler is divided into 3 files:
- `compiler.py` is the main entrypoint to the program. It does error handling and printing.
- `encoders.py` defines how operations and branching codes are encoded.
- `keywords.py` defines the programs keywords as objects.

The core is divided into 5 files:
- `executor.rs` implements all the operators.
- `keywords.rs` defines the operation codes, branching codes and flags along with utility functions for each type.
- `loading.rs` defines functions for reading the input files.
- `main.rs` is the main entrypoint to the program.
- `pipeline.rs` contains the `fetch`, `decode` and `execute` functions.

For implementation details, the important functions have comments explaining how they work.

## How to use

### Requirements
To use the project, Python (>= 3.10) and Rust (>= 1.67.0) (with cargo) are required.

### Using the compiler
The compiler can be found under `compiler/compiler.py`. To use it, simply: `python3 compiler.py [INPUT FILE]`.

### Using the virtual core
- Compiling the core: `cargo build --release`
- Running the core: `cargo run --release [INPUT CODE] [INITIAL STATE FILE] (optional: --verbose)`

## Explanation of the tested programs
1) init
   - we start by inserting the value we want in decimal base (1 = 0x01)
   - we then left shift by two hexadecimal digits (8 bits)
   - we add the next value in decimal base (35 = 0x23)
   - rinse and repeat...
   - a similar algorithm is used for r1 and r2, only by adding different values depending on the output we wish to have
2) 128 bits addition
   - we add the lower 64 bits together into r6
   - we then add the higher 64 bits together into r5 by taking into account an eventual carry from the previous addition
3) 64 to 128 bits left shift
   - r2 contains the higher 64 bits of the shift (initialise it to 0)
   - r3 contains the lower 64 bits of the shift (initialise it to r0)
   - branch to the final comparison (exit if r1 = 0, loop otherwise)
   - left shift r2 by 1
   - left shift r3 by 1
   - if the previous operation overflowed, catch the overflow and add it to r2 (this step is what creates the "transfer" to the second register)
   - decrement r1
   - we loop over until r1 is equal to zero (r1 = number of bits to shift by)
   1) bonus: 128 bits left shift. It is the exact same program with some minor differences:
      - r0 and r1 contain the initial value (high bits = r0, low bits = r1)
      - r2 contains the number of bits to shift by
      - r3 contains the higher 64 bits (initialise it to r0)
      - r4 contains the lower 64 bits (initialise it to r1)
4) 64 bits multiplication
   1) Slow way
      - a * b is the same as (a + ... + a) b times
      - the slow way is doing b addition of the a number
      - r1 contains the b value
      - r3 contains the bottom 64 bits part of the result
      - r2 contains the top 64 bits part of the result
      - the result of the product is stored in [r2][r3]
   2) Fast way
      - *doing a 1 bit left shift on a number is the same as multiplying it by 2*
      - *a 1 bit right shift on a number is division by 2*
      - instead of doing successive additions, we can make multiplication by 2 of the result, which is in fact a 1 bit left shift and a division by 2 of the multiplier (1 bit right shift)
      - eg: x * 8 = (x + x + x + x) * 4 = 2x * 4
      - with this technique, we can save a lot of operation
      - it works well as long as you want to multiply by a number divisible by 2.
      When the number is no longer divisible by 2, you have to use another register to do a multiplication and an addition
      - to better understand the processus, let's see a simple example:
         - 5 * 14
         - 5 * 2 * 7
         - 10 * 7
         - 10 * 6 + 10
         - 10 * 2 * 3 + 10
         - 20 * 3 + 10
         - 20 * 2 + 20 + 10
         - 40 + 30
         - 70

5) 64 to 128 bits factorial
   - for the factorial, we use exactly the same concept but we decompose it in sucessive multiplication to build our result
   - example:
      - 5!
      - 5 * 4!
      - 5 * 4 * 3!
      - 5 * 2 * 2 * 3!
      - 10 * 2 * 3!
      - 20 * 3!
      - 20 * 3 * 2!
      - (20 * 2 + 20) * 2!
      - (40 + 20) * 2!
      - 60 * 2!
      - 60 * 2
      - 120
6) from C code
   - comments related to this function are embedded within the assembly code

## Answers to the questions
> Which parts of a 64-bit processor are 64 bits wide?

In a 64-bit processor, the registers are 64 bits wide.

> Which instructions can potentially create a carry?

Addition, subtraction and bit-shifting operations create carries.

> What is the purpose of the add carry (ADC) instruction?

The add carry instruction adds its two operands and also adds 1 if the carry flag is set. This allows operations on numbers that are greater than 64 bits in length.

> What are the checks to realise during a branch instruction?

Here are some checks the CPU needs to realise during a branch instruction:
- Check if the branch should be taken or not if it is a conditional branch.
- Calculating the target address and checking if it is a valid address in the program's memory space.

> Is it possible to pipeline the virtual core?

It would be possible to pipeline the virtual core, but it would require parallel programming in order to fetch, 
decode and execute instructions simultaneously. 

Pipelining would also introduce a lot of complexity in the handling of instructions, especially for branches that would 
require optimisations such as branch prediction algorithms or pipeline flushing.