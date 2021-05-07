pub enum OpCode {
    Return,
}

pub struct Chunk {
    code: usize,
    count: i32,
    capacity: i32,
}

impl Chunk {
    fn new() -> Self {
        Self {
            code: 0,
            count: 0,
            capacity: 0
        }
    }

    fn write_chunk(&mut self, byte: u8) {
        if self.capacity < self.count + 1 {
            let old_capacity:i32 = self.capacity;
            self.capacity = self.grow_capacity(old_capacity);
            self.code = self.grow_array(0, self.code, old_capacity, self.capacity);
        }

        self.code[self.count as usize] = byte;
        self.count +=1;
    }
}
fn main() {
    println!("Hello, world!");
}
