pub const MEMORY_SIZE: usize = 16 * 1024; // 16 kibibytes

pub struct Memory {
    pub memory: [u8; MEMORY_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            memory: [0; MEMORY_SIZE],
        }
    }
}
