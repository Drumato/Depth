	.file	"c.c"
	.intel_syntax noprefix
	.text
	.globl	main
	.type	main, @function
main:
	push	rbp
	mov	rbp, rsp
	add al, 0x04
	add ax, 0x04
	add eax, 0x04
	add rsi, 0x04
	add esi, 0x04
	add si, 0x04
	add cl, 0x04
	add rsi,rdi
	add esi,edi
	add si,di
	add cl,dl
	pop	rbp
	ret
	.size	main, .-main
	.ident	"GCC: (Ubuntu 7.4.0-1ubuntu1~18.04) 7.4.0"
	.section	.note.GNU-stack,"",@progbits
