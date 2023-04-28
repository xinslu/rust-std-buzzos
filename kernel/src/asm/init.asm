global init_start
section .text.init
bits 32
init_start:
    ; System Call Number
    mov eax, 0x0

    ; 4 Parameters
    mov ecx, 0x10
    mov edx, 0x1
    mov esi, 0x0
    mov edi, 0x0

    ; Uncomment for storing string during write syscall
    ; ; Stores string 'Z' at addr in ecx reg
    ; mov esi, 0x5A
    ; mov [ecx], esi
    
    ; User System Calls Trap Number
    int 64

end:
    jmp end