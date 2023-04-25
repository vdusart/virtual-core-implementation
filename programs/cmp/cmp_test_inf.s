; This function returns 0xaf because A < B.
; Conventions used for this function:
;     - First function parameter: r0.
;     - Second function parameter: r1.
;     - Function return value: r2.
; Explanation
; This program puts the '0xfe' value in r2 (the return register) and only changes its value (to '0xaf') when A is greater than B.
MOV r0, 49
MOV r1, 215
; int compare(int a, int b)
MOV r2, 254
CMP r0, r1
BG 2
MOV r2, 175
