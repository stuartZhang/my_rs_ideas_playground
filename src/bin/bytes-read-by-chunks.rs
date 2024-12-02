//! bytes crate 按块读取字节缓存
use ::bytes::{ Buf, Bytes };
use ::std::cmp;
fn main() {
    let mut bytes = Bytes::from(&[1, 2, 3, 4, 5, 6, 7, 8][..]);
    let mut index = 1_u8;
    // 只进游标还未到尾端
    while bytes.has_remaining() {
        // 可安全地读取的块大小 [0, 3]
        let chunk_size = cmp::min(3, bytes.remaining());
        // 读取块内容
        let chunk = &bytes.chunk()[..chunk_size];
        // 打印块内容
        println!("第{index}块: {chunk:?}");
        // 跳转【只进游标】至下一个块
        bytes.advance(chunk_size);
        index += 1;
    }
}