MOV r3, r0
B 4
LSH r2, r2, 1
LSH r3, r3, 1
ADC r2, r2, 0
CMP r1, 0
BNE -4