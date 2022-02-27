# Chapter 2. System calls

This OS offers important low level abstractions through the use of system calls. This will be made using the SVC instruction.
Here, a non extensive list of system calls are shown:

File management
1. open
2. close
3. read
4. write

Device management
1. ioctl

Connection management
1. socket
2. bind
3. listen
4. accept
5. connect
6. send
7. recv

Memory management
1. malloc
2. realloc
3. free

The syscalls will follow the AAPCS specification, so the arguments of these syscalls will be put in registers R0-R3.