use crate::*;

/// # Safety
/// vk_fields_ptr and proof_fields must point to a valid recursion format structure
/// laid out in the acir format recursion constraint
/// input_aggregation_obj_ptr must point to a valid aggregation object whose
/// structure is also laid out in the acir format recursion constraint  
pub unsafe fn verify_proof(
    vk_fields_ptr: &[u8],
    proof_fields: &[u8],
    num_public_inputs: u32,
    input_aggregation_obj_ptr: &[u8],
    output_aggregation_obj_ptr: *mut *mut u8,
) -> usize {
    acir_proofs_verify_recursive_proof(
        proof_fields.as_ptr() as *const u8,
        proof_fields.len() as u32,
        vk_fields_ptr.as_ptr() as *const u8,
        vk_fields_ptr.len() as u32,
        num_public_inputs,
        input_aggregation_obj_ptr.as_ptr() as *const u8,
        output_aggregation_obj_ptr,
    )
}
