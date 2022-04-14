.thumb
.syntax unified

.global switch_to_psp
.type switch_to_psp,%function

.global load_context
.type load_context,%function

.global save_context
.type save_context,%function

.global raise_svc
.type raise_svc,%function


switch_to_psp:
	@ldr r0,=0x00000003
	@msr control,r0
	@isb
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


raise_svc:
	@svc 1
	bx lr
	.size raise_svc, . - raise_svc
