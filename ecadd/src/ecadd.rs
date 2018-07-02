// bn128 point addition copied from https://github.com/ethereumjs/rustbn.js

extern "C" {
    fn callDataCopy(resultOffset: *const u8, dataOffset: u32, length: u32);
    fn eeiReturn(dataOffset: *const u32, length: u32);
    fn print32(data: u32);
}

extern crate ewasm_api;
extern crate bn;

use std::io::{self, Read};

pub struct Error(pub &'static str);

impl From<&'static str> for Error {
    fn from(val: &'static str) -> Self {
	Error(val)
    }
}

fn read_point(reader: &mut io::Chain<&[u8], io::Repeat>) -> Result<::bn::G1, Error> {
    use bn::{Fq, AffineG1, G1, Group};

    let mut buf = [0u8; 32];

    reader.read_exact(&mut buf[..]).expect("reading from zero-extended memory cannot fail; qed");
    let px = Fq::from_slice(&buf[0..32]).map_err(|_| Error::from("Invalid point x coordinate"))?;

    reader.read_exact(&mut buf[..]).expect("reading from zero-extended memory cannot fail; qed");
    let py = Fq::from_slice(&buf[0..32]).map_err(|_| Error::from("Invalid point y coordinate"))?;

    Ok(
	if px == Fq::zero() && py == Fq::zero() {
	    G1::zero()
	} else {
	    AffineG1::new(px, py).map_err(|_| Error::from("Invalid curve point"))?.into()
	}
    )
}

#[no_mangle]
pub fn main() {
	use bn::AffineG1;

    // try input from https://github.com/ethereum/tests/blob/9741ed0bc1fb660c5ffefd751c24bc739104ce5e/src/GeneralStateTestsFiller/stZeroKnowledge/pointAddFiller.json#L179
    // output should be https://github.com/ethereum/tests/blob/9741ed0bc1fb660c5ffefd751c24bc739104ce5e/src/GeneralStateTestsFiller/stZeroKnowledge/pointAddFiller.json#L40

    let input: [u8; 128] = [0;128];
    //let mut input: Vec<u8> = Vec::new();
    let input_offset: u32 = 0;
    let input_length: usize = 128;
    let output_length: usize = 64;
    //input.reserve(input_length);

    //let ptr_input = input.as_mut_ptr();
    let ptr_input = &input as *const u8;

    
    unsafe {
        callDataCopy(ptr_input, input_offset, input_length as u32);
    }
    

    // rust api specific:
    //=================================
    
    let input_offset_usize: usize = 0;
    let input_length_usize: usize = 128;

    let data_input = ewasm_api::calldata_copy(input_offset_usize, input_length_usize);

    //let ptr_input = data_input.as_ptr();
    //---------------------------------

    // Vec<u8> length 128
    let mut padded_input = input.chain(io::repeat(0));

    let mut padded_buf = [0u8; 128];
    padded_input.read_exact(&mut padded_buf[..]).expect("reading from zero-extended memory cannot fail; qed");

    let point1 = &padded_buf[0..64];
    let point2 = &padded_buf[64..128];

    let mut point1_padded = point1.chain(io::repeat(0));
    let mut point2_padded = point2.chain(io::repeat(0));

    let p1;
    match read_point(&mut point1_padded) {
        Ok(p) => {
            p1 = p;
        },
        Err(_) => { return; }
    }

    match read_point(&mut point2_padded) {
        Ok(p) => {
            let p2 = p;
            let mut ecadd_output_buf = [0u8; 64];
            if let Some(sum) = AffineG1:: from_jacobian(p1 + p2) {
                sum.x().to_big_endian(&mut ecadd_output_buf[0..32]).expect("Cannot fail since 0..32 is 32-byte length");
                sum.y().to_big_endian(&mut ecadd_output_buf[32..64]).expect("Cannot fail since 32..64 is 32-byte length");
            }

            // Replace calculated result, use hardcoded values
            //let mut ecadd_output_buf2 = [0u8; 64];
            //ecadd_output_buf2 = [0x03, 0x06, 0x44, 0xe7, 0x2e, 0x13, 0x1a, 0x02, 0x9b, 0x85, 0x04, 0x5b, 0x68, 0x18, 0x15, 0x85, 0xd9, 0x78, 0x16, 0xa9, 0x16, 0x87, 0x1c, 0xa8, 0xd3, 0xc2, 0x08, 0xc1, 0x6d, 0x87, 0xcf, 0xd3,
            //                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

            //let raw_ecadd_result = &ecadd_output_buf as *const u8;

            let vec_buf = ecadd_output_buf.to_vec();
            //let raw_vec_buf = &vec_buf as *const u32;

            unsafe {
                // TODO: result is backwards (endianness)
                eeiReturn(vec_buf.as_ptr() as *const u32, output_length as u32);
            }


            /*
            unsafe{
                for i in (0..64) {
                    print32(vec_buf[i] as u32);
                }
            }
            */


            // rust api specific:
            //=================================
            //ewasm_api::finish_data(vec_buf);
            //---------------------------------
            return;
        },
        Err(_) => { return; }
    }

}
