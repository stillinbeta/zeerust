extern crate zeerust;

use zeerust::z80;

#[test]
fn hello_zeerust() {
    let z80 = z80::Z80::default();
    let buf = z80::io::BufOutput::default();
}
