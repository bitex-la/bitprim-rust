/*
pub type block_indexes_t = *mut ::std::os::raw::c_void;
pub type block_list_t = *mut ::std::os::raw::c_void;
pub type compact_block_t = *mut ::std::os::raw::c_void;
pub type history_compact_t = *mut ::std::os::raw::c_void;
pub type history_compact_list_t = *mut ::std::os::raw::c_void;
pub type input_t = *mut ::std::os::raw::c_void;
pub type input_list_t = *mut ::std::os::raw::c_void;
pub type input_point_t = *mut ::std::os::raw::c_void;
pub type merkle_block_t = *mut ::std::os::raw::c_void;
pub type script_t = *mut ::std::os::raw::c_void;
pub type output_t = *mut ::std::os::raw::c_void;
pub type output_list_t = *mut ::std::os::raw::c_void;
pub type output_point_t = *mut ::std::os::raw::c_void;
pub type point_t = *mut ::std::os::raw::c_void;
pub type point_list_t = *mut ::std::os::raw::c_void;
pub type transaction_t = *mut ::std::os::raw::c_void;
pub type transaction_list_t = *mut ::std::os::raw::c_void;
pub type get_blocks_t = *mut ::std::os::raw::c_void;
pub type get_blocks_ptr_t = *mut ::std::os::raw::c_void;
pub type get_headers_t = *mut ::std::os::raw::c_void;
pub type get_headers_ptr_t = *mut ::std::os::raw::c_void;
pub type payment_address_t = *mut ::std::os::raw::c_void;
pub type binary_t = *mut ::std::os::raw::c_void;
pub type stealth_compact_t = *mut ::std::os::raw::c_void;
pub type stealth_compact_list_t = *mut ::std::os::raw::c_void;
pub type hash_list_t = *mut ::std::os::raw::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct short_hash_t {
    pub hash: [u8; 20usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hash_t {
    pub hash: [u8; 32usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct long_hash_t {
    pub hash: [u8; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ec_secret_t {
    pub data: [u8; 32usize],
}
pub type ec_public_t = *mut ::std::os::raw::c_void;
pub type hd_private_t = *mut ::std::os::raw::c_void;
pub type word_list_t = *mut ::std::os::raw::c_void;

pub type stealth_fetch_handler_t =
    ::std::option::Option<
        unsafe extern "C" fn(arg1: chain_t,
                             arg2: *mut ::std::os::raw::c_void,
                             arg3: error_code_t,
                             arg4: stealth_compact_list_t),
    >;
pub type history_fetch_handler_t =
    ::std::option::Option<
        unsafe extern "C" fn(arg1: chain_t,
                             arg2: *mut ::std::os::raw::c_void,
                             arg3: error_code_t,
                             arg4: history_compact_list_t),
    >;
pub type output_fetch_handler_t =
    ::std::option::Option<
        unsafe extern "C" fn(arg1: chain_t,
                             arg2: *mut ::std::os::raw::c_void,
                             arg3: error_code_t,
                             output: output_t),
    >;
pub type spend_fetch_handler_t =
    ::std::option::Option<
        unsafe extern "C" fn(arg1: chain_t,
                             arg2: *mut ::std::os::raw::c_void,
                             arg3: error_code_t,
                             arg4: input_point_t),
    >;
pub type transaction_fetch_handler_t =
    ::std::option::Option<
        unsafe extern "C" fn(arg1: chain_t,
                             arg2: *mut ::std::os::raw::c_void,
                             arg3: error_code_t,
                             arg4: transaction_t,
                             arg5: u64,
                             arg6: u64),
    >;
pub type transaction_index_fetch_handler_t =
    ::std::option::Option<
        unsafe extern "C" fn(arg1: chain_t,
                             arg2: *mut ::std::os::raw::c_void,
                             arg3: error_code_t,
                             arg4: u64,
                             arg5: u64),
    >;
pub type validate_tx_handler_t =
    ::std::option::Option<
        unsafe extern "C" fn(arg1: chain_t,
                             arg2: *mut ::std::os::raw::c_void,
                             arg3: error_code_t,
                             arg4: *const ::std::os::raw::c_char),
    >;
pub type block_locator_fetch_handler_t =
    ::std::option::Option<
        unsafe extern "C" fn(arg1: chain_t,
                             arg2: *mut ::std::os::raw::c_void,
                             arg3: error_code_t,
                             arg4: get_headers_ptr_t),
    >;
pub type result_handler_t =
    ::std::option::Option<
        unsafe extern "C" fn(arg1: chain_t,
                             arg2: *mut ::std::os::raw::c_void,
                             arg3: error_code_t),
    >;
pub type subscribe_blockchain_handler_t =
    ::std::option::Option<
        unsafe extern "C" fn(arg1: executor_t,
                             arg2: chain_t,
                             arg3: *mut ::std::os::raw::c_void,
                             arg4: error_code_t,
                             arg5: u64,
                             arg6: block_list_t,
                             arg7: block_list_t)
                             -> ::std::os::raw::c_int,
    >;
pub type subscribe_transaction_handler_t =
    ::std::option::Option<
        unsafe extern "C" fn(arg1: executor_t,
                             arg2: chain_t,
                             arg3: *mut ::std::os::raw::c_void,
                             arg4: error_code_t,
                             arg5: transaction_t)
                             -> ::std::os::raw::c_int,
    >;

extern "C" {
    pub fn stealth_compact_get_ephemeral_public_key_hash(stealth: stealth_compact_t) -> hash_t;
    pub fn stealth_compact_get_ephemeral_public_key_hash_out(
        stealth: stealth_compact_t,
        out_epk_hash: *mut hash_t,
    );
    pub fn stealth_compact_get_transaction_hash(stealth: stealth_compact_t) -> hash_t;
    pub fn stealth_compact_get_transaction_hash_out(
        stealth: stealth_compact_t,
        out_tx_hash: *mut hash_t,
    );
    pub fn stealth_compact_get_public_key_hash(stealth: stealth_compact_t) -> short_hash_t;
    pub fn stealth_compact_get_public_key_hash_out(
        stealth: stealth_compact_t,
        out_pk_hash: *mut short_hash_t,
    );
}
extern "C" {
    pub fn stealth_compact_list_destruct(list: stealth_compact_list_t);
    pub fn stealth_compact_list_count(list: stealth_compact_list_t) -> u64;
    pub fn stealth_compact_list_nth(list: stealth_compact_list_t, n: u64) -> stealth_compact_t;
}
extern "C" {
    pub fn chain_input_construct_default() -> input_t;
    pub fn chain_input_construct(
        previous_output: output_point_t,
        script: script_t,
        sequence: u32,
    ) -> input_t;
    pub fn chain_input_destruct(input: input_t);
    pub fn chain_input_is_valid(input: input_t) -> ::std::os::raw::c_int;
    pub fn chain_input_is_final(input: input_t) -> ::std::os::raw::c_int;
    pub fn chain_input_serialized_size(input: input_t, wire: ::std::os::raw::c_int) -> u64;
    pub fn chain_input_sequence(input: input_t) -> u32;
    pub fn chain_input_signature_operations(
        input: input_t,
        bip16_active: ::std::os::raw::c_int,
    ) -> u64;
    pub fn chain_input_script(input: input_t) -> script_t;
    pub fn chain_input_previous_output(input: input_t) -> output_point_t;
}
extern "C" {
    pub fn chain_input_list_construct_default() -> input_list_t;
    pub fn chain_input_list_push_back(list: input_list_t, input: input_t);
    pub fn chain_input_list_destruct(list: input_list_t);
    pub fn chain_input_list_count(list: input_list_t) -> u64;
    pub fn chain_input_list_nth(list: input_list_t, n: u64) -> input_t;
}
extern "C" {
    pub fn chain_merkle_block_hash_nth(block: merkle_block_t, n: u64) -> hash_t;
    pub fn chain_merkle_block_hash_nth_out(block: merkle_block_t, n: u64, out_hash: *mut hash_t);
    pub fn chain_merkle_block_header(block: merkle_block_t) -> header_t;
    pub fn chain_merkle_block_is_valid(block: merkle_block_t) -> ::std::os::raw::c_int;
    pub fn chain_merkle_block_hash_count(block: merkle_block_t) -> u64;
    pub fn chain_merkle_block_serialized_size(block: merkle_block_t, version: u32) -> u64;
    pub fn chain_merkle_block_total_transaction_count(block: merkle_block_t) -> u64;
    pub fn chain_merkle_block_destruct(block: merkle_block_t);
    pub fn chain_merkle_block_reset(block: merkle_block_t);
}
extern "C" {
    pub fn chain_output_construct_default() -> output_t;
    pub fn chain_output_construct(value: u64, script: script_t) -> output_t;
    pub fn chain_output_destruct(output: output_t);
    pub fn chain_output_is_valid(output: output_t) -> ::std::os::raw::c_int;
    pub fn chain_output_serialized_size(output: output_t, wire: ::std::os::raw::c_int) -> u64;
    pub fn chain_output_value(output: output_t) -> u64;
    pub fn chain_output_signature_operations(output: output_t) -> u64;
    pub fn chain_output_script(output: output_t) -> script_t;
    pub fn chain_output_payment_address(
        output: output_t,
        use_testnet_rules: ::std::os::raw::c_int,
    ) -> payment_address_t;
}
extern "C" {
    pub fn chain_output_list_construct_default() -> output_list_t;
    pub fn chain_output_list_push_back(list: output_list_t, output: output_t);
    pub fn chain_output_list_destruct(list: output_list_t);
    pub fn chain_output_list_count(list: output_list_t) -> u64;
    pub fn chain_output_list_nth(list: output_list_t, n: u64) -> output_t;
}
extern "C" {
    pub fn output_point_get_hash(op: output_point_t) -> hash_t;
    pub fn output_point_get_hash_out(op: output_point_t, out_hash: *mut hash_t);
    pub fn output_point_construct() -> output_point_t;
    pub fn output_point_construct_from_hash_index(hash: hash_t, index: u32) -> output_point_t;
    pub fn output_point_get_index(output: output_point_t) -> u32;
    pub fn output_point_destruct(op: output_point_t);
}
extern "C" {
    pub fn chain_payment_address_encoded(
        payment_address: payment_address_t,
    ) -> *const ::std::os::raw::c_char;
    pub fn chain_payment_address_construct_from_string(
        address: *const ::std::os::raw::c_char,
    ) -> payment_address_t;
    pub fn chain_payment_address_version(payment_address: payment_address_t) -> u8;
    pub fn chain_payment_address_is_valid(
        payment_address: payment_address_t,
    ) -> ::std::os::raw::c_int;
    pub fn chain_payment_address_destruct(payment_address: payment_address_t);
}
extern "C" {
    pub fn chain_point_get_hash(point: point_t) -> hash_t;
    pub fn chain_point_get_hash_out(point: point_t, out_hash: *mut hash_t);
    pub fn chain_point_is_valid(point: point_t) -> ::std::os::raw::c_int;
    pub fn chain_point_get_index(point: point_t) -> u32;
    pub fn chain_point_get_checksum(point: point_t) -> u64;
}
extern "C" {
    pub fn point_list_nth(point_list: point_list_t, n: u64) -> point_t;
    pub fn point_list_count(point_list: point_list_t) -> u64;
    pub fn point_list_destruct(point_list: point_list_t);
}
extern "C" {
    pub fn chain_script_destruct(script: script_t);
    pub fn chain_script_is_valid(script: script_t) -> ::std::os::raw::c_int;
    pub fn chain_script_is_valid_operations(script: script_t) -> ::std::os::raw::c_int;
    pub fn chain_script_satoshi_content_size(script: script_t) -> u64;
    pub fn chain_script_serialized_size(script: script_t, prefix: ::std::os::raw::c_int) -> u64;
    pub fn chain_script_to_string(
        script: script_t,
        active_forks: u32,
    ) -> *const ::std::os::raw::c_char;
    pub fn chain_script_type(script: script_t) -> *const ::std::os::raw::c_char;
    pub fn chain_script_to_data(
        script: script_t,
        prefix: ::std::os::raw::c_int,
        out_size: *mut u64,
    ) -> *const u8;
    pub fn chain_script_sigops(script: script_t, embedded: ::std::os::raw::c_int) -> u64;
    pub fn chain_script_embedded_sigops(script: script_t, prevout_script: script_t) -> u64;
}

extern "C" {
    pub fn chain_transaction_construct_default() -> transaction_t;
    pub fn chain_transaction_construct(
        version: u32,
        locktime: u32,
        inputs: input_list_t,
        outputs: output_list_t,
    ) -> transaction_t;
    pub fn chain_transaction_destruct(transaction: transaction_t);
    pub fn chain_transaction_is_valid(transaction: transaction_t) -> ::std::os::raw::c_int;
    pub fn chain_transaction_version(transaction: transaction_t) -> u32;
    pub fn chain_transaction_set_version(transaction: transaction_t, version: u32);
    pub fn chain_transaction_hash(transaction: transaction_t) -> hash_t;
    pub fn chain_transaction_hash_out(transaction: transaction_t, out_hash: *mut hash_t);
    pub fn chain_transaction_hash_sighash_type(
        transaction: transaction_t,
        sighash_type: u32,
    ) -> hash_t;
    pub fn chain_transaction_hash_sighash_type_out(
        transaction: transaction_t,
        sighash_type: u32,
        out_hash: *mut hash_t,
    );
    pub fn chain_transaction_locktime(transaction: transaction_t) -> u32;
    pub fn chain_transaction_serialized_size(
        transaction: transaction_t,
        wire: ::std::os::raw::c_int,
    ) -> u64;
    pub fn chain_transaction_fees(transaction: transaction_t) -> u64;
    pub fn chain_transaction_signature_operations(transaction: transaction_t) -> u64;
    pub fn chain_transaction_signature_operations_bip16_active(
        transaction: transaction_t,
        bip16_active: ::std::os::raw::c_int,
    ) -> u64;
    pub fn chain_transaction_total_input_value(transaction: transaction_t) -> u64;
    pub fn chain_transaction_total_output_value(transaction: transaction_t) -> u64;
    pub fn chain_transaction_is_coinbase(transaction: transaction_t) -> ::std::os::raw::c_int;
    pub fn chain_transaction_is_null_non_coinbase(
        transaction: transaction_t,
    ) -> ::std::os::raw::c_int;
    pub fn chain_transaction_is_oversized_coinbase(
        transaction: transaction_t,
    ) -> ::std::os::raw::c_int;
    pub fn chain_transaction_is_mature(
        transaction: transaction_t,
        target_height: u64,
    ) -> ::std::os::raw::c_int;
    pub fn chain_transaction_is_overspent(transaction: transaction_t) -> ::std::os::raw::c_int;
    pub fn chain_transaction_is_double_spend(
        transaction: transaction_t,
        include_unconfirmed: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn chain_transaction_is_missing_previous_outputs(
        transaction: transaction_t,
    ) -> ::std::os::raw::c_int;
    pub fn chain_transaction_is_final(
        transaction: transaction_t,
        block_height: u64,
        block_time: u32,
    ) -> ::std::os::raw::c_int;
    pub fn chain_transaction_is_locktime_conflict(
        transaction: transaction_t,
    ) -> ::std::os::raw::c_int;
    pub fn chain_transaction_outputs(transaction: transaction_t) -> output_list_t;
    pub fn chain_transaction_inputs(transaction: transaction_t) -> input_list_t;
    pub fn chain_transaction_to_data(
        script: script_t,
        wire: ::std::os::raw::c_int,
        out_size: *mut u64,
    ) -> *const u8;
}

extern "C" {
    pub fn chain_transaction_list_construct_default() -> transaction_list_t;
    pub fn chain_transaction_list_push_back(list: transaction_list_t, transaction: transaction_t);
    pub fn chain_transaction_list_destruct(list: transaction_list_t);
    pub fn chain_transaction_list_count(list: transaction_list_t) -> u64;
    pub fn chain_transaction_list_nth(list: transaction_list_t, n: u64) -> transaction_t;
}

extern "C" {
    pub fn p2p_address_count(p2p: p2p_t) -> u64;
}

extern "C" {
    pub fn wallet_mnemonics_to_seed(mnemonics: word_list_t) -> long_hash_t;
}
extern "C" {
    pub fn wallet_ec_new(seed: *mut u8, n: u64) -> ec_secret_t;
    pub fn wallet_ec_to_public(
        secret: ec_secret_t,
        uncompressed: ::std::os::raw::c_int,
    ) -> ec_public_t;
    pub fn wallet_ec_to_address(point: ec_public_t, version: u32) -> payment_address_t;
}
extern "C" {
    pub fn wallet_hd_new(seed: *mut u8, n: u64, version: u32) -> hd_private_t;
    pub fn wallet_hd_private_to_ec(key: hd_private_t) -> ec_secret_t;
}

extern "C" {
    pub fn word_list_construct() -> word_list_t;
    pub fn word_list_add_word(word_list: word_list_t, word: *const ::std::os::raw::c_char);
    pub fn word_list_destruct(word_list: word_list_t);
}

*/
