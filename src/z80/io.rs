//! Methods associated with the IN and OUT instructions of the z80
use std::sync::Mutex;

use super::Z80;

/// An InputDevice can be read from, one byte at a time
pub trait InputDevice {
    /// Read a single byte
    fn input(&self) -> u8;
}

/// An OutputDevice can be written to, one byte at a time
pub trait OutputDevice {
    /// Write a single byte
    fn output(&self, val: u8);
}

impl<'a> Z80<'a> {
    /// Install an input device at the given index. For example:
    /// ```
    /// use zeerust::z80;
    ///
    /// let mut z80 = z80::Z80::default();
    /// let inp = z80::io::BufInput::new(vec!(b'Z'));
    /// z80.install_input(0, &inp);
    ///```
    /// This will then be usable with `IN (0), <register>`.
    pub fn install_input(&mut self, index: u8, device: &'a InputDevice) {
        self.input_devices.insert(index, device);
    }

    /// Install an output device at the given index. For example:
    /// ```
    /// use zeerust::z80;
    ///
    /// let mut z80 = z80::Z80::default();
    /// let out = z80::io::BufOutput::default();
    /// z80.install_output(0, &out);
    ///```
    /// This will then be usable with `OUT (0), <register>`.
    pub fn install_output(&mut self, index: u8, device: &'a OutputDevice) {
        self.output_devices.insert(index, device);
    }
}

/// BufInput is a simple InputDevice than produces input when requested, from back to front.
/// Useful in tests.
#[derive(Default)]
pub struct BufInput {
    input: Mutex<Vec<u8>>,
}

impl InputDevice for BufInput {
    /// Read the right-most byte from the internal buffer
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

/// BufOutput is a simple Output device that recieves output and appends it to an internal vector.
#[derive(Default)]
pub struct BufOutput {
    output: Mutex<Vec<u8>>,
}

impl BufOutput {
    /// All of the outputs recieved from the processor, most recent last.
    pub fn result(&self) -> Vec<u8> {
        self.output.lock().unwrap().to_vec()
    }
}

impl OutputDevice for BufOutput {
    /// Write a byte to the end of the internal buffer
    fn output(&self, val: u8) {
        self.output.lock().unwrap().push(val)
    }
}
