# Chapter 2. System calls

This OS offers important low level abstractions through the use of system calls. This will be made using the SVC instruction.
Here, a non extensive list of system calls are shown:

File management (prefix 1)
1. open
2. close
3. read
4. write
5. stat
6. seek
7. mkdir
8. rmdir
9. readdir
10. ioctl
11. remove

Connection management (prefix 2)
1. socket
2. bind
3. listen
4. accept
5. connect
6. send
7. recv

Thread management (prefix 3)
1. yield
2. pause
3. getid
4. spawn
5. delete

Memory management (prefix 4)
1. malloc
2. realloc
3. free
4. mq_open
5. mq_write
6. mq_read

The syscalls will follow the AAPCS specification, so the arguments of these syscalls will be put in registers R0-R3.