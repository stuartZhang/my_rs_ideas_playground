use libc::{ errno_t, time_t, tm };
use std::{ error::Error, io::Error as IoError, mem::MaybeUninit, ptr };

fn main() -> Result<(), Box<dyn Error>> {
    let mut tm: MaybeUninit<tm> = MaybeUninit::uninit();
    let tm = unsafe {
        let t: time_t = libc::time(ptr::null_mut());
        let err: errno_t = libc::localtime_s(tm.as_mut_ptr(), &t);
        dbg!(err);
        if err != 0 {
            return Err(Box::new(IoError::last_os_error()));
        }
        tm.assume_init()
    };
    println!("当前时间: {}年{}月{}日 {}时{}分{}秒", tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday, tm.tm_hour, tm.tm_min, tm.tm_sec);
    Ok(())
}