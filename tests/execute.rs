extern crate zeerust;

use zeerust::z80;

const HELLO_ZEERUST: &'static [u8] = include_bytes!("zeerust.bin");

#[test]
fn hello_zeerust() {
    let mut z80 = z80::Z80::default();
    let buf = z80::io::BufOutput::default();
    z80.install_output(0x00, &buf);

    z80.run(HELLO_ZEERUST);

    assert_eq!(
        vec!(
            'Z' as u8,
            'E' as u8,
            'E' as u8,
            'R' as u8,
            'U' as u8,
            'S' as u8,
            'T' as u8,
        ),
        buf.result()
    );
}
