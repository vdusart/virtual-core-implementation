; Set register 0 to 0x0123456789abcdef
ADD r0, r0, 1   ; 01
LSH r0, r0, 8
ADD r0, r0, 35  ; 23
LSH r0, r0, 8
ADD r0, r0, 69  ; 45
LSH r0, r0, 8
ADD r0, r0, 103 ; 67
LSH r0, r0, 8
ADD r0, r0, 137 ; 89
LSH r0, r0, 8
ADD r0, r0, 171 ; ab
LSH r0, r0, 8
ADD r0, r0, 205 ; cd
LSH r0, r0, 8
ADD r0, r0, 239 ; ef
; Set register 1 to 0xa5a5a5a5a5a5a5a5
ADD r1, r1, 165
LSH r1, r1, 8
ADD r1, r1, 165
LSH r1, r1, 8
ADD r1, r1, 165
LSH r1, r1, 8
ADD r1, r1, 165
LSH r1, r1, 8
ADD r1, r1, 165
LSH r1, r1, 8
ADD r1, r1, 165
LSH r1, r1, 8
ADD r1, r1, 165
LSH r1, r1, 8
ADD r1, r1, 165
; Set register 2 to 0xaef45d745aff584f
ADD r2, r2, 174
LSH r2, r2, 8
ADD r2, r2, 244
LSH r2, r2, 8
ADD r2, r2, 93
LSH r2, r2, 8
ADD r2, r2, 116
LSH r2, r2, 8
ADD r2, r2, 90
LSH r2, r2, 8
ADD r2, r2, 255
LSH r2, r2, 8
ADD r2, r2, 88
LSH r2, r2, 8
ADD r2, r2, 79