extern crate zeerust;

use zeerust::examples::{COUNTDOWN_BIN, FIZZBUZZ_BIN, HELLO_WORLD_BIN, HELLO_ZEERUST_BIN};
use zeerust::z80;

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
    assert_eq!(b"ZEERUST".to_vec(), run(HELLO_ZEERUST_BIN));
}

#[test]
fn hello_world() {
    assert_eq!(b"Hello World\n".to_vec(), run(HELLO_WORLD_BIN))
}

#[test]
fn countdown() {
    assert_eq!(b"9\n8\n7\n6\n5\n4\n3\n2\n1\n".to_vec(), run(COUNTDOWN_BIN))
}

#[test]
fn fizzbuzz() {
    let expected: Vec<u8> = [
        "01", "02", "Fizz", "04", "Buzz", "Fizz", "07", "08", "Fizz", "Buzz", "11", "Fizz", "13",
        "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz\n",
    ]
    .join("\n")
    .as_bytes()
    .to_vec();
    assert_eq!(expected, run(FIZZBUZZ_BIN));
}
