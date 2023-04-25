MOV r3, r1
MOV r4, r0
B 5
LSH r3, r3, 1
LSH r4, r4, 1
ADC r3, r3, 0
SUB r2, r2, 1
CMP r2, 0
BNE -5
