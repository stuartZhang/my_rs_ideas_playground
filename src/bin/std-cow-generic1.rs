use ::std::borrow::Cow;
// Into<Cow<'a, _>> 的泛型用得好，值得学习与模仿。
fn to_uppercase<'a, S: Into<Cow<'a, str>>>(s: S) -> Cow<'a, str> {
    match s.into() {
        Cow::Borrowed(borrowed) => { // 原本存的是引用，后续就得分情况处理。
            if borrowed.chars().any(|c| c.is_lowercase()) { // 修改，保存所有权副本
                Cow::Owned(borrowed.to_uppercase())
            } else { // 不修改，保存引用
                Cow::Borrowed(borrowed)
            }
        }
        Cow::Owned(mut owned) => { // 原本存的是所有权值，就直接修改
            owned.make_ascii_uppercase();
            Cow::Owned(owned)
        }
    }
}
fn main() {
    println!("{}", to_uppercase("hello"));
    println!("{}", to_uppercase("HELLO"));
    println!("{}", to_uppercase(String::from("World")));
}