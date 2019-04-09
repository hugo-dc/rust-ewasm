// see ewasm "WRC20" pseudocode https://gist.github.com/axic/16158c5c88fbc7b1d09dfa8c658bc363
extern crate ewasm_api;

#[no_mangle]
pub fn main() {

    // 0x9993021a do_balance() ABI signature
    let do_balance_signature: [u8; 4] = [153, 147, 2, 26];

    // 0x5d359fbd do_transfer() ABI signature
    let do_transfer_signature: [u8; 4] = [93, 53, 159, 189];

    let data_size : usize = ewasm_api::calldata_size(); // getCallDataSize

    if data_size < 4 {
        ewasm_api::revert();
    }

    let function_selector: Vec<u8> = ewasm_api::unsafe_calldata_copy(0 as usize, 4 as usize); // callDataCopy

    //let transfer_test_bytes: [u8; 8] = [1, 1, 1, 1, 2, 2, 2, 2];
    //let raw_transfer_test_bytes = &transfer_test_bytes as *const u8;

    if function_selector == do_transfer_signature {
        if data_size != 32 {
            ewasm_api::revert();
        }

        //let caller_address: [u8;20] = ewasm_api::caller();

        // allocate 256 bits of memory because storageStore will read the whole 256 bits
        let recipient_offset: usize = 4;
        let recipient_length: usize = 20;

        let cd_result: Vec<u8> = ewasm_api::unsafe_calldata_copy(recipient_offset, recipient_length);
        let mut recipient: [u8; 32] = [0;32];
        for i in 0..recipient_length {
            recipient[i] = cd_result[i]
        }
        //let recipient: ArrayVec<_> = recipient.into_iter().collect();
        //let recipient: [u8; 32] = recipient.into_inner().unwrap();

        let recipient_test_key: [u8; 32] =
            [254, 254, 254, 254, 0, 0, 0, 0,
             0,     0,   0,   0, 0, 0, 0, 0,
             0,     0,   0,   0, 0, 0, 0, 0,
             0,     0,   0,   0, 0, 0, 0, 0];

        // write to sstore for quick test
        //ewasm_api::storage_store(&recipient_test_key, &recipient);

        // allocate 256 bits of memory because storageStore will read the whole 256 bits
        let value_offset: usize = 24;
        let value_length: usize = 8;
        let cb_result: Vec<u8> = ewasm_api::unsafe_calldata_copy(value_offset, value_length);
        let mut value: [u8; 32] = [0; 32];
        for i in 0..value_length {
            value[i] = cb_result[i];
        }
        //let value: ArrayVec<_> = value.into_iter().collect();
        //let value: [u8;32] = value.into_inner().unwrap();

        let value_test_key: [u8; 32] = [255, 255, 255, 255, 0, 0, 0, 0,
                                        0,     0,   0,   0, 0, 0, 0, 0,
                                        0,     0,   0,   0, 0, 0, 0, 0,
                                        0,     0,   0,   0, 0, 0, 0, 0];

        // write to sstore for quick test
        //ewasm_api::storage_store(&value_test_key, &value);
        return;
    }

    if function_selector == do_balance_signature {
        if data_size != 24 {
            ewasm_api::revert();
        }
        //                                de   ad   be   ef
        let main_address: Vec<u8> = vec![222, 173, 190, 239,
                                         0,     0,   0,   0,
                                         0,     0,   0,   0,
                                         0,     0,   0,   0,
                                         0,     0,   0,   0];

        let address = ewasm_api::unsafe_calldata_copy(4 as usize, 20 as usize); // calldatacopy
        /* let expected: [u8;32] = [15, 66, 64, 0, 0, 0, 0, 0,
                                 0, 0, 0, 0, 0, 0, 0, 0,
                                 0, 0, 0, 0, 0, 0, 0, 0,
                                 0, 0, 0, 0, 0, 0, 0, 0];
         */
        if main_address == address {
            let mut sstore_key: [u8;32] = [0;32];
        
            for i in 0..20 {
                sstore_key[i] = address[i];
            }

            let balance = ewasm_api::storage_load(&sstore_key); // storageLoad
            //                       

            //if balance[0] > 0 {
            ewasm_api::finish_data(&balance);
            //}
        }
    }

    return;
}
