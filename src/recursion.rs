use crate::*;

pub unsafe fn verify_proof(
    vk_fields_ptr: &[u8],
    proof_fields: &[u8],
    public_inputs: &[u8],
    input_aggregation_obj_ptr: &[u8],
    output_aggregation_obj_ptr: *mut *mut u8,
) -> usize {

    acir_proofs_verify_recursive_proof(
        proof_fields.as_ptr() as *const u8,
        proof_fields.len() as u32,
        vk_fields_ptr.as_ptr() as *const u8,
        vk_fields_ptr.len() as u32,
        public_inputs.as_ptr() as *const u8,
        input_aggregation_obj_ptr.as_ptr() as *const u8,
        output_aggregation_obj_ptr,
    )
}