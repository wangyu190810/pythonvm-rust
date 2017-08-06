pub mod decode;

use std::io;
use super::objects::{ObjectRef, ObjectStore, PrimitiveObjects};


pub fn read_object<R: io::Read>(reader: &mut R, store: &mut ObjectStore, primitive_objects: &PrimitiveObjects) -> Result<ObjectRef, decode::UnmarshalError> {
    decode::read_object(reader, store, primitive_objects, &mut Vec::new())
}

pub fn check_magic(buf: &[u8]) -> bool {
    // See CPython's Lib/importlib/_bootstrap_external.py for a list of magic numbers per version
    println!("{:?}", &buf);
    if buf.len() != 4 {
        panic!("Magic token should be of size 4.")
    }
    let version = buf[0] as u16 + ((buf[1] as u16) << 8);
    println!("{:?}", &version);
    if version >= 20121 { // < 3.0
        false
    }
    else {
        println!("{:?}", &version);
        // 3310 <= version /* ≥ 3.4rc2 */ && version <= 3370 /* < 3.7 */
        //#     Python 3.7a0  3390 (add LOAD_METHOD and CALL_METHOD opcodes)
        3310 <= version /* ≥ 3.4rc2 */ && version <= 3390 /* < 3.7 */
    }
}
