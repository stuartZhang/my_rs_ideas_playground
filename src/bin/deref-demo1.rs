use ::std::{ convert::AsRef, ops::Deref };

struct Name(String);

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn print_name<T: AsRef<str>>(name: T) {
    println!("Name: {}", name.as_ref());
}

fn main() {
    let name = Name(String::from("Alice"));
    /*
    因为 impl<T, U> AsRef<U> for &T where T: AsRef<U> + ?Sized, U: ?Sized 的覆盖实现(https://doc.rust-lang.org/std/convert/trait.AsRef.html#impl-AsRef%3CU%3E-for-%26T)，所以 &T 与 T 的 as_ref(&self) 都是返回 &U。这被称为 AsRef 的自动解引用。
     */
    print_name(&name); // 【引用转换】作用于这一条语句
    println!("Length: {}", name.len()); // 【自动解引用】作用于这一条语句。
}