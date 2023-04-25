MOV r2, 0
MOV r3, r0
B 5
LSH r2, r2, 1
LSH r3, r3, 1
ADC r2, r2, 0
SUB r1, r1, 1
CMP r1, 0
BNE -5
