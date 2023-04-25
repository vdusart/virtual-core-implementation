; A > B
MOV r0, 49
MOV r1, 21
; int compare(int a, int b)
MOV r2, 254
CMP r0, r1
BG 2
MOV r2, 175