#[cfg(test)]
mod tests {
    use crate::{InputOutput,SYS_READ,SYS_WRITE};
    // LIBC handling 
    extern crate libc;
    #[link(name="c")]
    unsafe extern "C"  {
    }

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // LIBC handling 
    #[test]
    fn test_input() {
        let  stdin = InputOutput::from(SYS_READ);
        let  stdout = InputOutput::from(SYS_WRITE);
        assert_eq!(stdin.fd, 0);
        assert_eq!(stdout.fd, 1);
    }
}