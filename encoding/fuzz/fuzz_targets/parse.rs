#![no_main]

use libfuzzer_sys::fuzz_target;
use lorawan::parser::{parse, PhyPayload};

fuzz_target!(|data: &[u8]| {
    let owned_copy = data.to_vec();
    
    let parsed = match parse(owned_copy) {
        Ok(parsed) => parsed,
        Err(_) => return,
    }

    match parsed {
        JoinRequest(join_request_payload) => {

        },
        JoinAccept(join_accept_payload) => {

        },
        Data(data_payload) => {

        },
    }
});