extern crate zeerust;

use std::env;
use std::fs::File;
use std::io::{stdout, Read, Result, Write};

use zeerust::z80;
use zeerust::z80::io;

struct StdoutOutput {}

impl io::OutputDevice for StdoutOutput {
    fn output(&self, byte: u8) {
        let _ = stdout().write(&[byte]);
    }
}

const STDOUT_DEVICE: StdoutOutput = StdoutOutput {};

fn main() -> Result<()> {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Missing file to run");
        std::process::exit(1);
    });
    let mut file = File::open(filename)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let mut z80 = z80::Z80::default();
    z80.install_output(0x00, &STDOUT_DEVICE);
    z80.load(buf.as_slice());
    z80.run();
    Ok(())
}
