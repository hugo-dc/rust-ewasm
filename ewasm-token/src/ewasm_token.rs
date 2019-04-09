// see ewasm "WRC20" pseudocode https://gist.github.com/axic/16158c5c88fbc7b1d09dfa8c658bc363
extern crate ewasm_api;

#[no_mangle]
pub fn main() {

    let data_size = ewasm_api::calldata_size();

    
    
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
    return;
}
