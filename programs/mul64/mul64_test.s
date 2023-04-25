; Slow & dumb version
; B 3
; ADD r3, r3, r0
; SUB r1, r1, 1
; CMP r1, 0
; BNE -3
; -------------------------------
; ----------- SMART WAY ---------
; -------------------------------
MOV r2, 0
MOV r3, r0
MOV r13, 0
MOV r14, 0
B 14
MOV r15, 0
RSH r1, r1, 1
ADC r15, r15, 0
LSH r1, r1, 1
CMP r15, 0
BNE 6
; Even part
LSH r2, r2, 1
LSH r3, r3, 1
ADC r2, r2, 0
RSH r1, r1, 1
B 3
; jump to skip odd part
; Odd part
ADD r14, r14, r3
ADC r13, r13, r2
CMP r1, 1
BG -14
ADD r3, r3, r14
ADC r2, r2, r13
