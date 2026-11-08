; KiK Boot - Legacy BIOS Stage 1
; This is a simple 512-byte boot sector that prints to serial and hangs

BITS 16
ORG 0x7C00

start:
    ; Disable interrupts
    cli
    
    ; Set up segments
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    
    ; Enable interrupts
    sti
    
    ; Initialize serial port (COM1)
    mov dx, 0x3F8 + 1    ; IER
    xor al, al
    out dx, al
    
    mov dx, 0x3F8 + 3    ; LCR
    mov al, 0x80
    out dx, al
    
    mov dx, 0x3F8        ; DLL
    mov al, 0x03
    out dx, al
    
    mov dx, 0x3F8 + 1    ; DLM
    xor al, al
    out dx, al
    
    mov dx, 0x3F8 + 3    ; LCR
    mov al, 0x03
    out dx, al
    
    mov dx, 0x3F8 + 2    ; FCR
    mov al, 0xC7
    out dx, al
    
    mov dx, 0x3F8 + 4    ; MCR
    mov al, 0x0B
    out dx, al
    
    ; Print message
    mov si, msg
    call print_string
    
    ; Hang
    cli
    hlt
    jmp $

print_string:
    lodsb
    test al, al
    jz .done
    ; Wait for transmit buffer empty
    mov dx, 0x3F8 + 5
.wait:
    in al, dx
    test al, 0x20
    jz .wait
    ; Write character
    mov al, [si-1]
    mov dx, 0x3F8
    out dx, al
    jmp print_string
.done:
    ret

msg db "KiK Boot - BIOS", 13, 10, "Loading KiK OS kernel...", 13, 10, 0

; Pad to 510 bytes and add boot signature
times 510-($-$$) db 0
dw 0xAA55