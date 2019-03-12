use super::opcode;
use crate::ops::Op;

pub fn parse_stream(stream: Vec<u8>) -> Vec<Op> {
    let mut i = 0;
    let mut ops = vec![];

    while i < stream.len() {
        let opr: [u8; 4] = [
            stream[i],
            stream.get(i + 1).map_or(0x00, |i| *i),
            stream.get(i + 2).map_or(0x00, |i| *i),
            stream.get(i + 3).map_or(0x00, |i| *i),
        ];
        let (opc, consumed) = opcode(opr);
        ops.push(opc);
        i += consumed;
    }
    ops
}
