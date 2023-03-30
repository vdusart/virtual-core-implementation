; Slow & dumb version
; B 3
; ADD r3, r3, r0
; SUB r1, r1, 1
; CMP r1, 0
; BNE -3
; -------------------------------
; ----------- SMART WAY ---------
; -------------------------------
MOV r3, r0
MOV r14, 0
B 11
MOV r15, 0
RSH r1, r1, 1
ADC r15, r15, 0
LSH r1, r1, 1
CMP r15, 0
BNE 4
; PARTIE pair
LSH r3, r3, 1
RSH r1, r1, 1
B 2
; jump pour sauter impair
; PARTIE impair
ADD r14, r14, r3
CMP r1, 1
BG -11
ADD r3, r3, r14