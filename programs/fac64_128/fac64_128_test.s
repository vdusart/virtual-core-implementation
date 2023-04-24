CMP r0, 0
BNE 3
MOV r3, 1
B 26
MOV r3, r0
MOV r2, 0
SUB r0, r0, 1
MOV r1, r0
MOV r13, 0
MOV r14, 0
B 14
MOV r15, 0
RSH r1, r1, 1
ADC r15, r15, 0
LSH r1, r1, 1
CMP r15, 0
BNE 6
; PARTIE pair
LSH r2, r2, 1
LSH r3, r3, 1
ADC r2, r2, 0
RSH r1, r1, 1
B 3
; jump pour sauter impair
; PARTIE impair
ADD r14, r14, r3
ADC r13, r13, r2
CMP r1, 1
BG -14
ADD r3, r3, r14
ADC r2, r2, r13
CMP r0, 1
BG -23