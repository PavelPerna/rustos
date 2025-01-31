#![no_std]
#![no_main]
use core::fmt::Write;
use kore::std::io::*;

pub mod kore;
pub mod test;

// LIBC handling 
extern crate libc;
#[link(name="c")]
unsafe extern "C"  {
}

#[unsafe(no_mangle)]
pub  fn main()-> !{
    let mut stdout = InputOutput::from(SYS_WRITE); // Standard output
    let mut stdin = InputOutput::from(SYS_READ);
    let buf:&mut [u8] = &mut [0;1024];                                      

    
    loop{
        let len :i32 = stdin.readln(buf).expect("Error reading from STDIN !!!");
        if len > 0 {
            let data = core::str::from_utf8(&buf[0..len as usize]).expect("Pica");
            let _ = write!(&mut stdout,"Nactena data: Delka({}) , Obsah({})",len,data);
        }else{  
            let _ = write!(&mut stdout,"Chyba  pico !!!" );
        }    
    }
}

pub fn test(){
    let mut stdout = InputOutput::from(SYS_WRITE); // Standard output
    let mut stdin = InputOutput::from(SYS_READ);
    let buf:&mut [u8] = &mut [0;1024];                                      

    let len :i32 = stdin.read(buf, 1024).expect("Error reading from STDIN !!!");
    if len > 0 {
        let data = core::str::from_utf8(&buf[0..len as usize]).expect("Pica");
        let _ = write!(&mut stdout,"Nactena data: Delka({}) , Obsah({})",len,data);
    }else{  
        let _ = write!(&mut stdout,"Chyba  pico !!!" );
    }
}
