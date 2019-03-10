use std::sync::Mutex;

use super::Z80;

pub trait InputDevice {
    fn input(&self) -> u8;
}

pub trait OutputDevice {
    fn output(&self, val: u8);
}

impl<'a> Z80<'a> {
    pub fn install_input(&mut self, index: u8, device: &'a InputDevice) {
        self.input_devices.insert(index, device);
    }

    pub fn install_output(&mut self, index: u8, device: &'a OutputDevice) {
        self.output_devices.insert(index, device);
    }
}

#[derive(Default)]
pub struct BufInput {
    input: Mutex<Vec<u8>>,
}

impl InputDevice for BufInput {
    fn input(&self) -> u8 {
        self.input.lock().unwrap().pop().unwrap()
    }
}

impl BufInput {
    pub fn new(v: Vec<u8>) -> Self {
        Self {
            input: Mutex::new(v),
        }
    }
}

#[derive(Default)]
pub struct BufOutput {
    output: Mutex<Vec<u8>>,
}

impl BufOutput {
    pub fn result(&self) -> Vec<u8> {
        self.output.lock().unwrap().to_vec()
    }
}

impl OutputDevice for BufOutput {
    fn output(&self, val: u8) {
        self.output.lock().unwrap().push(val)
    }
}
