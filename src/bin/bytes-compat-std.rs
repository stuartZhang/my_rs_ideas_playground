//! bytes crate 可与标准库中的 std::io::Cursor 无缝衔接使用
use ::bytes::BytesMut;
use ::std::io::{ BufReader, Cursor, Read };
fn main() {
    // 从二进制字符串字面量构造 bytes 实例
    let mut bytes = BytesMut::from(&b"Hello, World!"[..]);
    // 从 bytes 实例构造标准库的 Cursor 实例
    let cursor = Cursor::new(&mut bytes);
    // 使用标准库的Read trait
    let mut reader = BufReader::new(cursor);
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();

    println!("Read string: {}", string);
}