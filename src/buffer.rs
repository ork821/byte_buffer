pub struct Buffer {
    current_position : usize,
    buffer : Vec<u8>
}

impl Buffer {
    pub fn from(vec : Vec<u8>) -> Buffer {
        Buffer {
            buffer : vec,
            current_position : 0
        }
    }

    pub fn get_u32(&self) -> u32 {
        let slice = &self.buffer[0..4];
        u32::from_be_bytes(slice)
    }
}
