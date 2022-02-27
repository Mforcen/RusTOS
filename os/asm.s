.thumb
.syntax unified

.global switch_to_psp
.type switch_to_psp,%function

.global load_context
.type load_context,%function

.global save_context
.type save_context,%function

.global init_os
.type init_os,%function

.global raise_svc
.type raise_svc,%function

.global SVCall
.type SVCall,%function

SVCall:
@	mrs r0, psp @; r0 is saved automatically when entering the handler
@	ldmia r0!, {r4, r11} @; saving the rest of the cpu registers into the stack pointer
@	ldr r1, =GLOBAL_SP
@	ldr r2, =NOW_TASK
@	lsl r2, r2, #2 @; left shift by 2 in order to get address of specific task
@	add r1, r1, r2 @; now address is in r0
@	str r0, [r1] @; r0 contains psp, r1 contains pointer to global_sp item
	bx lr
	.size SVCall, . - SVCall @; . is the this address operator, like the linker script


switch_to_psp:
@	ldr r0,=0x00000003
@	msr control,r0
@	isb
	bx lr
	.size switch_to_psp, . - switch_to_psp
	

load_context:
@	ldmia	r0!, {r4,r11} @; this will read all the registers from r0, which is the stack pointer of the task
	bx	lr @; this will return to the caller
	.size load_context, . - load_context


save_context: @; this operation will take all the registers that the CPU did not store and save it to recover it later
@	mrs	r0, psp @; read process stack pointer address and place it in r0
@	stmdb	r0!, {r4-r11} @; write several registers at once (store multiple decrement before), due to !, r0 new address will be updated
	bx	lr @; this will return to the caller
	.size save_context, . - save_context


init_os: @;this function will perform the first context change
@	ldr r0, =GLOBAL_SP
@	ldr r0, [r0]
@	msr psp, r0
@	ldr r1,=0x00000003
@	msr control, r1
@	isb
@	add r0, r0, #56
@	ldr r0, [r0]
@	bx r0
	bx lr
	.size init_os, . - init_os



raise_svc:
	svc 1
	bx lr
	.size raise_svc, . - raise_svc
