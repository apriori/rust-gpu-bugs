# rust-gpu-bugs

Showing a few bugs in rust-gpu (2e4de94a6651baabe410252152eec05e948e9627).

### 1. Using a glam matrix types multiple times in a shader entry point signature as in the following example

```rs
use spirv_std::glam::Mat4;

#[spirv(compute(threads(64)))]
pub fn two_matrices(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] arg0: &mut [Mat4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] arg1: &[Mat4],
) {
    // dummy assigns so variables do not get optimized out
    arg0[0] = arg1[0];
}
```

results in the generated spirv decorating the exact same id multiple times with `OpMemberName`, visible here:

```
OpMemberName %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 0 "x_axis"
OpMemberName %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 1 "y_axis"
OpMemberName %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 2 "z_axis"
OpMemberName %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 3 "w_axis"
OpName %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ "spirv_std::glam::core::storage::Columns4<spirv_std::glam::XYZW<f32>>"
OpMemberName %spirv_std__glam__Mat4 0 "0"
OpName %spirv_std__glam__Mat4 "spirv_std::glam::Mat4"
OpName %arg0 "arg0"
OpName %arg1 "arg1"
OpName %CustomStruct "CustomStruct"
OpName %arg0_0 "arg0"
OpName %arg1_0 "arg1"
OpMemberName %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 0 "x_axis"
OpMemberName %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 1 "y_axis"
OpMemberName %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 2 "z_axis"
OpMemberName %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 3 "w_axis"
OpMemberName %spirv_std__glam__Mat4 0 "0"
OpMemberDecorate %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 0 Offset 0
OpMemberDecorate %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 1 Offset 16
OpMemberDecorate %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 2 Offset 32
OpMemberDecorate %spirv_std__glam__core__storage__Columns4_spirv_std__glam__XYZW_f32__ 3 Offset 48
OpMemberDecorate %spirv_std__glam__Mat4 0 Offset 0
OpDecorate %_runtimearr_spirv_std__glam__Mat4 ArrayStride 64
```

### 2. Custom structs within the shader are not annotated with `OpMemberName` at all:

```rs
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
```

The `result.spv-dis` contains only the mere name of the struct `CustomStruct` but no further names
of its members.

