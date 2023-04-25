use crate::*;

/// # Safety
pub unsafe fn get_solidity_verifier(
    g2_ptr: &[u8],
    vk_ptr: &[u8],
    output_buf: *mut *mut u8,
) -> usize {
    acir_proofs_get_solidity_verifier(
        g2_ptr.as_ptr() as *const u8,
        vk_ptr.as_ptr() as *const u8,
        output_buf,
    )
}

/// # Safety
/// cs_prt must point to a valid constraints system structure of type standard_format
pub unsafe fn get_exact_circuit_size(cs_prt: *const u8) -> u32 {
    acir_proofs_get_exact_circuit_size(cs_prt)
}

/// # Safety
/// cs_prt must point to a valid constraints system structure of type standard_format
pub unsafe fn get_total_circuit_size(cs_prt: *const u8) -> u32 {
    acir_proofs_get_total_circuit_size(cs_prt)
}

/// # Safety
/// cs_prt must point to a valid constraints system structure of type standard_format
pub unsafe fn init_proving_key(cs_ptr: &[u8], pk_data_ptr: *mut *mut u8) -> usize {
    let cs_ptr = cs_ptr.as_ptr();
    acir_proofs_init_proving_key(cs_ptr, pk_data_ptr as *const *mut u8 as *mut *const u8)
}

/// # Safety
/// pippenger must point to a valid Pippenger object
pub unsafe fn init_verification_key(
    pippenger: *mut ::std::os::raw::c_void,
    g2_ptr: &[u8],
    pk_ptr: &[u8],
    vk_data_ptr: *mut *mut u8,
) -> usize {
    acir_proofs_init_verification_key(
        pippenger,
        g2_ptr.as_ptr() as *const u8,
        pk_ptr.as_ptr() as *const u8,
        vk_data_ptr as *const *mut u8 as *mut *const u8,
    )
}

pub unsafe fn serialize_verification_key_into_field_elements(
    g2_ptr: &[u8],
    vk_buf: &[u8],
    serialized_vk_buf: *mut *mut u8,
    serialized_vk_hash_buf: *mut *mut u8,
) -> usize {
    acir_serialize_verification_key_into_field_elements(
        g2_ptr.as_ptr() as *const u8,
        vk_buf.as_ptr() as *const u8,
        serialized_vk_buf as *const *mut u8 as *mut *mut u8,
        serialized_vk_hash_buf as *const *mut u8 as *mut *mut u8,
    )
}

pub unsafe fn serialize_proof_into_field_elements(
    proof: &[u8],
    serialized_proof_data_buf: *mut *mut u8,
    proof_data_length: usize,
) -> usize {
    acir_serialize_proof_into_field_elements(
        proof.as_ptr() as *const u8,
        serialized_proof_data_buf,
        proof_data_length,
    )
}

/// # Safety
/// pippenger must point to a valid Pippenger object
pub unsafe fn create_proof_with_pk(
    pippenger: *mut ::std::os::raw::c_void,
    g2_ptr: &[u8],
    pk_ptr: &[u8],
    cs_ptr: &[u8],
    witness_ptr: &[u8],
    proof_data_ptr: *mut *mut u8,
    is_recursive: bool,
) -> usize {
    let cs_ptr = cs_ptr.as_ptr() as *const u8;
    let pk_ptr = pk_ptr.as_ptr() as *const u8;
    acir_proofs_new_proof(
        pippenger,
        g2_ptr.as_ptr(),
        pk_ptr,
        cs_ptr,
        witness_ptr.as_ptr(),
        proof_data_ptr as *const *mut u8 as *mut *mut u8,
        is_recursive,
    )
}

/// # Safety
/// cs_prt must point to a valid constraints system structure of type standard_format
pub unsafe fn verify_with_vk(
    g2_ptr: &[u8],
    vk_ptr: &[u8],
    cs_ptr: &[u8],
    proof: &[u8],
    is_recursive: bool,
) -> bool {
    let proof_ptr = proof.as_ptr() as *const u8;

    acir_proofs_verify_proof(
        g2_ptr.as_ptr() as *const u8,
        vk_ptr.as_ptr() as *const u8,
        cs_ptr.as_ptr() as *const u8,
        proof_ptr as *mut u8,
        proof.len() as u32,
        is_recursive,
    )
}
