section .bss
probable_primes : resb 1000000

section .data
number : db "%ld", 0xA, 0
len : dq 1000000
section .text
extern printf
global main
main:
        mov rsi, 0
        fill_array:
                mov byte[probable_primes + rsi], 1
                inc rsi
                cmp rsi, qword[len]
                jl fill_array
        
        mov rsi, 2
        mov rcx, 2
        condition:
                cmp rcx, qword[len]
                jl for
                jmp continue
        for:
                condition2:
                        mov rax, rsi
                        imul rax, rcx
                        cmp rax, qword[len]
                        jl for2
                        jmp continue2
                for2:
                        mov byte[probable_primes + rax], 0
                        inc rsi
                        jmp condition2
                continue2:
                        mov rsi, 2
                        inc rcx
                        jmp condition

        continue:
        mov r12, 2
        sum:
                cmp byte[probable_primes + r12], 1
                je is_prime
                jmp conti
                is_prime:
                        mov rdi, number
                        mov rsi, r12
                        mov al, 0
                        call printf
                conti:
                        inc r12
                        cmp r12, qword[len]
                        jl sum
        mov rbx, 0
        mov rax, 0
        int 0x80
