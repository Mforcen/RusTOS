pub const FILE_OPEN:	u32 = 0x10;
pub const FILE_CLOSE:	u32 = 0x11;
pub const FILE_READ:	u32 = 0x12;
pub const FILE_WRITE:	u32 = 0x13;
pub const FILE_STAT:	u32 = 0x14;
pub const FILE_MKDIR:	u32 = 0x15;
pub const FILE_RMDIR:	u32 = 0x16;
pub const FILE_READDIR:	u32 = 0x17;
pub const DEVICE_MGMT:	u32 = 0x18;

pub const CONN_SOCKET:	u32 = 0x20;
pub const CONN_BIND:	u32 = 0x21;
pub const CONN_LISTEN:	u32 = 0x22;
pub const CONN_ACCEPT:	u32 = 0x23;
pub const CONN_CONNECT:	u32 = 0x24;
pub const CONN_SEND:	u32 = 0x25;
pub const CONN_RECV:	u32 = 0x26;

pub const THREAD_YIELD:	u32 = 0x30;
pub const THREAD_PAUSE:	u32 = 0x31;
pub const THREAD_GETID:	u32 = 0x32;
pub const THREAD_SPAWN:	u32 = 0x33;
pub const THREAD_DEL:	u32 = 0x34;

pub const MEM_ALLOC:	u32 = 0x40;
pub const MEM_REALLOC:	u32 = 0x41;
pub const MEM_FREE:		u32 = 0x42;
pub const MEM_MQOPEN:	u32 = 0x43;
pub const MEM_MQWRITE:	u32 = 0x44;
pub const MEM_MQREAD:	u32 = 0x45;