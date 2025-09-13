	global main

	section .data

message: db "Hello, World", 10
message_len: equ $-message
message_2: db "Hello,   :", 10
message_2_len: equ $-message_2
	;; awd
	section .text

main:
	;d wa

	mov rax, 1
	mov rdi, 1
	mov rsi, message
	mov rdx, message_len
	syscall

	mov rax, 60
	mov rdi, 0
	syscall

other:
	mov rax, 50
