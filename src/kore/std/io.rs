use core;
use core::arch::asm;
use core::str;
use core::ffi::c_int;
use core::fmt::Write;
use core::result::Result;

pub const SYS_READ : c_int = 0;
pub const SYS_WRITE : c_int = 1;
#[warn(unused_assignments)]
pub trait Read {
    // Required method
    fn read(&mut self, buf: &mut [u8], count:i32) -> Result<i32, i32>;
    fn readln(&mut self, buf: &mut [u8]) -> Result<i32, i32>;
}

pub struct InputOutput{
    pub fd: c_int, // IO handle 0 - Input 1 - Output
}


impl From<c_int> for InputOutput{
    fn from(fd :c_int) -> Self{
        Self{fd:fd}
    }
}


impl Read for InputOutput {
    fn read(&mut self, buf: &mut [u8], count: i32) -> Result<i32, i32> {
        let res = unsafe {
            let mut _len:isize = 0;
            asm!(
                "mov rax, {:r}",
                "mov rdi, {:r}",
                "mov rsi, {:r}",
                "mov rdx, {:r}",
                "syscall",
                    in(reg) self.fd, 
                    in(reg) 0, 
                    in(reg) buf.as_mut_ptr(),
                    in(reg) count,
                    lateout("eax") _len
            );
            _len
        };
        if res < 0 {
            return Err(0);
        }
        Ok(res as i32)
    }
    fn readln(&mut self, buf: &mut [u8]) -> Result<i32, i32> {
        let read = self.read(buf, 1024);
        match read {
            Ok(n)=> Ok(n - 1),
            Err(_)=>Err(-1)
        }
    }
}


impl Write for InputOutput {
    fn write_str(&mut self, buf: &str)-> Result<(), core::fmt::Error> {
        unsafe {
            asm!(
                "mov rax, {:r}",
                "mov rdi, {:r}",
                "mov rsi, {:r}",
                "mov rdx, {:r}",
                "syscall",
                    in(reg) self.fd, 
                    in(reg) 0, 
                    in(reg) buf.as_ptr() as i64,
                    in(reg) buf.len() as i64,
                    options(readonly)
            );
        }
        Ok(())
    }
    fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) -> Result<(), core::fmt::Error> {
        core::fmt::write(self, args)?;
        Ok(())
    }
}
