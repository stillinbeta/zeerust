extern crate zeerust;

use zeerust::z80;

const HELLO_ZEERUST: &[u8] = include_bytes!("zeerust.bin");

#[test]
fn hello_zeerust() {
    let mut z80 = z80::Z80::default();
    let buf = z80::io::BufOutput::default();
    z80.install_output(0x00, &buf);

    z80.run(HELLO_ZEERUST);

    assert_eq!(vec!(b'Z', b'E', b'E', b'R', b'U', b'S', b'T'), buf.result());
}
