use ::std::{ convert::AsRef, ops::Deref, path::{ Path, PathBuf } };
//
// 因为该函数的唯一【形参】兼容于任何“可解引用为 &Path 的自定义引用【实参】”。
//
fn print_fst<T: AsRef<Path>>(file_path: T) {
    let file_path: &Path = file_path.as_ref(); // 手动解引用，而不是自动解引用
    println!("[静态分派]文件路径fst= {}", file_path.display());
}
fn print_dst1(file_path: &dyn AsRef<Path>) {
    let file_path: &Path = file_path.as_ref(); // 手动解引用，而不是自动解引用
    println!("[动态分派][普通引用]文件路径fst= {}", file_path.display());
}
fn print_dst2(file_path: Box<dyn AsRef<Path>>) {
    let file_path: &Path = file_path.deref().as_ref(); // 手动解引用，而不是自动解引用
    println!("[动态分派][智能指针]文件路径fst= {}", file_path.display());
}
fn main() {
    let string_file_path = "/etc/<string>".to_string();
    let path_buf = PathBuf::from("/etc/<PathBuf>");
    //
    // 因为【标准库】预置了对“自定义引用”的引用的泛型覆
    // 盖实现，所以
    //
    // 1. 静态分派手到擒来
    // 1. AsRef<F> 实现类也就具备了部分“自动解引用”能力。
    print_fst(&string_file_path); // &String  ➜ &Path
    print_fst(&path_buf);         // &PathBuf ➜ &Path
    // 2. 甚至，引用的引用也都能处理。
    print_fst(&&string_file_path); // &&String  ➜ &Path
    print_fst(&&path_buf);         // &&PathBuf ➜ &Path
    // 3. 而且，普通引用的动态分派
    print_dst1(&string_file_path);
    print_dst1(&path_buf);
    // 最后，再消费掉变量的所有权
    print_fst(string_file_path); // String  ➜ &Path
    print_fst(path_buf);         // PathBuf ➜ &Path

    let string_file_path = Box::new("/etc/<string>".to_string());
    let path_buf = Box::new(PathBuf::from("/etc/<PathBuf>"));
    print_dst2(string_file_path);
    print_dst2(path_buf);
}