use byte_buffer::Buffer;

fn main() {
    let vec: Vec<u8> = vec![109,118,104,100,5,6,7,8];
    let mut buffer = Buffer::from(vec);
    println!("{}", buffer.get_u64());
    println!("{:?}", buffer.slice(1,4));
    buffer.reset_offset();
    println!("{}", buffer.decode_next_bytes(4));
}
