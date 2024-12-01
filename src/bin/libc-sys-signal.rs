use ::libc::{ c_int, sighandler_t, signal, SIGINT };
use ::std::{ sync::atomic::{ AtomicBool, Ordering }, thread, time::Duration };

static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() {
    println!("程序运行中，按Ctrl+C退出...");
    let join = thread::spawn(|| {
        extern "C" fn handle_sigint(signal: c_int) {
            dbg!(signal);
            RUNNING.store(false, Ordering::SeqCst);
        }
        unsafe { signal(SIGINT, handle_sigint as sighandler_t) };
        while RUNNING.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(1));
        }
    });
    join.join().unwrap();
    println!("程序正常退出");
}