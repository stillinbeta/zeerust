extern crate zeerust;

use zeerust::z80;

const HELLO_ZEERUST: &[u8] = include_bytes!("zeerust.bin");
const HELLO_WORLD: &[u8] = include_bytes!("hello_world.bin");
const COUNTDOWN: &[u8] = include_bytes!("countdown.bin");

fn run(program: &[u8]) -> Vec<u8> {
    let mut z80 = z80::Z80::default();
    let buf = z80::io::BufOutput::default();
    z80.install_output(0x00, &buf);

    z80.load(program);
    z80.run();
    buf.result()
}

#[test]
fn hello_zeerust() {
    assert_eq!(b"ZEERUST".to_vec(), run(HELLO_ZEERUST));
}

#[test]
fn hello_world() {
    assert_eq!(b"Hello World\n".to_vec(), run(HELLO_WORLD))
}

#[test]
fn countdown() {
    assert_eq!(b"9\n8\n7\n6\n5\n4\n3\n2\n1\n".to_vec(), run(COUNTDOWN))
}
