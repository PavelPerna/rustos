#[cfg(target_arch = "aarch32")]
use aarch32;
#[cfg(target_arch = "aarch64")]
use aarch64;
#[cfg(target_arch = "android")]
use android;
#[cfg(target_arch = "apple")]
use apple;
#[cfg(target_arch = "arm")]
use arm;
#[cfg(target_arch = "x86")]
use x86;
#[cfg(target_arch = "x86_64")]
use x86_64;


pub struct SyscallArguments {
    pub arg0: usize,
    pub arg1: usize,
    pub arg2: usize,
    pub arg3: usize,
    pub arg4: usize,
    pub arg5: usize,
}

macro_rules! syscall {
    ($nr:expr) => { Asm::syscall0($nr:expr) };
    ($nr:expr, $a1:expr) => { Asm::syscall1($nr:expr, $a1:expr) };
    ($nr:expr, $a1:expr, $a2:expr) => { Asm::syscall2($a1:expr, $a2:expr) };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => { Asm::syscall3($nr:expr, $a1:expr, $a2:expr) };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => { $nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => { $nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => { Asm::syscall6($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) };
}

pub struct Asm{
}

pub trait SysCall{
    fn syscall(sysno: SysID, arg : &SyscallArguments ){

    }
    fn syscall0(sysno){
        
    }
}
