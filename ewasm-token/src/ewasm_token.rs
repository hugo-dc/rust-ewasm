// see ewasm "WRC20" pseudocode https://gist.github.com/axic/16158c5c88fbc7b1d09dfa8c658bc363
extern crate ewasm_api;
use ewasm_api::types::*;

extern "C" {
    fn storageStore(keyOffset: *const u32, valueOffset: *const u32);
}

#[no_mangle]
pub fn main() {

    let data_size = ewasm_api::calldata_size(); // calldatasize
    
    if data_size != 24 {
        ewasm_api::revert();
    }
    
    //                                de   ad   be   ef
    let main_address: Vec<u8> = vec![222, 173, 190, 239,
                                     0,     0,   0,   0,
                                     0,     0,   0,   0,
                                     0,     0,   0,   0,
                                     0,     0,   0,   0];

    let mut address: Vec<u8> = Vec::new();

    ewasm_api::unsafe_calldata_copy(4 as usize, 20 as usize, &mut address); // calldatacopy

    let mut sstore_key = Bytes32 { bytes: [0;32] };

        
    for i in 0..address.len() {
        sstore_key.bytes[i] = address[i];
    }

    let balance = ewasm_api::storage_load(&sstore_key); // storageLoad

    let mut value_test_key = StorageKey {
        bytes: [0, 0, 0, 0, 0, 0, 0, 0,
                0,     0,   0,   0, 0, 0, 0, 0,
                0,     0,   0,   0, 0, 0, 0, 0,
                0,     0,   0,   0, 0, 0, 0, 1]
    };

    let storage_value = Bytes32 { bytes: address[0..32] };
    
    ewasm_api::storage_store(&value_test_key, &address);
    
    ewasm_api::finish_data(&balance.bytes);
    return;
}
