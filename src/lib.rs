use std::convert::TryInto;

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

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn offset(&self) -> usize { 
        self.current_position 
    }

    pub fn get_u8(&mut self) -> u8 {
        let result  = self.buffer[self.current_position];
        self.current_position += 1;
        result
    }

    pub fn get_u16(&mut self) -> u16 {
        let slice : [u8; 2] = self.buffer[self.current_position..self.current_position+2]
            .try_into()
            .unwrap();
        self.current_position += 2;
        u16::from_be_bytes(slice)
    }

    pub fn get_u32(&mut self) -> u32 {
        let slice : [u8; 4] = self
            .buffer[self.current_position..self.current_position + 4]
            .try_into()
            .unwrap();
        self.current_position += 4;
        u32::from_be_bytes(slice)
    }

    pub fn get_u64(&mut self) -> u64 {
        let slice : [u8; 8] = self
            .buffer[self.current_position..self.current_position + 8]
            .try_into()
            .unwrap();
        self.current_position += 8;
        u64::from_be_bytes(slice)
    }

    pub fn slice(&mut self, start : usize, end : usize) -> &[u8] {
        self.buffer[start..end].as_ref()
    }

    pub fn reset_offset(&mut self) {
        self.current_position = 0;
    }

    pub fn move_offset_back(&mut self, count : usize) {
        if self.current_position - count < 0 {
            panic!("Offset less than 0");
        }
        self.current_position -= count;
    }

    pub fn move_offset_forward(&mut self, count : usize) {
        if self.current_position + count > self.buffer.len() {
            panic!("Offset less than buffer size");
        }
        self.current_position += count;
    }

    pub fn decode_next_bytes(&mut self, count : usize) -> String {
        let slice = self.buffer[self.current_position..self.current_position + count]
            .to_vec();
        String::from_utf8(slice).unwrap()
    }
}
