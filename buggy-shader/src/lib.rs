#![feature(asm)]
#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]

use spirv_std::glam::Mat4;


#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

#[spirv(compute(threads(64)))]
pub fn two_matrices(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] arg0: &mut [Mat4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] arg1: &[Mat4],
) {
    // dummy assigns so variables do not get optimized out
    arg0[0] = arg1[0];
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CustomStruct
{
    pub member0: f32,
    pub member1: f32,
}

#[spirv(compute(threads(64)))]
pub fn two_custom_structs(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] arg0: &mut [CustomStruct],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] arg1: &[CustomStruct],
) {
    // dummy assigns so variables do not get optimized out
    arg0[0].member0 = arg1[0].member0;
    arg0[0].member1 = arg1[0].member1;
}