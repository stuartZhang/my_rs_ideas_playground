fn main() {
    use ::std::mem;
    enum Example1 { _A }
    println!("Example1: alignment = {1}; size = {0}", mem::size_of::<Example1>(), mem::align_of::<Example1>());
    #[repr(C)]
    enum Example2 { _A }
    println!("Example2: alignment = {1}; size = {0}", mem::size_of::<Example2>(), mem::align_of::<Example2>());
}