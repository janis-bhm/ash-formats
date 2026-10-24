extern crate alloc;
use alloc::boxed::Box;
use ash::vk::*;
use core::ffi::c_void;
use core::ptr;
#[doc = r" Field operations for an extension feature structure in a Vulkan `pNext` chain."]
pub trait FeatureStruct: Copy {
    const STRUCTURE_TYPE: StructureType;
    fn contains_fields(&self, other: &Self) -> bool;
    fn union_fields(&mut self, other: &Self);
}
impl FeatureStruct for PhysicalDevice16BitStorageFeatures<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.storage_buffer16_bit_access & other.storage_buffer16_bit_access)
            == other.storage_buffer16_bit_access
            && (self.uniform_and_storage_buffer16_bit_access
                & other.uniform_and_storage_buffer16_bit_access)
                == other.uniform_and_storage_buffer16_bit_access
            && (self.storage_push_constant16 & other.storage_push_constant16)
                == other.storage_push_constant16
            && (self.storage_input_output16 & other.storage_input_output16)
                == other.storage_input_output16
    }
    fn union_fields(&mut self, other: &Self) {
        self.storage_buffer16_bit_access |= other.storage_buffer16_bit_access;
        self.uniform_and_storage_buffer16_bit_access |=
            other.uniform_and_storage_buffer16_bit_access;
        self.storage_push_constant16 |= other.storage_push_constant16;
        self.storage_input_output16 |= other.storage_input_output16;
    }
}
impl FeatureStruct for PhysicalDevice4444FormatsFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.format_a4r4g4b4 & other.format_a4r4g4b4) == other.format_a4r4g4b4
            && (self.format_a4b4g4r4 & other.format_a4b4g4r4) == other.format_a4b4g4r4
    }
    fn union_fields(&mut self, other: &Self) {
        self.format_a4r4g4b4 |= other.format_a4r4g4b4;
        self.format_a4b4g4r4 |= other.format_a4b4g4r4;
    }
}
impl FeatureStruct for PhysicalDevice8BitStorageFeatures<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.storage_buffer8_bit_access & other.storage_buffer8_bit_access)
            == other.storage_buffer8_bit_access
            && (self.uniform_and_storage_buffer8_bit_access
                & other.uniform_and_storage_buffer8_bit_access)
                == other.uniform_and_storage_buffer8_bit_access
            && (self.storage_push_constant8 & other.storage_push_constant8)
                == other.storage_push_constant8
    }
    fn union_fields(&mut self, other: &Self) {
        self.storage_buffer8_bit_access |= other.storage_buffer8_bit_access;
        self.uniform_and_storage_buffer8_bit_access |= other.uniform_and_storage_buffer8_bit_access;
        self.storage_push_constant8 |= other.storage_push_constant8;
    }
}
impl FeatureStruct for PhysicalDeviceASTCDecodeFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.decode_mode_shared_exponent & other.decode_mode_shared_exponent)
            == other.decode_mode_shared_exponent
    }
    fn union_fields(&mut self, other: &Self) {
        self.decode_mode_shared_exponent |= other.decode_mode_shared_exponent;
    }
}
impl FeatureStruct for PhysicalDeviceAccelerationStructureFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.acceleration_structure & other.acceleration_structure) == other.acceleration_structure
            && (self.acceleration_structure_capture_replay
                & other.acceleration_structure_capture_replay)
                == other.acceleration_structure_capture_replay
            && (self.acceleration_structure_indirect_build
                & other.acceleration_structure_indirect_build)
                == other.acceleration_structure_indirect_build
            && (self.acceleration_structure_host_commands
                & other.acceleration_structure_host_commands)
                == other.acceleration_structure_host_commands
            && (self.descriptor_binding_acceleration_structure_update_after_bind
                & other.descriptor_binding_acceleration_structure_update_after_bind)
                == other.descriptor_binding_acceleration_structure_update_after_bind
    }
    fn union_fields(&mut self, other: &Self) {
        self.acceleration_structure |= other.acceleration_structure;
        self.acceleration_structure_capture_replay |= other.acceleration_structure_capture_replay;
        self.acceleration_structure_indirect_build |= other.acceleration_structure_indirect_build;
        self.acceleration_structure_host_commands |= other.acceleration_structure_host_commands;
        self.descriptor_binding_acceleration_structure_update_after_bind |=
            other.descriptor_binding_acceleration_structure_update_after_bind;
    }
}
impl FeatureStruct for PhysicalDeviceAddressBindingReportFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.report_address_binding & other.report_address_binding) == other.report_address_binding
    }
    fn union_fields(&mut self, other: &Self) {
        self.report_address_binding |= other.report_address_binding;
    }
}
impl FeatureStruct for PhysicalDeviceAmigoProfilingFeaturesSEC<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.amigo_profiling & other.amigo_profiling) == other.amigo_profiling
    }
    fn union_fields(&mut self, other: &Self) {
        self.amigo_profiling |= other.amigo_profiling;
    }
}
impl FeatureStruct for PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.attachment_feedback_loop_layout & other.attachment_feedback_loop_layout)
            == other.attachment_feedback_loop_layout
    }
    fn union_fields(&mut self, other: &Self) {
        self.attachment_feedback_loop_layout |= other.attachment_feedback_loop_layout;
    }
}
impl FeatureStruct for PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.advanced_blend_coherent_operations & other.advanced_blend_coherent_operations)
            == other.advanced_blend_coherent_operations
    }
    fn union_fields(&mut self, other: &Self) {
        self.advanced_blend_coherent_operations |= other.advanced_blend_coherent_operations;
    }
}
impl FeatureStruct for PhysicalDeviceBorderColorSwizzleFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.border_color_swizzle & other.border_color_swizzle) == other.border_color_swizzle
            && (self.border_color_swizzle_from_image & other.border_color_swizzle_from_image)
                == other.border_color_swizzle_from_image
    }
    fn union_fields(&mut self, other: &Self) {
        self.border_color_swizzle |= other.border_color_swizzle;
        self.border_color_swizzle_from_image |= other.border_color_swizzle_from_image;
    }
}
impl FeatureStruct for PhysicalDeviceBufferDeviceAddressFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.buffer_device_address & other.buffer_device_address) == other.buffer_device_address
            && (self.buffer_device_address_capture_replay
                & other.buffer_device_address_capture_replay)
                == other.buffer_device_address_capture_replay
            && (self.buffer_device_address_multi_device & other.buffer_device_address_multi_device)
                == other.buffer_device_address_multi_device
    }
    fn union_fields(&mut self, other: &Self) {
        self.buffer_device_address |= other.buffer_device_address;
        self.buffer_device_address_capture_replay |= other.buffer_device_address_capture_replay;
        self.buffer_device_address_multi_device |= other.buffer_device_address_multi_device;
    }
}
impl FeatureStruct for PhysicalDeviceBufferDeviceAddressFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.buffer_device_address & other.buffer_device_address) == other.buffer_device_address
            && (self.buffer_device_address_capture_replay
                & other.buffer_device_address_capture_replay)
                == other.buffer_device_address_capture_replay
            && (self.buffer_device_address_multi_device & other.buffer_device_address_multi_device)
                == other.buffer_device_address_multi_device
    }
    fn union_fields(&mut self, other: &Self) {
        self.buffer_device_address |= other.buffer_device_address;
        self.buffer_device_address_capture_replay |= other.buffer_device_address_capture_replay;
        self.buffer_device_address_multi_device |= other.buffer_device_address_multi_device;
    }
}
impl FeatureStruct for PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.clusterculling_shader & other.clusterculling_shader) == other.clusterculling_shader
            && (self.multiview_cluster_culling_shader & other.multiview_cluster_culling_shader)
                == other.multiview_cluster_culling_shader
    }
    fn union_fields(&mut self, other: &Self) {
        self.clusterculling_shader |= other.clusterculling_shader;
        self.multiview_cluster_culling_shader |= other.multiview_cluster_culling_shader;
    }
}
impl FeatureStruct for PhysicalDeviceCoherentMemoryFeaturesAMD<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.device_coherent_memory & other.device_coherent_memory) == other.device_coherent_memory
    }
    fn union_fields(&mut self, other: &Self) {
        self.device_coherent_memory |= other.device_coherent_memory;
    }
}
impl FeatureStruct for PhysicalDeviceColorWriteEnableFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.color_write_enable & other.color_write_enable) == other.color_write_enable
    }
    fn union_fields(&mut self, other: &Self) {
        self.color_write_enable |= other.color_write_enable;
    }
}
impl FeatureStruct for PhysicalDeviceConditionalRenderingFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.conditional_rendering & other.conditional_rendering) == other.conditional_rendering
            && (self.inherited_conditional_rendering & other.inherited_conditional_rendering)
                == other.inherited_conditional_rendering
    }
    fn union_fields(&mut self, other: &Self) {
        self.conditional_rendering |= other.conditional_rendering;
        self.inherited_conditional_rendering |= other.inherited_conditional_rendering;
    }
}
impl FeatureStruct for PhysicalDeviceCooperativeMatrixFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.cooperative_matrix & other.cooperative_matrix) == other.cooperative_matrix
            && (self.cooperative_matrix_robust_buffer_access
                & other.cooperative_matrix_robust_buffer_access)
                == other.cooperative_matrix_robust_buffer_access
    }
    fn union_fields(&mut self, other: &Self) {
        self.cooperative_matrix |= other.cooperative_matrix;
        self.cooperative_matrix_robust_buffer_access |=
            other.cooperative_matrix_robust_buffer_access;
    }
}
impl FeatureStruct for PhysicalDeviceCooperativeMatrixFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.cooperative_matrix & other.cooperative_matrix) == other.cooperative_matrix
            && (self.cooperative_matrix_robust_buffer_access
                & other.cooperative_matrix_robust_buffer_access)
                == other.cooperative_matrix_robust_buffer_access
    }
    fn union_fields(&mut self, other: &Self) {
        self.cooperative_matrix |= other.cooperative_matrix;
        self.cooperative_matrix_robust_buffer_access |=
            other.cooperative_matrix_robust_buffer_access;
    }
}
impl FeatureStruct for PhysicalDeviceCopyMemoryIndirectFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.indirect_copy & other.indirect_copy) == other.indirect_copy
    }
    fn union_fields(&mut self, other: &Self) {
        self.indirect_copy |= other.indirect_copy;
    }
}
impl FeatureStruct for PhysicalDeviceCornerSampledImageFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.corner_sampled_image & other.corner_sampled_image) == other.corner_sampled_image
    }
    fn union_fields(&mut self, other: &Self) {
        self.corner_sampled_image |= other.corner_sampled_image;
    }
}
impl FeatureStruct for PhysicalDeviceCoverageReductionModeFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.coverage_reduction_mode & other.coverage_reduction_mode)
            == other.coverage_reduction_mode
    }
    fn union_fields(&mut self, other: &Self) {
        self.coverage_reduction_mode |= other.coverage_reduction_mode;
    }
}
impl FeatureStruct for PhysicalDeviceCubicClampFeaturesQCOM<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.cubic_range_clamp & other.cubic_range_clamp) == other.cubic_range_clamp
    }
    fn union_fields(&mut self, other: &Self) {
        self.cubic_range_clamp |= other.cubic_range_clamp;
    }
}
impl FeatureStruct for PhysicalDeviceCubicWeightsFeaturesQCOM<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.selectable_cubic_weights & other.selectable_cubic_weights)
            == other.selectable_cubic_weights
    }
    fn union_fields(&mut self, other: &Self) {
        self.selectable_cubic_weights |= other.selectable_cubic_weights;
    }
}
impl FeatureStruct for PhysicalDeviceCudaKernelLaunchFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.cuda_kernel_launch_features & other.cuda_kernel_launch_features)
            == other.cuda_kernel_launch_features
    }
    fn union_fields(&mut self, other: &Self) {
        self.cuda_kernel_launch_features |= other.cuda_kernel_launch_features;
    }
}
impl FeatureStruct for PhysicalDeviceCustomBorderColorFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.custom_border_colors & other.custom_border_colors) == other.custom_border_colors
            && (self.custom_border_color_without_format & other.custom_border_color_without_format)
                == other.custom_border_color_without_format
    }
    fn union_fields(&mut self, other: &Self) {
        self.custom_border_colors |= other.custom_border_colors;
        self.custom_border_color_without_format |= other.custom_border_color_without_format;
    }
}
impl FeatureStruct for PhysicalDeviceDepthBiasControlFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.depth_bias_control & other.depth_bias_control) == other.depth_bias_control
            && (self.least_representable_value_force_unorm_representation
                & other.least_representable_value_force_unorm_representation)
                == other.least_representable_value_force_unorm_representation
            && (self.float_representation & other.float_representation)
                == other.float_representation
            && (self.depth_bias_exact & other.depth_bias_exact) == other.depth_bias_exact
    }
    fn union_fields(&mut self, other: &Self) {
        self.depth_bias_control |= other.depth_bias_control;
        self.least_representable_value_force_unorm_representation |=
            other.least_representable_value_force_unorm_representation;
        self.float_representation |= other.float_representation;
        self.depth_bias_exact |= other.depth_bias_exact;
    }
}
impl FeatureStruct for PhysicalDeviceDepthClampZeroOneFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.depth_clamp_zero_one & other.depth_clamp_zero_one) == other.depth_clamp_zero_one
    }
    fn union_fields(&mut self, other: &Self) {
        self.depth_clamp_zero_one |= other.depth_clamp_zero_one;
    }
}
impl FeatureStruct for PhysicalDeviceDepthClipControlFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.depth_clip_control & other.depth_clip_control) == other.depth_clip_control
    }
    fn union_fields(&mut self, other: &Self) {
        self.depth_clip_control |= other.depth_clip_control;
    }
}
impl FeatureStruct for PhysicalDeviceDepthClipEnableFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.depth_clip_enable & other.depth_clip_enable) == other.depth_clip_enable
    }
    fn union_fields(&mut self, other: &Self) {
        self.depth_clip_enable |= other.depth_clip_enable;
    }
}
impl FeatureStruct for PhysicalDeviceDescriptorBufferFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.descriptor_buffer & other.descriptor_buffer) == other.descriptor_buffer
            && (self.descriptor_buffer_capture_replay & other.descriptor_buffer_capture_replay)
                == other.descriptor_buffer_capture_replay
            && (self.descriptor_buffer_image_layout_ignored
                & other.descriptor_buffer_image_layout_ignored)
                == other.descriptor_buffer_image_layout_ignored
            && (self.descriptor_buffer_push_descriptors & other.descriptor_buffer_push_descriptors)
                == other.descriptor_buffer_push_descriptors
    }
    fn union_fields(&mut self, other: &Self) {
        self.descriptor_buffer |= other.descriptor_buffer;
        self.descriptor_buffer_capture_replay |= other.descriptor_buffer_capture_replay;
        self.descriptor_buffer_image_layout_ignored |= other.descriptor_buffer_image_layout_ignored;
        self.descriptor_buffer_push_descriptors |= other.descriptor_buffer_push_descriptors;
    }
}
impl FeatureStruct for PhysicalDeviceDescriptorIndexingFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_input_attachment_array_dynamic_indexing
            & other.shader_input_attachment_array_dynamic_indexing)
            == other.shader_input_attachment_array_dynamic_indexing
            && (self.shader_uniform_texel_buffer_array_dynamic_indexing
                & other.shader_uniform_texel_buffer_array_dynamic_indexing)
                == other.shader_uniform_texel_buffer_array_dynamic_indexing
            && (self.shader_storage_texel_buffer_array_dynamic_indexing
                & other.shader_storage_texel_buffer_array_dynamic_indexing)
                == other.shader_storage_texel_buffer_array_dynamic_indexing
            && (self.shader_uniform_buffer_array_non_uniform_indexing
                & other.shader_uniform_buffer_array_non_uniform_indexing)
                == other.shader_uniform_buffer_array_non_uniform_indexing
            && (self.shader_sampled_image_array_non_uniform_indexing
                & other.shader_sampled_image_array_non_uniform_indexing)
                == other.shader_sampled_image_array_non_uniform_indexing
            && (self.shader_storage_buffer_array_non_uniform_indexing
                & other.shader_storage_buffer_array_non_uniform_indexing)
                == other.shader_storage_buffer_array_non_uniform_indexing
            && (self.shader_storage_image_array_non_uniform_indexing
                & other.shader_storage_image_array_non_uniform_indexing)
                == other.shader_storage_image_array_non_uniform_indexing
            && (self.shader_input_attachment_array_non_uniform_indexing
                & other.shader_input_attachment_array_non_uniform_indexing)
                == other.shader_input_attachment_array_non_uniform_indexing
            && (self.shader_uniform_texel_buffer_array_non_uniform_indexing
                & other.shader_uniform_texel_buffer_array_non_uniform_indexing)
                == other.shader_uniform_texel_buffer_array_non_uniform_indexing
            && (self.shader_storage_texel_buffer_array_non_uniform_indexing
                & other.shader_storage_texel_buffer_array_non_uniform_indexing)
                == other.shader_storage_texel_buffer_array_non_uniform_indexing
            && (self.descriptor_binding_uniform_buffer_update_after_bind
                & other.descriptor_binding_uniform_buffer_update_after_bind)
                == other.descriptor_binding_uniform_buffer_update_after_bind
            && (self.descriptor_binding_sampled_image_update_after_bind
                & other.descriptor_binding_sampled_image_update_after_bind)
                == other.descriptor_binding_sampled_image_update_after_bind
            && (self.descriptor_binding_storage_image_update_after_bind
                & other.descriptor_binding_storage_image_update_after_bind)
                == other.descriptor_binding_storage_image_update_after_bind
            && (self.descriptor_binding_storage_buffer_update_after_bind
                & other.descriptor_binding_storage_buffer_update_after_bind)
                == other.descriptor_binding_storage_buffer_update_after_bind
            && (self.descriptor_binding_uniform_texel_buffer_update_after_bind
                & other.descriptor_binding_uniform_texel_buffer_update_after_bind)
                == other.descriptor_binding_uniform_texel_buffer_update_after_bind
            && (self.descriptor_binding_storage_texel_buffer_update_after_bind
                & other.descriptor_binding_storage_texel_buffer_update_after_bind)
                == other.descriptor_binding_storage_texel_buffer_update_after_bind
            && (self.descriptor_binding_update_unused_while_pending
                & other.descriptor_binding_update_unused_while_pending)
                == other.descriptor_binding_update_unused_while_pending
            && (self.descriptor_binding_partially_bound & other.descriptor_binding_partially_bound)
                == other.descriptor_binding_partially_bound
            && (self.descriptor_binding_variable_descriptor_count
                & other.descriptor_binding_variable_descriptor_count)
                == other.descriptor_binding_variable_descriptor_count
            && (self.runtime_descriptor_array & other.runtime_descriptor_array)
                == other.runtime_descriptor_array
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_input_attachment_array_dynamic_indexing |=
            other.shader_input_attachment_array_dynamic_indexing;
        self.shader_uniform_texel_buffer_array_dynamic_indexing |=
            other.shader_uniform_texel_buffer_array_dynamic_indexing;
        self.shader_storage_texel_buffer_array_dynamic_indexing |=
            other.shader_storage_texel_buffer_array_dynamic_indexing;
        self.shader_uniform_buffer_array_non_uniform_indexing |=
            other.shader_uniform_buffer_array_non_uniform_indexing;
        self.shader_sampled_image_array_non_uniform_indexing |=
            other.shader_sampled_image_array_non_uniform_indexing;
        self.shader_storage_buffer_array_non_uniform_indexing |=
            other.shader_storage_buffer_array_non_uniform_indexing;
        self.shader_storage_image_array_non_uniform_indexing |=
            other.shader_storage_image_array_non_uniform_indexing;
        self.shader_input_attachment_array_non_uniform_indexing |=
            other.shader_input_attachment_array_non_uniform_indexing;
        self.shader_uniform_texel_buffer_array_non_uniform_indexing |=
            other.shader_uniform_texel_buffer_array_non_uniform_indexing;
        self.shader_storage_texel_buffer_array_non_uniform_indexing |=
            other.shader_storage_texel_buffer_array_non_uniform_indexing;
        self.descriptor_binding_uniform_buffer_update_after_bind |=
            other.descriptor_binding_uniform_buffer_update_after_bind;
        self.descriptor_binding_sampled_image_update_after_bind |=
            other.descriptor_binding_sampled_image_update_after_bind;
        self.descriptor_binding_storage_image_update_after_bind |=
            other.descriptor_binding_storage_image_update_after_bind;
        self.descriptor_binding_storage_buffer_update_after_bind |=
            other.descriptor_binding_storage_buffer_update_after_bind;
        self.descriptor_binding_uniform_texel_buffer_update_after_bind |=
            other.descriptor_binding_uniform_texel_buffer_update_after_bind;
        self.descriptor_binding_storage_texel_buffer_update_after_bind |=
            other.descriptor_binding_storage_texel_buffer_update_after_bind;
        self.descriptor_binding_update_unused_while_pending |=
            other.descriptor_binding_update_unused_while_pending;
        self.descriptor_binding_partially_bound |= other.descriptor_binding_partially_bound;
        self.descriptor_binding_variable_descriptor_count |=
            other.descriptor_binding_variable_descriptor_count;
        self.runtime_descriptor_array |= other.runtime_descriptor_array;
    }
}
impl FeatureStruct for PhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.descriptor_pool_overallocation & other.descriptor_pool_overallocation)
            == other.descriptor_pool_overallocation
    }
    fn union_fields(&mut self, other: &Self) {
        self.descriptor_pool_overallocation |= other.descriptor_pool_overallocation;
    }
}
impl FeatureStruct for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.descriptor_set_host_mapping & other.descriptor_set_host_mapping)
            == other.descriptor_set_host_mapping
    }
    fn union_fields(&mut self, other: &Self) {
        self.descriptor_set_host_mapping |= other.descriptor_set_host_mapping;
    }
}
impl FeatureStruct for PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.device_generated_compute & other.device_generated_compute)
            == other.device_generated_compute
            && (self.device_generated_compute_pipelines & other.device_generated_compute_pipelines)
                == other.device_generated_compute_pipelines
            && (self.device_generated_compute_capture_replay
                & other.device_generated_compute_capture_replay)
                == other.device_generated_compute_capture_replay
    }
    fn union_fields(&mut self, other: &Self) {
        self.device_generated_compute |= other.device_generated_compute;
        self.device_generated_compute_pipelines |= other.device_generated_compute_pipelines;
        self.device_generated_compute_capture_replay |=
            other.device_generated_compute_capture_replay;
    }
}
impl FeatureStruct for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.device_generated_commands & other.device_generated_commands)
            == other.device_generated_commands
    }
    fn union_fields(&mut self, other: &Self) {
        self.device_generated_commands |= other.device_generated_commands;
    }
}
impl FeatureStruct for PhysicalDeviceDeviceMemoryReportFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.device_memory_report & other.device_memory_report) == other.device_memory_report
    }
    fn union_fields(&mut self, other: &Self) {
        self.device_memory_report |= other.device_memory_report;
    }
}
impl FeatureStruct for PhysicalDeviceDiagnosticsConfigFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.diagnostics_config & other.diagnostics_config) == other.diagnostics_config
    }
    fn union_fields(&mut self, other: &Self) {
        self.diagnostics_config |= other.diagnostics_config;
    }
}
impl FeatureStruct for PhysicalDeviceDisplacementMicromapFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.displacement_micromap & other.displacement_micromap) == other.displacement_micromap
    }
    fn union_fields(&mut self, other: &Self) {
        self.displacement_micromap |= other.displacement_micromap;
    }
}
impl FeatureStruct for PhysicalDeviceDynamicRenderingFeatures<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.dynamic_rendering & other.dynamic_rendering) == other.dynamic_rendering
    }
    fn union_fields(&mut self, other: &Self) {
        self.dynamic_rendering |= other.dynamic_rendering;
    }
}
impl FeatureStruct for PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.dynamic_rendering_local_read & other.dynamic_rendering_local_read)
            == other.dynamic_rendering_local_read
    }
    fn union_fields(&mut self, other: &Self) {
        self.dynamic_rendering_local_read |= other.dynamic_rendering_local_read;
    }
}
impl FeatureStruct for PhysicalDeviceExclusiveScissorFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.exclusive_scissor & other.exclusive_scissor) == other.exclusive_scissor
    }
    fn union_fields(&mut self, other: &Self) {
        self.exclusive_scissor |= other.exclusive_scissor;
    }
}
impl FeatureStruct for PhysicalDeviceExtendedDynamicState2FeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.extended_dynamic_state2 & other.extended_dynamic_state2)
            == other.extended_dynamic_state2
            && (self.extended_dynamic_state2_logic_op & other.extended_dynamic_state2_logic_op)
                == other.extended_dynamic_state2_logic_op
            && (self.extended_dynamic_state2_patch_control_points
                & other.extended_dynamic_state2_patch_control_points)
                == other.extended_dynamic_state2_patch_control_points
    }
    fn union_fields(&mut self, other: &Self) {
        self.extended_dynamic_state2 |= other.extended_dynamic_state2;
        self.extended_dynamic_state2_logic_op |= other.extended_dynamic_state2_logic_op;
        self.extended_dynamic_state2_patch_control_points |=
            other.extended_dynamic_state2_patch_control_points;
    }
}
impl FeatureStruct for PhysicalDeviceExtendedDynamicState3FeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.extended_dynamic_state3_tessellation_domain_origin
            & other.extended_dynamic_state3_tessellation_domain_origin)
            == other.extended_dynamic_state3_tessellation_domain_origin
            && (self.extended_dynamic_state3_depth_clamp_enable
                & other.extended_dynamic_state3_depth_clamp_enable)
                == other.extended_dynamic_state3_depth_clamp_enable
            && (self.extended_dynamic_state3_polygon_mode
                & other.extended_dynamic_state3_polygon_mode)
                == other.extended_dynamic_state3_polygon_mode
            && (self.extended_dynamic_state3_rasterization_samples
                & other.extended_dynamic_state3_rasterization_samples)
                == other.extended_dynamic_state3_rasterization_samples
            && (self.extended_dynamic_state3_sample_mask
                & other.extended_dynamic_state3_sample_mask)
                == other.extended_dynamic_state3_sample_mask
            && (self.extended_dynamic_state3_alpha_to_coverage_enable
                & other.extended_dynamic_state3_alpha_to_coverage_enable)
                == other.extended_dynamic_state3_alpha_to_coverage_enable
            && (self.extended_dynamic_state3_alpha_to_one_enable
                & other.extended_dynamic_state3_alpha_to_one_enable)
                == other.extended_dynamic_state3_alpha_to_one_enable
            && (self.extended_dynamic_state3_logic_op_enable
                & other.extended_dynamic_state3_logic_op_enable)
                == other.extended_dynamic_state3_logic_op_enable
            && (self.extended_dynamic_state3_color_blend_enable
                & other.extended_dynamic_state3_color_blend_enable)
                == other.extended_dynamic_state3_color_blend_enable
            && (self.extended_dynamic_state3_color_blend_equation
                & other.extended_dynamic_state3_color_blend_equation)
                == other.extended_dynamic_state3_color_blend_equation
            && (self.extended_dynamic_state3_color_write_mask
                & other.extended_dynamic_state3_color_write_mask)
                == other.extended_dynamic_state3_color_write_mask
            && (self.extended_dynamic_state3_rasterization_stream
                & other.extended_dynamic_state3_rasterization_stream)
                == other.extended_dynamic_state3_rasterization_stream
            && (self.extended_dynamic_state3_conservative_rasterization_mode
                & other.extended_dynamic_state3_conservative_rasterization_mode)
                == other.extended_dynamic_state3_conservative_rasterization_mode
            && (self.extended_dynamic_state3_extra_primitive_overestimation_size
                & other.extended_dynamic_state3_extra_primitive_overestimation_size)
                == other.extended_dynamic_state3_extra_primitive_overestimation_size
            && (self.extended_dynamic_state3_depth_clip_enable
                & other.extended_dynamic_state3_depth_clip_enable)
                == other.extended_dynamic_state3_depth_clip_enable
            && (self.extended_dynamic_state3_sample_locations_enable
                & other.extended_dynamic_state3_sample_locations_enable)
                == other.extended_dynamic_state3_sample_locations_enable
            && (self.extended_dynamic_state3_color_blend_advanced
                & other.extended_dynamic_state3_color_blend_advanced)
                == other.extended_dynamic_state3_color_blend_advanced
            && (self.extended_dynamic_state3_provoking_vertex_mode
                & other.extended_dynamic_state3_provoking_vertex_mode)
                == other.extended_dynamic_state3_provoking_vertex_mode
            && (self.extended_dynamic_state3_line_rasterization_mode
                & other.extended_dynamic_state3_line_rasterization_mode)
                == other.extended_dynamic_state3_line_rasterization_mode
            && (self.extended_dynamic_state3_line_stipple_enable
                & other.extended_dynamic_state3_line_stipple_enable)
                == other.extended_dynamic_state3_line_stipple_enable
            && (self.extended_dynamic_state3_depth_clip_negative_one_to_one
                & other.extended_dynamic_state3_depth_clip_negative_one_to_one)
                == other.extended_dynamic_state3_depth_clip_negative_one_to_one
            && (self.extended_dynamic_state3_viewport_w_scaling_enable
                & other.extended_dynamic_state3_viewport_w_scaling_enable)
                == other.extended_dynamic_state3_viewport_w_scaling_enable
            && (self.extended_dynamic_state3_viewport_swizzle
                & other.extended_dynamic_state3_viewport_swizzle)
                == other.extended_dynamic_state3_viewport_swizzle
            && (self.extended_dynamic_state3_coverage_to_color_enable
                & other.extended_dynamic_state3_coverage_to_color_enable)
                == other.extended_dynamic_state3_coverage_to_color_enable
            && (self.extended_dynamic_state3_coverage_to_color_location
                & other.extended_dynamic_state3_coverage_to_color_location)
                == other.extended_dynamic_state3_coverage_to_color_location
            && (self.extended_dynamic_state3_coverage_modulation_mode
                & other.extended_dynamic_state3_coverage_modulation_mode)
                == other.extended_dynamic_state3_coverage_modulation_mode
            && (self.extended_dynamic_state3_coverage_modulation_table_enable
                & other.extended_dynamic_state3_coverage_modulation_table_enable)
                == other.extended_dynamic_state3_coverage_modulation_table_enable
            && (self.extended_dynamic_state3_coverage_modulation_table
                & other.extended_dynamic_state3_coverage_modulation_table)
                == other.extended_dynamic_state3_coverage_modulation_table
            && (self.extended_dynamic_state3_coverage_reduction_mode
                & other.extended_dynamic_state3_coverage_reduction_mode)
                == other.extended_dynamic_state3_coverage_reduction_mode
            && (self.extended_dynamic_state3_representative_fragment_test_enable
                & other.extended_dynamic_state3_representative_fragment_test_enable)
                == other.extended_dynamic_state3_representative_fragment_test_enable
            && (self.extended_dynamic_state3_shading_rate_image_enable
                & other.extended_dynamic_state3_shading_rate_image_enable)
                == other.extended_dynamic_state3_shading_rate_image_enable
    }
    fn union_fields(&mut self, other: &Self) {
        self.extended_dynamic_state3_tessellation_domain_origin |=
            other.extended_dynamic_state3_tessellation_domain_origin;
        self.extended_dynamic_state3_depth_clamp_enable |=
            other.extended_dynamic_state3_depth_clamp_enable;
        self.extended_dynamic_state3_polygon_mode |= other.extended_dynamic_state3_polygon_mode;
        self.extended_dynamic_state3_rasterization_samples |=
            other.extended_dynamic_state3_rasterization_samples;
        self.extended_dynamic_state3_sample_mask |= other.extended_dynamic_state3_sample_mask;
        self.extended_dynamic_state3_alpha_to_coverage_enable |=
            other.extended_dynamic_state3_alpha_to_coverage_enable;
        self.extended_dynamic_state3_alpha_to_one_enable |=
            other.extended_dynamic_state3_alpha_to_one_enable;
        self.extended_dynamic_state3_logic_op_enable |=
            other.extended_dynamic_state3_logic_op_enable;
        self.extended_dynamic_state3_color_blend_enable |=
            other.extended_dynamic_state3_color_blend_enable;
        self.extended_dynamic_state3_color_blend_equation |=
            other.extended_dynamic_state3_color_blend_equation;
        self.extended_dynamic_state3_color_write_mask |=
            other.extended_dynamic_state3_color_write_mask;
        self.extended_dynamic_state3_rasterization_stream |=
            other.extended_dynamic_state3_rasterization_stream;
        self.extended_dynamic_state3_conservative_rasterization_mode |=
            other.extended_dynamic_state3_conservative_rasterization_mode;
        self.extended_dynamic_state3_extra_primitive_overestimation_size |=
            other.extended_dynamic_state3_extra_primitive_overestimation_size;
        self.extended_dynamic_state3_depth_clip_enable |=
            other.extended_dynamic_state3_depth_clip_enable;
        self.extended_dynamic_state3_sample_locations_enable |=
            other.extended_dynamic_state3_sample_locations_enable;
        self.extended_dynamic_state3_color_blend_advanced |=
            other.extended_dynamic_state3_color_blend_advanced;
        self.extended_dynamic_state3_provoking_vertex_mode |=
            other.extended_dynamic_state3_provoking_vertex_mode;
        self.extended_dynamic_state3_line_rasterization_mode |=
            other.extended_dynamic_state3_line_rasterization_mode;
        self.extended_dynamic_state3_line_stipple_enable |=
            other.extended_dynamic_state3_line_stipple_enable;
        self.extended_dynamic_state3_depth_clip_negative_one_to_one |=
            other.extended_dynamic_state3_depth_clip_negative_one_to_one;
        self.extended_dynamic_state3_viewport_w_scaling_enable |=
            other.extended_dynamic_state3_viewport_w_scaling_enable;
        self.extended_dynamic_state3_viewport_swizzle |=
            other.extended_dynamic_state3_viewport_swizzle;
        self.extended_dynamic_state3_coverage_to_color_enable |=
            other.extended_dynamic_state3_coverage_to_color_enable;
        self.extended_dynamic_state3_coverage_to_color_location |=
            other.extended_dynamic_state3_coverage_to_color_location;
        self.extended_dynamic_state3_coverage_modulation_mode |=
            other.extended_dynamic_state3_coverage_modulation_mode;
        self.extended_dynamic_state3_coverage_modulation_table_enable |=
            other.extended_dynamic_state3_coverage_modulation_table_enable;
        self.extended_dynamic_state3_coverage_modulation_table |=
            other.extended_dynamic_state3_coverage_modulation_table;
        self.extended_dynamic_state3_coverage_reduction_mode |=
            other.extended_dynamic_state3_coverage_reduction_mode;
        self.extended_dynamic_state3_representative_fragment_test_enable |=
            other.extended_dynamic_state3_representative_fragment_test_enable;
        self.extended_dynamic_state3_shading_rate_image_enable |=
            other.extended_dynamic_state3_shading_rate_image_enable;
    }
}
impl FeatureStruct for PhysicalDeviceExtendedDynamicStateFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.extended_dynamic_state & other.extended_dynamic_state) == other.extended_dynamic_state
    }
    fn union_fields(&mut self, other: &Self) {
        self.extended_dynamic_state |= other.extended_dynamic_state;
    }
}
impl FeatureStruct for PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.extended_sparse_address_space & other.extended_sparse_address_space)
            == other.extended_sparse_address_space
    }
    fn union_fields(&mut self, other: &Self) {
        self.extended_sparse_address_space |= other.extended_sparse_address_space;
    }
}
impl FeatureStruct for PhysicalDeviceExternalFormatResolveFeaturesANDROID<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.external_format_resolve & other.external_format_resolve)
            == other.external_format_resolve
    }
    fn union_fields(&mut self, other: &Self) {
        self.external_format_resolve |= other.external_format_resolve;
    }
}
impl FeatureStruct for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.external_memory_rdma & other.external_memory_rdma) == other.external_memory_rdma
    }
    fn union_fields(&mut self, other: &Self) {
        self.external_memory_rdma |= other.external_memory_rdma;
    }
}
impl FeatureStruct for PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.screen_buffer_import & other.screen_buffer_import) == other.screen_buffer_import
    }
    fn union_fields(&mut self, other: &Self) {
        self.screen_buffer_import |= other.screen_buffer_import;
    }
}
impl FeatureStruct for PhysicalDeviceFaultFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_FAULT_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.device_fault & other.device_fault) == other.device_fault
            && (self.device_fault_vendor_binary & other.device_fault_vendor_binary)
                == other.device_fault_vendor_binary
    }
    fn union_fields(&mut self, other: &Self) {
        self.device_fault |= other.device_fault;
        self.device_fault_vendor_binary |= other.device_fault_vendor_binary;
    }
}
impl FeatureStruct for PhysicalDeviceFragmentDensityMap2FeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.fragment_density_map_deferred & other.fragment_density_map_deferred)
            == other.fragment_density_map_deferred
    }
    fn union_fields(&mut self, other: &Self) {
        self.fragment_density_map_deferred |= other.fragment_density_map_deferred;
    }
}
impl FeatureStruct for PhysicalDeviceFragmentDensityMapFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.fragment_density_map & other.fragment_density_map) == other.fragment_density_map
            && (self.fragment_density_map_dynamic & other.fragment_density_map_dynamic)
                == other.fragment_density_map_dynamic
            && (self.fragment_density_map_non_subsampled_images
                & other.fragment_density_map_non_subsampled_images)
                == other.fragment_density_map_non_subsampled_images
    }
    fn union_fields(&mut self, other: &Self) {
        self.fragment_density_map |= other.fragment_density_map;
        self.fragment_density_map_dynamic |= other.fragment_density_map_dynamic;
        self.fragment_density_map_non_subsampled_images |=
            other.fragment_density_map_non_subsampled_images;
    }
}
impl FeatureStruct for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.fragment_density_map_offset & other.fragment_density_map_offset)
            == other.fragment_density_map_offset
    }
    fn union_fields(&mut self, other: &Self) {
        self.fragment_density_map_offset |= other.fragment_density_map_offset;
    }
}
impl FeatureStruct for PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.fragment_shader_barycentric & other.fragment_shader_barycentric)
            == other.fragment_shader_barycentric
    }
    fn union_fields(&mut self, other: &Self) {
        self.fragment_shader_barycentric |= other.fragment_shader_barycentric;
    }
}
impl FeatureStruct for PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.fragment_shader_sample_interlock & other.fragment_shader_sample_interlock)
            == other.fragment_shader_sample_interlock
            && (self.fragment_shader_pixel_interlock & other.fragment_shader_pixel_interlock)
                == other.fragment_shader_pixel_interlock
            && (self.fragment_shader_shading_rate_interlock
                & other.fragment_shader_shading_rate_interlock)
                == other.fragment_shader_shading_rate_interlock
    }
    fn union_fields(&mut self, other: &Self) {
        self.fragment_shader_sample_interlock |= other.fragment_shader_sample_interlock;
        self.fragment_shader_pixel_interlock |= other.fragment_shader_pixel_interlock;
        self.fragment_shader_shading_rate_interlock |= other.fragment_shader_shading_rate_interlock;
    }
}
impl FeatureStruct for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.fragment_shading_rate_enums & other.fragment_shading_rate_enums)
            == other.fragment_shading_rate_enums
            && (self.supersample_fragment_shading_rates & other.supersample_fragment_shading_rates)
                == other.supersample_fragment_shading_rates
            && (self.no_invocation_fragment_shading_rates
                & other.no_invocation_fragment_shading_rates)
                == other.no_invocation_fragment_shading_rates
    }
    fn union_fields(&mut self, other: &Self) {
        self.fragment_shading_rate_enums |= other.fragment_shading_rate_enums;
        self.supersample_fragment_shading_rates |= other.supersample_fragment_shading_rates;
        self.no_invocation_fragment_shading_rates |= other.no_invocation_fragment_shading_rates;
    }
}
impl FeatureStruct for PhysicalDeviceFragmentShadingRateFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.pipeline_fragment_shading_rate & other.pipeline_fragment_shading_rate)
            == other.pipeline_fragment_shading_rate
            && (self.primitive_fragment_shading_rate & other.primitive_fragment_shading_rate)
                == other.primitive_fragment_shading_rate
            && (self.attachment_fragment_shading_rate & other.attachment_fragment_shading_rate)
                == other.attachment_fragment_shading_rate
    }
    fn union_fields(&mut self, other: &Self) {
        self.pipeline_fragment_shading_rate |= other.pipeline_fragment_shading_rate;
        self.primitive_fragment_shading_rate |= other.primitive_fragment_shading_rate;
        self.attachment_fragment_shading_rate |= other.attachment_fragment_shading_rate;
    }
}
impl FeatureStruct for PhysicalDeviceFrameBoundaryFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.frame_boundary & other.frame_boundary) == other.frame_boundary
    }
    fn union_fields(&mut self, other: &Self) {
        self.frame_boundary |= other.frame_boundary;
    }
}
impl FeatureStruct for PhysicalDeviceGlobalPriorityQueryFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.global_priority_query & other.global_priority_query) == other.global_priority_query
    }
    fn union_fields(&mut self, other: &Self) {
        self.global_priority_query |= other.global_priority_query;
    }
}
impl FeatureStruct for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.graphics_pipeline_library & other.graphics_pipeline_library)
            == other.graphics_pipeline_library
    }
    fn union_fields(&mut self, other: &Self) {
        self.graphics_pipeline_library |= other.graphics_pipeline_library;
    }
}
impl FeatureStruct for PhysicalDeviceHostImageCopyFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.host_image_copy & other.host_image_copy) == other.host_image_copy
    }
    fn union_fields(&mut self, other: &Self) {
        self.host_image_copy |= other.host_image_copy;
    }
}
impl FeatureStruct for PhysicalDeviceHostQueryResetFeatures<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.host_query_reset & other.host_query_reset) == other.host_query_reset
    }
    fn union_fields(&mut self, other: &Self) {
        self.host_query_reset |= other.host_query_reset;
    }
}
impl FeatureStruct for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.image2_d_view_of3_d & other.image2_d_view_of3_d) == other.image2_d_view_of3_d
            && (self.sampler2_d_view_of3_d & other.sampler2_d_view_of3_d)
                == other.sampler2_d_view_of3_d
    }
    fn union_fields(&mut self, other: &Self) {
        self.image2_d_view_of3_d |= other.image2_d_view_of3_d;
        self.sampler2_d_view_of3_d |= other.sampler2_d_view_of3_d;
    }
}
impl FeatureStruct for PhysicalDeviceImageCompressionControlFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.image_compression_control & other.image_compression_control)
            == other.image_compression_control
    }
    fn union_fields(&mut self, other: &Self) {
        self.image_compression_control |= other.image_compression_control;
    }
}
impl FeatureStruct for PhysicalDeviceImageProcessing2FeaturesQCOM<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.texture_block_match2 & other.texture_block_match2) == other.texture_block_match2
    }
    fn union_fields(&mut self, other: &Self) {
        self.texture_block_match2 |= other.texture_block_match2;
    }
}
impl FeatureStruct for PhysicalDeviceImageProcessingFeaturesQCOM<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.texture_sample_weighted & other.texture_sample_weighted)
            == other.texture_sample_weighted
            && (self.texture_box_filter & other.texture_box_filter) == other.texture_box_filter
            && (self.texture_block_match & other.texture_block_match) == other.texture_block_match
    }
    fn union_fields(&mut self, other: &Self) {
        self.texture_sample_weighted |= other.texture_sample_weighted;
        self.texture_box_filter |= other.texture_box_filter;
        self.texture_block_match |= other.texture_block_match;
    }
}
impl FeatureStruct for PhysicalDeviceImageRobustnessFeatures<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.robust_image_access & other.robust_image_access) == other.robust_image_access
    }
    fn union_fields(&mut self, other: &Self) {
        self.robust_image_access |= other.robust_image_access;
    }
}
impl FeatureStruct for PhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.image_sliced_view_of3_d & other.image_sliced_view_of3_d)
            == other.image_sliced_view_of3_d
    }
    fn union_fields(&mut self, other: &Self) {
        self.image_sliced_view_of3_d |= other.image_sliced_view_of3_d;
    }
}
impl FeatureStruct for PhysicalDeviceImageViewMinLodFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.min_lod & other.min_lod) == other.min_lod
    }
    fn union_fields(&mut self, other: &Self) {
        self.min_lod |= other.min_lod;
    }
}
impl FeatureStruct for PhysicalDeviceImagelessFramebufferFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.imageless_framebuffer & other.imageless_framebuffer) == other.imageless_framebuffer
    }
    fn union_fields(&mut self, other: &Self) {
        self.imageless_framebuffer |= other.imageless_framebuffer;
    }
}
impl FeatureStruct for PhysicalDeviceIndexTypeUint8FeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.index_type_uint8 & other.index_type_uint8) == other.index_type_uint8
    }
    fn union_fields(&mut self, other: &Self) {
        self.index_type_uint8 |= other.index_type_uint8;
    }
}
impl FeatureStruct for PhysicalDeviceInheritedViewportScissorFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.inherited_viewport_scissor2_d & other.inherited_viewport_scissor2_d)
            == other.inherited_viewport_scissor2_d
    }
    fn union_fields(&mut self, other: &Self) {
        self.inherited_viewport_scissor2_d |= other.inherited_viewport_scissor2_d;
    }
}
impl FeatureStruct for PhysicalDeviceInlineUniformBlockFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.inline_uniform_block & other.inline_uniform_block) == other.inline_uniform_block
            && (self.descriptor_binding_inline_uniform_block_update_after_bind
                & other.descriptor_binding_inline_uniform_block_update_after_bind)
                == other.descriptor_binding_inline_uniform_block_update_after_bind
    }
    fn union_fields(&mut self, other: &Self) {
        self.inline_uniform_block |= other.inline_uniform_block;
        self.descriptor_binding_inline_uniform_block_update_after_bind |=
            other.descriptor_binding_inline_uniform_block_update_after_bind;
    }
}
impl FeatureStruct for PhysicalDeviceInvocationMaskFeaturesHUAWEI<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.invocation_mask & other.invocation_mask) == other.invocation_mask
    }
    fn union_fields(&mut self, other: &Self) {
        self.invocation_mask |= other.invocation_mask;
    }
}
impl FeatureStruct for PhysicalDeviceLegacyDitheringFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.legacy_dithering & other.legacy_dithering) == other.legacy_dithering
    }
    fn union_fields(&mut self, other: &Self) {
        self.legacy_dithering |= other.legacy_dithering;
    }
}
impl FeatureStruct for PhysicalDeviceLineRasterizationFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.rectangular_lines & other.rectangular_lines) == other.rectangular_lines
            && (self.bresenham_lines & other.bresenham_lines) == other.bresenham_lines
            && (self.smooth_lines & other.smooth_lines) == other.smooth_lines
            && (self.stippled_rectangular_lines & other.stippled_rectangular_lines)
                == other.stippled_rectangular_lines
            && (self.stippled_bresenham_lines & other.stippled_bresenham_lines)
                == other.stippled_bresenham_lines
            && (self.stippled_smooth_lines & other.stippled_smooth_lines)
                == other.stippled_smooth_lines
    }
    fn union_fields(&mut self, other: &Self) {
        self.rectangular_lines |= other.rectangular_lines;
        self.bresenham_lines |= other.bresenham_lines;
        self.smooth_lines |= other.smooth_lines;
        self.stippled_rectangular_lines |= other.stippled_rectangular_lines;
        self.stippled_bresenham_lines |= other.stippled_bresenham_lines;
        self.stippled_smooth_lines |= other.stippled_smooth_lines;
    }
}
impl FeatureStruct for PhysicalDeviceLinearColorAttachmentFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.linear_color_attachment & other.linear_color_attachment)
            == other.linear_color_attachment
    }
    fn union_fields(&mut self, other: &Self) {
        self.linear_color_attachment |= other.linear_color_attachment;
    }
}
impl FeatureStruct for PhysicalDeviceMaintenance4Features<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.maintenance4 & other.maintenance4) == other.maintenance4
    }
    fn union_fields(&mut self, other: &Self) {
        self.maintenance4 |= other.maintenance4;
    }
}
impl FeatureStruct for PhysicalDeviceMaintenance5FeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.maintenance5 & other.maintenance5) == other.maintenance5
    }
    fn union_fields(&mut self, other: &Self) {
        self.maintenance5 |= other.maintenance5;
    }
}
impl FeatureStruct for PhysicalDeviceMaintenance6FeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.maintenance6 & other.maintenance6) == other.maintenance6
    }
    fn union_fields(&mut self, other: &Self) {
        self.maintenance6 |= other.maintenance6;
    }
}
impl FeatureStruct for PhysicalDeviceMapMemoryPlacedFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.memory_map_placed & other.memory_map_placed) == other.memory_map_placed
            && (self.memory_map_range_placed & other.memory_map_range_placed)
                == other.memory_map_range_placed
            && (self.memory_unmap_reserve & other.memory_unmap_reserve)
                == other.memory_unmap_reserve
    }
    fn union_fields(&mut self, other: &Self) {
        self.memory_map_placed |= other.memory_map_placed;
        self.memory_map_range_placed |= other.memory_map_range_placed;
        self.memory_unmap_reserve |= other.memory_unmap_reserve;
    }
}
impl FeatureStruct for PhysicalDeviceMemoryDecompressionFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.memory_decompression & other.memory_decompression) == other.memory_decompression
    }
    fn union_fields(&mut self, other: &Self) {
        self.memory_decompression |= other.memory_decompression;
    }
}
impl FeatureStruct for PhysicalDeviceMemoryPriorityFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.memory_priority & other.memory_priority) == other.memory_priority
    }
    fn union_fields(&mut self, other: &Self) {
        self.memory_priority |= other.memory_priority;
    }
}
impl FeatureStruct for PhysicalDeviceMeshShaderFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.task_shader & other.task_shader) == other.task_shader
            && (self.mesh_shader & other.mesh_shader) == other.mesh_shader
            && (self.multiview_mesh_shader & other.multiview_mesh_shader)
                == other.multiview_mesh_shader
            && (self.primitive_fragment_shading_rate_mesh_shader
                & other.primitive_fragment_shading_rate_mesh_shader)
                == other.primitive_fragment_shading_rate_mesh_shader
            && (self.mesh_shader_queries & other.mesh_shader_queries) == other.mesh_shader_queries
    }
    fn union_fields(&mut self, other: &Self) {
        self.task_shader |= other.task_shader;
        self.mesh_shader |= other.mesh_shader;
        self.multiview_mesh_shader |= other.multiview_mesh_shader;
        self.primitive_fragment_shading_rate_mesh_shader |=
            other.primitive_fragment_shading_rate_mesh_shader;
        self.mesh_shader_queries |= other.mesh_shader_queries;
    }
}
impl FeatureStruct for PhysicalDeviceMeshShaderFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.task_shader & other.task_shader) == other.task_shader
            && (self.mesh_shader & other.mesh_shader) == other.mesh_shader
    }
    fn union_fields(&mut self, other: &Self) {
        self.task_shader |= other.task_shader;
        self.mesh_shader |= other.mesh_shader;
    }
}
impl FeatureStruct for PhysicalDeviceMultiDrawFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.multi_draw & other.multi_draw) == other.multi_draw
    }
    fn union_fields(&mut self, other: &Self) {
        self.multi_draw |= other.multi_draw;
    }
}
impl FeatureStruct for PhysicalDeviceMultiviewFeatures<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.multiview & other.multiview) == other.multiview
            && (self.multiview_geometry_shader & other.multiview_geometry_shader)
                == other.multiview_geometry_shader
            && (self.multiview_tessellation_shader & other.multiview_tessellation_shader)
                == other.multiview_tessellation_shader
    }
    fn union_fields(&mut self, other: &Self) {
        self.multiview |= other.multiview;
        self.multiview_geometry_shader |= other.multiview_geometry_shader;
        self.multiview_tessellation_shader |= other.multiview_tessellation_shader;
    }
}
impl FeatureStruct for PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.multiview_per_view_render_areas & other.multiview_per_view_render_areas)
            == other.multiview_per_view_render_areas
    }
    fn union_fields(&mut self, other: &Self) {
        self.multiview_per_view_render_areas |= other.multiview_per_view_render_areas;
    }
}
impl FeatureStruct for PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.multiview_per_view_viewports & other.multiview_per_view_viewports)
            == other.multiview_per_view_viewports
    }
    fn union_fields(&mut self, other: &Self) {
        self.multiview_per_view_viewports |= other.multiview_per_view_viewports;
    }
}
impl FeatureStruct for PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.mutable_descriptor_type & other.mutable_descriptor_type)
            == other.mutable_descriptor_type
    }
    fn union_fields(&mut self, other: &Self) {
        self.mutable_descriptor_type |= other.mutable_descriptor_type;
    }
}
impl FeatureStruct for PhysicalDeviceNestedCommandBufferFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.nested_command_buffer & other.nested_command_buffer) == other.nested_command_buffer
            && (self.nested_command_buffer_rendering & other.nested_command_buffer_rendering)
                == other.nested_command_buffer_rendering
            && (self.nested_command_buffer_simultaneous_use
                & other.nested_command_buffer_simultaneous_use)
                == other.nested_command_buffer_simultaneous_use
    }
    fn union_fields(&mut self, other: &Self) {
        self.nested_command_buffer |= other.nested_command_buffer;
        self.nested_command_buffer_rendering |= other.nested_command_buffer_rendering;
        self.nested_command_buffer_simultaneous_use |= other.nested_command_buffer_simultaneous_use;
    }
}
impl FeatureStruct for PhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.non_seamless_cube_map & other.non_seamless_cube_map) == other.non_seamless_cube_map
    }
    fn union_fields(&mut self, other: &Self) {
        self.non_seamless_cube_map |= other.non_seamless_cube_map;
    }
}
impl FeatureStruct for PhysicalDeviceOpacityMicromapFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.micromap & other.micromap) == other.micromap
            && (self.micromap_capture_replay & other.micromap_capture_replay)
                == other.micromap_capture_replay
            && (self.micromap_host_commands & other.micromap_host_commands)
                == other.micromap_host_commands
    }
    fn union_fields(&mut self, other: &Self) {
        self.micromap |= other.micromap;
        self.micromap_capture_replay |= other.micromap_capture_replay;
        self.micromap_host_commands |= other.micromap_host_commands;
    }
}
impl FeatureStruct for PhysicalDeviceOpticalFlowFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.optical_flow & other.optical_flow) == other.optical_flow
    }
    fn union_fields(&mut self, other: &Self) {
        self.optical_flow |= other.optical_flow;
    }
}
impl FeatureStruct for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.pageable_device_local_memory & other.pageable_device_local_memory)
            == other.pageable_device_local_memory
    }
    fn union_fields(&mut self, other: &Self) {
        self.pageable_device_local_memory |= other.pageable_device_local_memory;
    }
}
impl FeatureStruct for PhysicalDevicePerStageDescriptorSetFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.per_stage_descriptor_set & other.per_stage_descriptor_set)
            == other.per_stage_descriptor_set
            && (self.dynamic_pipeline_layout & other.dynamic_pipeline_layout)
                == other.dynamic_pipeline_layout
    }
    fn union_fields(&mut self, other: &Self) {
        self.per_stage_descriptor_set |= other.per_stage_descriptor_set;
        self.dynamic_pipeline_layout |= other.dynamic_pipeline_layout;
    }
}
impl FeatureStruct for PhysicalDevicePerformanceQueryFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.performance_counter_query_pools & other.performance_counter_query_pools)
            == other.performance_counter_query_pools
            && (self.performance_counter_multiple_query_pools
                & other.performance_counter_multiple_query_pools)
                == other.performance_counter_multiple_query_pools
    }
    fn union_fields(&mut self, other: &Self) {
        self.performance_counter_query_pools |= other.performance_counter_query_pools;
        self.performance_counter_multiple_query_pools |=
            other.performance_counter_multiple_query_pools;
    }
}
impl FeatureStruct for PhysicalDevicePipelineCreationCacheControlFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.pipeline_creation_cache_control & other.pipeline_creation_cache_control)
            == other.pipeline_creation_cache_control
    }
    fn union_fields(&mut self, other: &Self) {
        self.pipeline_creation_cache_control |= other.pipeline_creation_cache_control;
    }
}
impl FeatureStruct for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.pipeline_executable_info & other.pipeline_executable_info)
            == other.pipeline_executable_info
    }
    fn union_fields(&mut self, other: &Self) {
        self.pipeline_executable_info |= other.pipeline_executable_info;
    }
}
impl FeatureStruct for PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.pipeline_library_group_handles & other.pipeline_library_group_handles)
            == other.pipeline_library_group_handles
    }
    fn union_fields(&mut self, other: &Self) {
        self.pipeline_library_group_handles |= other.pipeline_library_group_handles;
    }
}
impl FeatureStruct for PhysicalDevicePipelinePropertiesFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.pipeline_properties_identifier & other.pipeline_properties_identifier)
            == other.pipeline_properties_identifier
    }
    fn union_fields(&mut self, other: &Self) {
        self.pipeline_properties_identifier |= other.pipeline_properties_identifier;
    }
}
impl FeatureStruct for PhysicalDevicePipelineProtectedAccessFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.pipeline_protected_access & other.pipeline_protected_access)
            == other.pipeline_protected_access
    }
    fn union_fields(&mut self, other: &Self) {
        self.pipeline_protected_access |= other.pipeline_protected_access;
    }
}
impl FeatureStruct for PhysicalDevicePipelineRobustnessFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.pipeline_robustness & other.pipeline_robustness) == other.pipeline_robustness
    }
    fn union_fields(&mut self, other: &Self) {
        self.pipeline_robustness |= other.pipeline_robustness;
    }
}
impl FeatureStruct for PhysicalDevicePortabilitySubsetFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.constant_alpha_color_blend_factors & other.constant_alpha_color_blend_factors)
            == other.constant_alpha_color_blend_factors
            && (self.events & other.events) == other.events
            && (self.image_view_format_reinterpretation & other.image_view_format_reinterpretation)
                == other.image_view_format_reinterpretation
            && (self.image_view_format_swizzle & other.image_view_format_swizzle)
                == other.image_view_format_swizzle
            && (self.image_view2_d_on3_d_image & other.image_view2_d_on3_d_image)
                == other.image_view2_d_on3_d_image
            && (self.multisample_array_image & other.multisample_array_image)
                == other.multisample_array_image
            && (self.mutable_comparison_samplers & other.mutable_comparison_samplers)
                == other.mutable_comparison_samplers
            && (self.point_polygons & other.point_polygons) == other.point_polygons
            && (self.sampler_mip_lod_bias & other.sampler_mip_lod_bias)
                == other.sampler_mip_lod_bias
            && (self.separate_stencil_mask_ref & other.separate_stencil_mask_ref)
                == other.separate_stencil_mask_ref
            && (self.shader_sample_rate_interpolation_functions
                & other.shader_sample_rate_interpolation_functions)
                == other.shader_sample_rate_interpolation_functions
            && (self.tessellation_isolines & other.tessellation_isolines)
                == other.tessellation_isolines
            && (self.tessellation_point_mode & other.tessellation_point_mode)
                == other.tessellation_point_mode
            && (self.triangle_fans & other.triangle_fans) == other.triangle_fans
            && (self.vertex_attribute_access_beyond_stride
                & other.vertex_attribute_access_beyond_stride)
                == other.vertex_attribute_access_beyond_stride
    }
    fn union_fields(&mut self, other: &Self) {
        self.constant_alpha_color_blend_factors |= other.constant_alpha_color_blend_factors;
        self.events |= other.events;
        self.image_view_format_reinterpretation |= other.image_view_format_reinterpretation;
        self.image_view_format_swizzle |= other.image_view_format_swizzle;
        self.image_view2_d_on3_d_image |= other.image_view2_d_on3_d_image;
        self.multisample_array_image |= other.multisample_array_image;
        self.mutable_comparison_samplers |= other.mutable_comparison_samplers;
        self.point_polygons |= other.point_polygons;
        self.sampler_mip_lod_bias |= other.sampler_mip_lod_bias;
        self.separate_stencil_mask_ref |= other.separate_stencil_mask_ref;
        self.shader_sample_rate_interpolation_functions |=
            other.shader_sample_rate_interpolation_functions;
        self.tessellation_isolines |= other.tessellation_isolines;
        self.tessellation_point_mode |= other.tessellation_point_mode;
        self.triangle_fans |= other.triangle_fans;
        self.vertex_attribute_access_beyond_stride |= other.vertex_attribute_access_beyond_stride;
    }
}
impl FeatureStruct for PhysicalDevicePresentBarrierFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.present_barrier & other.present_barrier) == other.present_barrier
    }
    fn union_fields(&mut self, other: &Self) {
        self.present_barrier |= other.present_barrier;
    }
}
impl FeatureStruct for PhysicalDevicePresentIdFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.present_id & other.present_id) == other.present_id
    }
    fn union_fields(&mut self, other: &Self) {
        self.present_id |= other.present_id;
    }
}
impl FeatureStruct for PhysicalDevicePresentWaitFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.present_wait & other.present_wait) == other.present_wait
    }
    fn union_fields(&mut self, other: &Self) {
        self.present_wait |= other.present_wait;
    }
}
impl FeatureStruct for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.primitive_topology_list_restart & other.primitive_topology_list_restart)
            == other.primitive_topology_list_restart
            && (self.primitive_topology_patch_list_restart
                & other.primitive_topology_patch_list_restart)
                == other.primitive_topology_patch_list_restart
    }
    fn union_fields(&mut self, other: &Self) {
        self.primitive_topology_list_restart |= other.primitive_topology_list_restart;
        self.primitive_topology_patch_list_restart |= other.primitive_topology_patch_list_restart;
    }
}
impl FeatureStruct for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.primitives_generated_query & other.primitives_generated_query)
            == other.primitives_generated_query
            && (self.primitives_generated_query_with_rasterizer_discard
                & other.primitives_generated_query_with_rasterizer_discard)
                == other.primitives_generated_query_with_rasterizer_discard
            && (self.primitives_generated_query_with_non_zero_streams
                & other.primitives_generated_query_with_non_zero_streams)
                == other.primitives_generated_query_with_non_zero_streams
    }
    fn union_fields(&mut self, other: &Self) {
        self.primitives_generated_query |= other.primitives_generated_query;
        self.primitives_generated_query_with_rasterizer_discard |=
            other.primitives_generated_query_with_rasterizer_discard;
        self.primitives_generated_query_with_non_zero_streams |=
            other.primitives_generated_query_with_non_zero_streams;
    }
}
impl FeatureStruct for PhysicalDevicePrivateDataFeatures<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.private_data & other.private_data) == other.private_data
    }
    fn union_fields(&mut self, other: &Self) {
        self.private_data |= other.private_data;
    }
}
impl FeatureStruct for PhysicalDeviceProtectedMemoryFeatures<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.protected_memory & other.protected_memory) == other.protected_memory
    }
    fn union_fields(&mut self, other: &Self) {
        self.protected_memory |= other.protected_memory;
    }
}
impl FeatureStruct for PhysicalDeviceProvokingVertexFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.provoking_vertex_last & other.provoking_vertex_last) == other.provoking_vertex_last
            && (self.transform_feedback_preserves_provoking_vertex
                & other.transform_feedback_preserves_provoking_vertex)
                == other.transform_feedback_preserves_provoking_vertex
    }
    fn union_fields(&mut self, other: &Self) {
        self.provoking_vertex_last |= other.provoking_vertex_last;
        self.transform_feedback_preserves_provoking_vertex |=
            other.transform_feedback_preserves_provoking_vertex;
    }
}
impl FeatureStruct for PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.format_rgba10x6_without_y_cb_cr_sampler
            & other.format_rgba10x6_without_y_cb_cr_sampler)
            == other.format_rgba10x6_without_y_cb_cr_sampler
    }
    fn union_fields(&mut self, other: &Self) {
        self.format_rgba10x6_without_y_cb_cr_sampler |=
            other.format_rgba10x6_without_y_cb_cr_sampler;
    }
}
impl FeatureStruct for PhysicalDeviceRawAccessChainsFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_raw_access_chains & other.shader_raw_access_chains)
            == other.shader_raw_access_chains
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_raw_access_chains |= other.shader_raw_access_chains;
    }
}
impl FeatureStruct for PhysicalDeviceRayQueryFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.ray_query & other.ray_query) == other.ray_query
    }
    fn union_fields(&mut self, other: &Self) {
        self.ray_query |= other.ray_query;
    }
}
impl FeatureStruct for PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.ray_tracing_invocation_reorder & other.ray_tracing_invocation_reorder)
            == other.ray_tracing_invocation_reorder
    }
    fn union_fields(&mut self, other: &Self) {
        self.ray_tracing_invocation_reorder |= other.ray_tracing_invocation_reorder;
    }
}
impl FeatureStruct for PhysicalDeviceRayTracingMaintenance1FeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.ray_tracing_maintenance1 & other.ray_tracing_maintenance1)
            == other.ray_tracing_maintenance1
            && (self.ray_tracing_pipeline_trace_rays_indirect2
                & other.ray_tracing_pipeline_trace_rays_indirect2)
                == other.ray_tracing_pipeline_trace_rays_indirect2
    }
    fn union_fields(&mut self, other: &Self) {
        self.ray_tracing_maintenance1 |= other.ray_tracing_maintenance1;
        self.ray_tracing_pipeline_trace_rays_indirect2 |=
            other.ray_tracing_pipeline_trace_rays_indirect2;
    }
}
impl FeatureStruct for PhysicalDeviceRayTracingMotionBlurFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.ray_tracing_motion_blur & other.ray_tracing_motion_blur)
            == other.ray_tracing_motion_blur
            && (self.ray_tracing_motion_blur_pipeline_trace_rays_indirect
                & other.ray_tracing_motion_blur_pipeline_trace_rays_indirect)
                == other.ray_tracing_motion_blur_pipeline_trace_rays_indirect
    }
    fn union_fields(&mut self, other: &Self) {
        self.ray_tracing_motion_blur |= other.ray_tracing_motion_blur;
        self.ray_tracing_motion_blur_pipeline_trace_rays_indirect |=
            other.ray_tracing_motion_blur_pipeline_trace_rays_indirect;
    }
}
impl FeatureStruct for PhysicalDeviceRayTracingPipelineFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.ray_tracing_pipeline & other.ray_tracing_pipeline) == other.ray_tracing_pipeline
            && (self.ray_tracing_pipeline_shader_group_handle_capture_replay
                & other.ray_tracing_pipeline_shader_group_handle_capture_replay)
                == other.ray_tracing_pipeline_shader_group_handle_capture_replay
            && (self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed
                & other.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed)
                == other.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed
            && (self.ray_tracing_pipeline_trace_rays_indirect
                & other.ray_tracing_pipeline_trace_rays_indirect)
                == other.ray_tracing_pipeline_trace_rays_indirect
            && (self.ray_traversal_primitive_culling & other.ray_traversal_primitive_culling)
                == other.ray_traversal_primitive_culling
    }
    fn union_fields(&mut self, other: &Self) {
        self.ray_tracing_pipeline |= other.ray_tracing_pipeline;
        self.ray_tracing_pipeline_shader_group_handle_capture_replay |=
            other.ray_tracing_pipeline_shader_group_handle_capture_replay;
        self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed |=
            other.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed;
        self.ray_tracing_pipeline_trace_rays_indirect |=
            other.ray_tracing_pipeline_trace_rays_indirect;
        self.ray_traversal_primitive_culling |= other.ray_traversal_primitive_culling;
    }
}
impl FeatureStruct for PhysicalDeviceRayTracingPositionFetchFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.ray_tracing_position_fetch & other.ray_tracing_position_fetch)
            == other.ray_tracing_position_fetch
    }
    fn union_fields(&mut self, other: &Self) {
        self.ray_tracing_position_fetch |= other.ray_tracing_position_fetch;
    }
}
impl FeatureStruct for PhysicalDeviceRayTracingValidationFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.ray_tracing_validation & other.ray_tracing_validation) == other.ray_tracing_validation
    }
    fn union_fields(&mut self, other: &Self) {
        self.ray_tracing_validation |= other.ray_tracing_validation;
    }
}
impl FeatureStruct for PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.relaxed_line_rasterization & other.relaxed_line_rasterization)
            == other.relaxed_line_rasterization
    }
    fn union_fields(&mut self, other: &Self) {
        self.relaxed_line_rasterization |= other.relaxed_line_rasterization;
    }
}
impl FeatureStruct for PhysicalDeviceRenderPassStripedFeaturesARM<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.render_pass_striped & other.render_pass_striped) == other.render_pass_striped
    }
    fn union_fields(&mut self, other: &Self) {
        self.render_pass_striped |= other.render_pass_striped;
    }
}
impl FeatureStruct for PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.representative_fragment_test & other.representative_fragment_test)
            == other.representative_fragment_test
    }
    fn union_fields(&mut self, other: &Self) {
        self.representative_fragment_test |= other.representative_fragment_test;
    }
}
impl FeatureStruct for PhysicalDeviceRobustness2FeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.robust_buffer_access2 & other.robust_buffer_access2) == other.robust_buffer_access2
            && (self.robust_image_access2 & other.robust_image_access2)
                == other.robust_image_access2
            && (self.null_descriptor & other.null_descriptor) == other.null_descriptor
    }
    fn union_fields(&mut self, other: &Self) {
        self.robust_buffer_access2 |= other.robust_buffer_access2;
        self.robust_image_access2 |= other.robust_image_access2;
        self.null_descriptor |= other.null_descriptor;
    }
}
impl FeatureStruct for PhysicalDeviceSamplerYcbcrConversionFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.sampler_ycbcr_conversion & other.sampler_ycbcr_conversion)
            == other.sampler_ycbcr_conversion
    }
    fn union_fields(&mut self, other: &Self) {
        self.sampler_ycbcr_conversion |= other.sampler_ycbcr_conversion;
    }
}
impl FeatureStruct for PhysicalDeviceScalarBlockLayoutFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.scalar_block_layout & other.scalar_block_layout) == other.scalar_block_layout
    }
    fn union_fields(&mut self, other: &Self) {
        self.scalar_block_layout |= other.scalar_block_layout;
    }
}
impl FeatureStruct for PhysicalDeviceSchedulingControlsFeaturesARM<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.scheduling_controls & other.scheduling_controls) == other.scheduling_controls
    }
    fn union_fields(&mut self, other: &Self) {
        self.scheduling_controls |= other.scheduling_controls;
    }
}
impl FeatureStruct for PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.separate_depth_stencil_layouts & other.separate_depth_stencil_layouts)
            == other.separate_depth_stencil_layouts
    }
    fn union_fields(&mut self, other: &Self) {
        self.separate_depth_stencil_layouts |= other.separate_depth_stencil_layouts;
    }
}
impl FeatureStruct for PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_float16_vector_atomics & other.shader_float16_vector_atomics)
            == other.shader_float16_vector_atomics
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_float16_vector_atomics |= other.shader_float16_vector_atomics;
    }
}
impl FeatureStruct for PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_buffer_float16_atomics & other.shader_buffer_float16_atomics)
            == other.shader_buffer_float16_atomics
            && (self.shader_buffer_float16_atomic_add & other.shader_buffer_float16_atomic_add)
                == other.shader_buffer_float16_atomic_add
            && (self.shader_buffer_float16_atomic_min_max
                & other.shader_buffer_float16_atomic_min_max)
                == other.shader_buffer_float16_atomic_min_max
            && (self.shader_buffer_float32_atomic_min_max
                & other.shader_buffer_float32_atomic_min_max)
                == other.shader_buffer_float32_atomic_min_max
            && (self.shader_buffer_float64_atomic_min_max
                & other.shader_buffer_float64_atomic_min_max)
                == other.shader_buffer_float64_atomic_min_max
            && (self.shader_shared_float16_atomics & other.shader_shared_float16_atomics)
                == other.shader_shared_float16_atomics
            && (self.shader_shared_float16_atomic_add & other.shader_shared_float16_atomic_add)
                == other.shader_shared_float16_atomic_add
            && (self.shader_shared_float16_atomic_min_max
                & other.shader_shared_float16_atomic_min_max)
                == other.shader_shared_float16_atomic_min_max
            && (self.shader_shared_float32_atomic_min_max
                & other.shader_shared_float32_atomic_min_max)
                == other.shader_shared_float32_atomic_min_max
            && (self.shader_shared_float64_atomic_min_max
                & other.shader_shared_float64_atomic_min_max)
                == other.shader_shared_float64_atomic_min_max
            && (self.shader_image_float32_atomic_min_max
                & other.shader_image_float32_atomic_min_max)
                == other.shader_image_float32_atomic_min_max
            && (self.sparse_image_float32_atomic_min_max
                & other.sparse_image_float32_atomic_min_max)
                == other.sparse_image_float32_atomic_min_max
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_buffer_float16_atomics |= other.shader_buffer_float16_atomics;
        self.shader_buffer_float16_atomic_add |= other.shader_buffer_float16_atomic_add;
        self.shader_buffer_float16_atomic_min_max |= other.shader_buffer_float16_atomic_min_max;
        self.shader_buffer_float32_atomic_min_max |= other.shader_buffer_float32_atomic_min_max;
        self.shader_buffer_float64_atomic_min_max |= other.shader_buffer_float64_atomic_min_max;
        self.shader_shared_float16_atomics |= other.shader_shared_float16_atomics;
        self.shader_shared_float16_atomic_add |= other.shader_shared_float16_atomic_add;
        self.shader_shared_float16_atomic_min_max |= other.shader_shared_float16_atomic_min_max;
        self.shader_shared_float32_atomic_min_max |= other.shader_shared_float32_atomic_min_max;
        self.shader_shared_float64_atomic_min_max |= other.shader_shared_float64_atomic_min_max;
        self.shader_image_float32_atomic_min_max |= other.shader_image_float32_atomic_min_max;
        self.sparse_image_float32_atomic_min_max |= other.sparse_image_float32_atomic_min_max;
    }
}
impl FeatureStruct for PhysicalDeviceShaderAtomicFloatFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_buffer_float32_atomics & other.shader_buffer_float32_atomics)
            == other.shader_buffer_float32_atomics
            && (self.shader_buffer_float32_atomic_add & other.shader_buffer_float32_atomic_add)
                == other.shader_buffer_float32_atomic_add
            && (self.shader_buffer_float64_atomics & other.shader_buffer_float64_atomics)
                == other.shader_buffer_float64_atomics
            && (self.shader_buffer_float64_atomic_add & other.shader_buffer_float64_atomic_add)
                == other.shader_buffer_float64_atomic_add
            && (self.shader_shared_float32_atomics & other.shader_shared_float32_atomics)
                == other.shader_shared_float32_atomics
            && (self.shader_shared_float32_atomic_add & other.shader_shared_float32_atomic_add)
                == other.shader_shared_float32_atomic_add
            && (self.shader_shared_float64_atomics & other.shader_shared_float64_atomics)
                == other.shader_shared_float64_atomics
            && (self.shader_shared_float64_atomic_add & other.shader_shared_float64_atomic_add)
                == other.shader_shared_float64_atomic_add
            && (self.shader_image_float32_atomics & other.shader_image_float32_atomics)
                == other.shader_image_float32_atomics
            && (self.shader_image_float32_atomic_add & other.shader_image_float32_atomic_add)
                == other.shader_image_float32_atomic_add
            && (self.sparse_image_float32_atomics & other.sparse_image_float32_atomics)
                == other.sparse_image_float32_atomics
            && (self.sparse_image_float32_atomic_add & other.sparse_image_float32_atomic_add)
                == other.sparse_image_float32_atomic_add
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_buffer_float32_atomics |= other.shader_buffer_float32_atomics;
        self.shader_buffer_float32_atomic_add |= other.shader_buffer_float32_atomic_add;
        self.shader_buffer_float64_atomics |= other.shader_buffer_float64_atomics;
        self.shader_buffer_float64_atomic_add |= other.shader_buffer_float64_atomic_add;
        self.shader_shared_float32_atomics |= other.shader_shared_float32_atomics;
        self.shader_shared_float32_atomic_add |= other.shader_shared_float32_atomic_add;
        self.shader_shared_float64_atomics |= other.shader_shared_float64_atomics;
        self.shader_shared_float64_atomic_add |= other.shader_shared_float64_atomic_add;
        self.shader_image_float32_atomics |= other.shader_image_float32_atomics;
        self.shader_image_float32_atomic_add |= other.shader_image_float32_atomic_add;
        self.sparse_image_float32_atomics |= other.sparse_image_float32_atomics;
        self.sparse_image_float32_atomic_add |= other.sparse_image_float32_atomic_add;
    }
}
impl FeatureStruct for PhysicalDeviceShaderAtomicInt64Features<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_buffer_int64_atomics & other.shader_buffer_int64_atomics)
            == other.shader_buffer_int64_atomics
            && (self.shader_shared_int64_atomics & other.shader_shared_int64_atomics)
                == other.shader_shared_int64_atomics
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_buffer_int64_atomics |= other.shader_buffer_int64_atomics;
        self.shader_shared_int64_atomics |= other.shader_shared_int64_atomics;
    }
}
impl FeatureStruct for PhysicalDeviceShaderClockFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_subgroup_clock & other.shader_subgroup_clock) == other.shader_subgroup_clock
            && (self.shader_device_clock & other.shader_device_clock) == other.shader_device_clock
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_subgroup_clock |= other.shader_subgroup_clock;
        self.shader_device_clock |= other.shader_device_clock;
    }
}
impl FeatureStruct for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_core_builtins & other.shader_core_builtins) == other.shader_core_builtins
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_core_builtins |= other.shader_core_builtins;
    }
}
impl FeatureStruct for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_demote_to_helper_invocation & other.shader_demote_to_helper_invocation)
            == other.shader_demote_to_helper_invocation
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_demote_to_helper_invocation |= other.shader_demote_to_helper_invocation;
    }
}
impl FeatureStruct for PhysicalDeviceShaderDrawParametersFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_draw_parameters & other.shader_draw_parameters) == other.shader_draw_parameters
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_draw_parameters |= other.shader_draw_parameters;
    }
}
impl FeatureStruct for PhysicalDeviceShaderEnqueueFeaturesAMDX<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_enqueue & other.shader_enqueue) == other.shader_enqueue
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_enqueue |= other.shader_enqueue;
    }
}
impl FeatureStruct for PhysicalDeviceShaderExpectAssumeFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_expect_assume & other.shader_expect_assume) == other.shader_expect_assume
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_expect_assume |= other.shader_expect_assume;
    }
}
impl FeatureStruct for PhysicalDeviceShaderFloat16Int8Features<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_float16 & other.shader_float16) == other.shader_float16
            && (self.shader_int8 & other.shader_int8) == other.shader_int8
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_float16 |= other.shader_float16;
        self.shader_int8 |= other.shader_int8;
    }
}
impl FeatureStruct for PhysicalDeviceShaderFloatControls2FeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_float_controls2 & other.shader_float_controls2) == other.shader_float_controls2
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_float_controls2 |= other.shader_float_controls2;
    }
}
impl FeatureStruct for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_image_int64_atomics & other.shader_image_int64_atomics)
            == other.shader_image_int64_atomics
            && (self.sparse_image_int64_atomics & other.sparse_image_int64_atomics)
                == other.sparse_image_int64_atomics
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_image_int64_atomics |= other.shader_image_int64_atomics;
        self.sparse_image_int64_atomics |= other.sparse_image_int64_atomics;
    }
}
impl FeatureStruct for PhysicalDeviceShaderImageFootprintFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.image_footprint & other.image_footprint) == other.image_footprint
    }
    fn union_fields(&mut self, other: &Self) {
        self.image_footprint |= other.image_footprint;
    }
}
impl FeatureStruct for PhysicalDeviceShaderIntegerDotProductFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_integer_dot_product & other.shader_integer_dot_product)
            == other.shader_integer_dot_product
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_integer_dot_product |= other.shader_integer_dot_product;
    }
}
impl FeatureStruct for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_integer_functions2 & other.shader_integer_functions2)
            == other.shader_integer_functions2
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_integer_functions2 |= other.shader_integer_functions2;
    }
}
impl FeatureStruct for PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_maximal_reconvergence & other.shader_maximal_reconvergence)
            == other.shader_maximal_reconvergence
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_maximal_reconvergence |= other.shader_maximal_reconvergence;
    }
}
impl FeatureStruct for PhysicalDeviceShaderModuleIdentifierFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_module_identifier & other.shader_module_identifier)
            == other.shader_module_identifier
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_module_identifier |= other.shader_module_identifier;
    }
}
impl FeatureStruct for PhysicalDeviceShaderObjectFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_object & other.shader_object) == other.shader_object
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_object |= other.shader_object;
    }
}
impl FeatureStruct for PhysicalDeviceShaderQuadControlFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_quad_control & other.shader_quad_control) == other.shader_quad_control
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_quad_control |= other.shader_quad_control;
    }
}
impl FeatureStruct for PhysicalDeviceShaderSMBuiltinsFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_sm_builtins & other.shader_sm_builtins) == other.shader_sm_builtins
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_sm_builtins |= other.shader_sm_builtins;
    }
}
impl FeatureStruct for PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_subgroup_extended_types & other.shader_subgroup_extended_types)
            == other.shader_subgroup_extended_types
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_subgroup_extended_types |= other.shader_subgroup_extended_types;
    }
}
impl FeatureStruct for PhysicalDeviceShaderSubgroupRotateFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_subgroup_rotate & other.shader_subgroup_rotate) == other.shader_subgroup_rotate
            && (self.shader_subgroup_rotate_clustered & other.shader_subgroup_rotate_clustered)
                == other.shader_subgroup_rotate_clustered
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_subgroup_rotate |= other.shader_subgroup_rotate;
        self.shader_subgroup_rotate_clustered |= other.shader_subgroup_rotate_clustered;
    }
}
impl FeatureStruct for PhysicalDeviceShaderTerminateInvocationFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_terminate_invocation & other.shader_terminate_invocation)
            == other.shader_terminate_invocation
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_terminate_invocation |= other.shader_terminate_invocation;
    }
}
impl FeatureStruct for PhysicalDeviceShaderTileImageFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_tile_image_color_read_access & other.shader_tile_image_color_read_access)
            == other.shader_tile_image_color_read_access
            && (self.shader_tile_image_depth_read_access
                & other.shader_tile_image_depth_read_access)
                == other.shader_tile_image_depth_read_access
            && (self.shader_tile_image_stencil_read_access
                & other.shader_tile_image_stencil_read_access)
                == other.shader_tile_image_stencil_read_access
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_tile_image_color_read_access |= other.shader_tile_image_color_read_access;
        self.shader_tile_image_depth_read_access |= other.shader_tile_image_depth_read_access;
        self.shader_tile_image_stencil_read_access |= other.shader_tile_image_stencil_read_access;
    }
}
impl FeatureStruct for PhysicalDeviceShadingRateImageFeaturesNV<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shading_rate_image & other.shading_rate_image) == other.shading_rate_image
            && (self.shading_rate_coarse_sample_order & other.shading_rate_coarse_sample_order)
                == other.shading_rate_coarse_sample_order
    }
    fn union_fields(&mut self, other: &Self) {
        self.shading_rate_image |= other.shading_rate_image;
        self.shading_rate_coarse_sample_order |= other.shading_rate_coarse_sample_order;
    }
}
impl FeatureStruct for PhysicalDeviceSubgroupSizeControlFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.subgroup_size_control & other.subgroup_size_control) == other.subgroup_size_control
            && (self.compute_full_subgroups & other.compute_full_subgroups)
                == other.compute_full_subgroups
    }
    fn union_fields(&mut self, other: &Self) {
        self.subgroup_size_control |= other.subgroup_size_control;
        self.compute_full_subgroups |= other.compute_full_subgroups;
    }
}
impl FeatureStruct for PhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.subpass_merge_feedback & other.subpass_merge_feedback) == other.subpass_merge_feedback
    }
    fn union_fields(&mut self, other: &Self) {
        self.subpass_merge_feedback |= other.subpass_merge_feedback;
    }
}
impl FeatureStruct for PhysicalDeviceSubpassShadingFeaturesHUAWEI<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.subpass_shading & other.subpass_shading) == other.subpass_shading
    }
    fn union_fields(&mut self, other: &Self) {
        self.subpass_shading |= other.subpass_shading;
    }
}
impl FeatureStruct for PhysicalDeviceSwapchainMaintenance1FeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.swapchain_maintenance1 & other.swapchain_maintenance1) == other.swapchain_maintenance1
    }
    fn union_fields(&mut self, other: &Self) {
        self.swapchain_maintenance1 |= other.swapchain_maintenance1;
    }
}
impl FeatureStruct for PhysicalDeviceSynchronization2Features<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.synchronization2 & other.synchronization2) == other.synchronization2
    }
    fn union_fields(&mut self, other: &Self) {
        self.synchronization2 |= other.synchronization2;
    }
}
impl FeatureStruct for PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.texel_buffer_alignment & other.texel_buffer_alignment) == other.texel_buffer_alignment
    }
    fn union_fields(&mut self, other: &Self) {
        self.texel_buffer_alignment |= other.texel_buffer_alignment;
    }
}
impl FeatureStruct for PhysicalDeviceTextureCompressionASTCHDRFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.texture_compression_astc_hdr & other.texture_compression_astc_hdr)
            == other.texture_compression_astc_hdr
    }
    fn union_fields(&mut self, other: &Self) {
        self.texture_compression_astc_hdr |= other.texture_compression_astc_hdr;
    }
}
impl FeatureStruct for PhysicalDeviceTilePropertiesFeaturesQCOM<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.tile_properties & other.tile_properties) == other.tile_properties
    }
    fn union_fields(&mut self, other: &Self) {
        self.tile_properties |= other.tile_properties;
    }
}
impl FeatureStruct for PhysicalDeviceTimelineSemaphoreFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.timeline_semaphore & other.timeline_semaphore) == other.timeline_semaphore
    }
    fn union_fields(&mut self, other: &Self) {
        self.timeline_semaphore |= other.timeline_semaphore;
    }
}
impl FeatureStruct for PhysicalDeviceTransformFeedbackFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.transform_feedback & other.transform_feedback) == other.transform_feedback
            && (self.geometry_streams & other.geometry_streams) == other.geometry_streams
    }
    fn union_fields(&mut self, other: &Self) {
        self.transform_feedback |= other.transform_feedback;
        self.geometry_streams |= other.geometry_streams;
    }
}
impl FeatureStruct for PhysicalDeviceUniformBufferStandardLayoutFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.uniform_buffer_standard_layout & other.uniform_buffer_standard_layout)
            == other.uniform_buffer_standard_layout
    }
    fn union_fields(&mut self, other: &Self) {
        self.uniform_buffer_standard_layout |= other.uniform_buffer_standard_layout;
    }
}
impl FeatureStruct for PhysicalDeviceVariablePointersFeatures<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.variable_pointers_storage_buffer & other.variable_pointers_storage_buffer)
            == other.variable_pointers_storage_buffer
            && (self.variable_pointers & other.variable_pointers) == other.variable_pointers
    }
    fn union_fields(&mut self, other: &Self) {
        self.variable_pointers_storage_buffer |= other.variable_pointers_storage_buffer;
        self.variable_pointers |= other.variable_pointers;
    }
}
impl FeatureStruct for PhysicalDeviceVertexAttributeDivisorFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.vertex_attribute_instance_rate_divisor & other.vertex_attribute_instance_rate_divisor)
            == other.vertex_attribute_instance_rate_divisor
            && (self.vertex_attribute_instance_rate_zero_divisor
                & other.vertex_attribute_instance_rate_zero_divisor)
                == other.vertex_attribute_instance_rate_zero_divisor
    }
    fn union_fields(&mut self, other: &Self) {
        self.vertex_attribute_instance_rate_divisor |= other.vertex_attribute_instance_rate_divisor;
        self.vertex_attribute_instance_rate_zero_divisor |=
            other.vertex_attribute_instance_rate_zero_divisor;
    }
}
impl FeatureStruct for PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.vertex_input_dynamic_state & other.vertex_input_dynamic_state)
            == other.vertex_input_dynamic_state
    }
    fn union_fields(&mut self, other: &Self) {
        self.vertex_input_dynamic_state |= other.vertex_input_dynamic_state;
    }
}
impl FeatureStruct for PhysicalDeviceVideoMaintenance1FeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.video_maintenance1 & other.video_maintenance1) == other.video_maintenance1
    }
    fn union_fields(&mut self, other: &Self) {
        self.video_maintenance1 |= other.video_maintenance1;
    }
}
impl FeatureStruct for PhysicalDeviceVulkan11Features<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.storage_buffer16_bit_access & other.storage_buffer16_bit_access)
            == other.storage_buffer16_bit_access
            && (self.uniform_and_storage_buffer16_bit_access
                & other.uniform_and_storage_buffer16_bit_access)
                == other.uniform_and_storage_buffer16_bit_access
            && (self.storage_push_constant16 & other.storage_push_constant16)
                == other.storage_push_constant16
            && (self.storage_input_output16 & other.storage_input_output16)
                == other.storage_input_output16
            && (self.multiview & other.multiview) == other.multiview
            && (self.multiview_geometry_shader & other.multiview_geometry_shader)
                == other.multiview_geometry_shader
            && (self.multiview_tessellation_shader & other.multiview_tessellation_shader)
                == other.multiview_tessellation_shader
            && (self.variable_pointers_storage_buffer & other.variable_pointers_storage_buffer)
                == other.variable_pointers_storage_buffer
            && (self.variable_pointers & other.variable_pointers) == other.variable_pointers
            && (self.protected_memory & other.protected_memory) == other.protected_memory
            && (self.sampler_ycbcr_conversion & other.sampler_ycbcr_conversion)
                == other.sampler_ycbcr_conversion
            && (self.shader_draw_parameters & other.shader_draw_parameters)
                == other.shader_draw_parameters
    }
    fn union_fields(&mut self, other: &Self) {
        self.storage_buffer16_bit_access |= other.storage_buffer16_bit_access;
        self.uniform_and_storage_buffer16_bit_access |=
            other.uniform_and_storage_buffer16_bit_access;
        self.storage_push_constant16 |= other.storage_push_constant16;
        self.storage_input_output16 |= other.storage_input_output16;
        self.multiview |= other.multiview;
        self.multiview_geometry_shader |= other.multiview_geometry_shader;
        self.multiview_tessellation_shader |= other.multiview_tessellation_shader;
        self.variable_pointers_storage_buffer |= other.variable_pointers_storage_buffer;
        self.variable_pointers |= other.variable_pointers;
        self.protected_memory |= other.protected_memory;
        self.sampler_ycbcr_conversion |= other.sampler_ycbcr_conversion;
        self.shader_draw_parameters |= other.shader_draw_parameters;
    }
}
impl FeatureStruct for PhysicalDeviceVulkan12Features<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.sampler_mirror_clamp_to_edge & other.sampler_mirror_clamp_to_edge)
            == other.sampler_mirror_clamp_to_edge
            && (self.draw_indirect_count & other.draw_indirect_count) == other.draw_indirect_count
            && (self.storage_buffer8_bit_access & other.storage_buffer8_bit_access)
                == other.storage_buffer8_bit_access
            && (self.uniform_and_storage_buffer8_bit_access
                & other.uniform_and_storage_buffer8_bit_access)
                == other.uniform_and_storage_buffer8_bit_access
            && (self.storage_push_constant8 & other.storage_push_constant8)
                == other.storage_push_constant8
            && (self.shader_buffer_int64_atomics & other.shader_buffer_int64_atomics)
                == other.shader_buffer_int64_atomics
            && (self.shader_shared_int64_atomics & other.shader_shared_int64_atomics)
                == other.shader_shared_int64_atomics
            && (self.shader_float16 & other.shader_float16) == other.shader_float16
            && (self.shader_int8 & other.shader_int8) == other.shader_int8
            && (self.descriptor_indexing & other.descriptor_indexing) == other.descriptor_indexing
            && (self.shader_input_attachment_array_dynamic_indexing
                & other.shader_input_attachment_array_dynamic_indexing)
                == other.shader_input_attachment_array_dynamic_indexing
            && (self.shader_uniform_texel_buffer_array_dynamic_indexing
                & other.shader_uniform_texel_buffer_array_dynamic_indexing)
                == other.shader_uniform_texel_buffer_array_dynamic_indexing
            && (self.shader_storage_texel_buffer_array_dynamic_indexing
                & other.shader_storage_texel_buffer_array_dynamic_indexing)
                == other.shader_storage_texel_buffer_array_dynamic_indexing
            && (self.shader_uniform_buffer_array_non_uniform_indexing
                & other.shader_uniform_buffer_array_non_uniform_indexing)
                == other.shader_uniform_buffer_array_non_uniform_indexing
            && (self.shader_sampled_image_array_non_uniform_indexing
                & other.shader_sampled_image_array_non_uniform_indexing)
                == other.shader_sampled_image_array_non_uniform_indexing
            && (self.shader_storage_buffer_array_non_uniform_indexing
                & other.shader_storage_buffer_array_non_uniform_indexing)
                == other.shader_storage_buffer_array_non_uniform_indexing
            && (self.shader_storage_image_array_non_uniform_indexing
                & other.shader_storage_image_array_non_uniform_indexing)
                == other.shader_storage_image_array_non_uniform_indexing
            && (self.shader_input_attachment_array_non_uniform_indexing
                & other.shader_input_attachment_array_non_uniform_indexing)
                == other.shader_input_attachment_array_non_uniform_indexing
            && (self.shader_uniform_texel_buffer_array_non_uniform_indexing
                & other.shader_uniform_texel_buffer_array_non_uniform_indexing)
                == other.shader_uniform_texel_buffer_array_non_uniform_indexing
            && (self.shader_storage_texel_buffer_array_non_uniform_indexing
                & other.shader_storage_texel_buffer_array_non_uniform_indexing)
                == other.shader_storage_texel_buffer_array_non_uniform_indexing
            && (self.descriptor_binding_uniform_buffer_update_after_bind
                & other.descriptor_binding_uniform_buffer_update_after_bind)
                == other.descriptor_binding_uniform_buffer_update_after_bind
            && (self.descriptor_binding_sampled_image_update_after_bind
                & other.descriptor_binding_sampled_image_update_after_bind)
                == other.descriptor_binding_sampled_image_update_after_bind
            && (self.descriptor_binding_storage_image_update_after_bind
                & other.descriptor_binding_storage_image_update_after_bind)
                == other.descriptor_binding_storage_image_update_after_bind
            && (self.descriptor_binding_storage_buffer_update_after_bind
                & other.descriptor_binding_storage_buffer_update_after_bind)
                == other.descriptor_binding_storage_buffer_update_after_bind
            && (self.descriptor_binding_uniform_texel_buffer_update_after_bind
                & other.descriptor_binding_uniform_texel_buffer_update_after_bind)
                == other.descriptor_binding_uniform_texel_buffer_update_after_bind
            && (self.descriptor_binding_storage_texel_buffer_update_after_bind
                & other.descriptor_binding_storage_texel_buffer_update_after_bind)
                == other.descriptor_binding_storage_texel_buffer_update_after_bind
            && (self.descriptor_binding_update_unused_while_pending
                & other.descriptor_binding_update_unused_while_pending)
                == other.descriptor_binding_update_unused_while_pending
            && (self.descriptor_binding_partially_bound & other.descriptor_binding_partially_bound)
                == other.descriptor_binding_partially_bound
            && (self.descriptor_binding_variable_descriptor_count
                & other.descriptor_binding_variable_descriptor_count)
                == other.descriptor_binding_variable_descriptor_count
            && (self.runtime_descriptor_array & other.runtime_descriptor_array)
                == other.runtime_descriptor_array
            && (self.sampler_filter_minmax & other.sampler_filter_minmax)
                == other.sampler_filter_minmax
            && (self.scalar_block_layout & other.scalar_block_layout) == other.scalar_block_layout
            && (self.imageless_framebuffer & other.imageless_framebuffer)
                == other.imageless_framebuffer
            && (self.uniform_buffer_standard_layout & other.uniform_buffer_standard_layout)
                == other.uniform_buffer_standard_layout
            && (self.shader_subgroup_extended_types & other.shader_subgroup_extended_types)
                == other.shader_subgroup_extended_types
            && (self.separate_depth_stencil_layouts & other.separate_depth_stencil_layouts)
                == other.separate_depth_stencil_layouts
            && (self.host_query_reset & other.host_query_reset) == other.host_query_reset
            && (self.timeline_semaphore & other.timeline_semaphore) == other.timeline_semaphore
            && (self.buffer_device_address & other.buffer_device_address)
                == other.buffer_device_address
            && (self.buffer_device_address_capture_replay
                & other.buffer_device_address_capture_replay)
                == other.buffer_device_address_capture_replay
            && (self.buffer_device_address_multi_device & other.buffer_device_address_multi_device)
                == other.buffer_device_address_multi_device
            && (self.vulkan_memory_model & other.vulkan_memory_model) == other.vulkan_memory_model
            && (self.vulkan_memory_model_device_scope & other.vulkan_memory_model_device_scope)
                == other.vulkan_memory_model_device_scope
            && (self.vulkan_memory_model_availability_visibility_chains
                & other.vulkan_memory_model_availability_visibility_chains)
                == other.vulkan_memory_model_availability_visibility_chains
            && (self.shader_output_viewport_index & other.shader_output_viewport_index)
                == other.shader_output_viewport_index
            && (self.shader_output_layer & other.shader_output_layer) == other.shader_output_layer
            && (self.subgroup_broadcast_dynamic_id & other.subgroup_broadcast_dynamic_id)
                == other.subgroup_broadcast_dynamic_id
    }
    fn union_fields(&mut self, other: &Self) {
        self.sampler_mirror_clamp_to_edge |= other.sampler_mirror_clamp_to_edge;
        self.draw_indirect_count |= other.draw_indirect_count;
        self.storage_buffer8_bit_access |= other.storage_buffer8_bit_access;
        self.uniform_and_storage_buffer8_bit_access |= other.uniform_and_storage_buffer8_bit_access;
        self.storage_push_constant8 |= other.storage_push_constant8;
        self.shader_buffer_int64_atomics |= other.shader_buffer_int64_atomics;
        self.shader_shared_int64_atomics |= other.shader_shared_int64_atomics;
        self.shader_float16 |= other.shader_float16;
        self.shader_int8 |= other.shader_int8;
        self.descriptor_indexing |= other.descriptor_indexing;
        self.shader_input_attachment_array_dynamic_indexing |=
            other.shader_input_attachment_array_dynamic_indexing;
        self.shader_uniform_texel_buffer_array_dynamic_indexing |=
            other.shader_uniform_texel_buffer_array_dynamic_indexing;
        self.shader_storage_texel_buffer_array_dynamic_indexing |=
            other.shader_storage_texel_buffer_array_dynamic_indexing;
        self.shader_uniform_buffer_array_non_uniform_indexing |=
            other.shader_uniform_buffer_array_non_uniform_indexing;
        self.shader_sampled_image_array_non_uniform_indexing |=
            other.shader_sampled_image_array_non_uniform_indexing;
        self.shader_storage_buffer_array_non_uniform_indexing |=
            other.shader_storage_buffer_array_non_uniform_indexing;
        self.shader_storage_image_array_non_uniform_indexing |=
            other.shader_storage_image_array_non_uniform_indexing;
        self.shader_input_attachment_array_non_uniform_indexing |=
            other.shader_input_attachment_array_non_uniform_indexing;
        self.shader_uniform_texel_buffer_array_non_uniform_indexing |=
            other.shader_uniform_texel_buffer_array_non_uniform_indexing;
        self.shader_storage_texel_buffer_array_non_uniform_indexing |=
            other.shader_storage_texel_buffer_array_non_uniform_indexing;
        self.descriptor_binding_uniform_buffer_update_after_bind |=
            other.descriptor_binding_uniform_buffer_update_after_bind;
        self.descriptor_binding_sampled_image_update_after_bind |=
            other.descriptor_binding_sampled_image_update_after_bind;
        self.descriptor_binding_storage_image_update_after_bind |=
            other.descriptor_binding_storage_image_update_after_bind;
        self.descriptor_binding_storage_buffer_update_after_bind |=
            other.descriptor_binding_storage_buffer_update_after_bind;
        self.descriptor_binding_uniform_texel_buffer_update_after_bind |=
            other.descriptor_binding_uniform_texel_buffer_update_after_bind;
        self.descriptor_binding_storage_texel_buffer_update_after_bind |=
            other.descriptor_binding_storage_texel_buffer_update_after_bind;
        self.descriptor_binding_update_unused_while_pending |=
            other.descriptor_binding_update_unused_while_pending;
        self.descriptor_binding_partially_bound |= other.descriptor_binding_partially_bound;
        self.descriptor_binding_variable_descriptor_count |=
            other.descriptor_binding_variable_descriptor_count;
        self.runtime_descriptor_array |= other.runtime_descriptor_array;
        self.sampler_filter_minmax |= other.sampler_filter_minmax;
        self.scalar_block_layout |= other.scalar_block_layout;
        self.imageless_framebuffer |= other.imageless_framebuffer;
        self.uniform_buffer_standard_layout |= other.uniform_buffer_standard_layout;
        self.shader_subgroup_extended_types |= other.shader_subgroup_extended_types;
        self.separate_depth_stencil_layouts |= other.separate_depth_stencil_layouts;
        self.host_query_reset |= other.host_query_reset;
        self.timeline_semaphore |= other.timeline_semaphore;
        self.buffer_device_address |= other.buffer_device_address;
        self.buffer_device_address_capture_replay |= other.buffer_device_address_capture_replay;
        self.buffer_device_address_multi_device |= other.buffer_device_address_multi_device;
        self.vulkan_memory_model |= other.vulkan_memory_model;
        self.vulkan_memory_model_device_scope |= other.vulkan_memory_model_device_scope;
        self.vulkan_memory_model_availability_visibility_chains |=
            other.vulkan_memory_model_availability_visibility_chains;
        self.shader_output_viewport_index |= other.shader_output_viewport_index;
        self.shader_output_layer |= other.shader_output_layer;
        self.subgroup_broadcast_dynamic_id |= other.subgroup_broadcast_dynamic_id;
    }
}
impl FeatureStruct for PhysicalDeviceVulkan13Features<'_> {
    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.robust_image_access & other.robust_image_access) == other.robust_image_access
            && (self.inline_uniform_block & other.inline_uniform_block)
                == other.inline_uniform_block
            && (self.descriptor_binding_inline_uniform_block_update_after_bind
                & other.descriptor_binding_inline_uniform_block_update_after_bind)
                == other.descriptor_binding_inline_uniform_block_update_after_bind
            && (self.pipeline_creation_cache_control & other.pipeline_creation_cache_control)
                == other.pipeline_creation_cache_control
            && (self.private_data & other.private_data) == other.private_data
            && (self.shader_demote_to_helper_invocation & other.shader_demote_to_helper_invocation)
                == other.shader_demote_to_helper_invocation
            && (self.shader_terminate_invocation & other.shader_terminate_invocation)
                == other.shader_terminate_invocation
            && (self.subgroup_size_control & other.subgroup_size_control)
                == other.subgroup_size_control
            && (self.compute_full_subgroups & other.compute_full_subgroups)
                == other.compute_full_subgroups
            && (self.synchronization2 & other.synchronization2) == other.synchronization2
            && (self.texture_compression_astc_hdr & other.texture_compression_astc_hdr)
                == other.texture_compression_astc_hdr
            && (self.shader_zero_initialize_workgroup_memory
                & other.shader_zero_initialize_workgroup_memory)
                == other.shader_zero_initialize_workgroup_memory
            && (self.dynamic_rendering & other.dynamic_rendering) == other.dynamic_rendering
            && (self.shader_integer_dot_product & other.shader_integer_dot_product)
                == other.shader_integer_dot_product
            && (self.maintenance4 & other.maintenance4) == other.maintenance4
    }
    fn union_fields(&mut self, other: &Self) {
        self.robust_image_access |= other.robust_image_access;
        self.inline_uniform_block |= other.inline_uniform_block;
        self.descriptor_binding_inline_uniform_block_update_after_bind |=
            other.descriptor_binding_inline_uniform_block_update_after_bind;
        self.pipeline_creation_cache_control |= other.pipeline_creation_cache_control;
        self.private_data |= other.private_data;
        self.shader_demote_to_helper_invocation |= other.shader_demote_to_helper_invocation;
        self.shader_terminate_invocation |= other.shader_terminate_invocation;
        self.subgroup_size_control |= other.subgroup_size_control;
        self.compute_full_subgroups |= other.compute_full_subgroups;
        self.synchronization2 |= other.synchronization2;
        self.texture_compression_astc_hdr |= other.texture_compression_astc_hdr;
        self.shader_zero_initialize_workgroup_memory |=
            other.shader_zero_initialize_workgroup_memory;
        self.dynamic_rendering |= other.dynamic_rendering;
        self.shader_integer_dot_product |= other.shader_integer_dot_product;
        self.maintenance4 |= other.maintenance4;
    }
}
impl FeatureStruct for PhysicalDeviceVulkanMemoryModelFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.vulkan_memory_model & other.vulkan_memory_model) == other.vulkan_memory_model
            && (self.vulkan_memory_model_device_scope & other.vulkan_memory_model_device_scope)
                == other.vulkan_memory_model_device_scope
            && (self.vulkan_memory_model_availability_visibility_chains
                & other.vulkan_memory_model_availability_visibility_chains)
                == other.vulkan_memory_model_availability_visibility_chains
    }
    fn union_fields(&mut self, other: &Self) {
        self.vulkan_memory_model |= other.vulkan_memory_model;
        self.vulkan_memory_model_device_scope |= other.vulkan_memory_model_device_scope;
        self.vulkan_memory_model_availability_visibility_chains |=
            other.vulkan_memory_model_availability_visibility_chains;
    }
}
impl FeatureStruct for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.workgroup_memory_explicit_layout & other.workgroup_memory_explicit_layout)
            == other.workgroup_memory_explicit_layout
            && (self.workgroup_memory_explicit_layout_scalar_block_layout
                & other.workgroup_memory_explicit_layout_scalar_block_layout)
                == other.workgroup_memory_explicit_layout_scalar_block_layout
            && (self.workgroup_memory_explicit_layout8_bit_access
                & other.workgroup_memory_explicit_layout8_bit_access)
                == other.workgroup_memory_explicit_layout8_bit_access
            && (self.workgroup_memory_explicit_layout16_bit_access
                & other.workgroup_memory_explicit_layout16_bit_access)
                == other.workgroup_memory_explicit_layout16_bit_access
    }
    fn union_fields(&mut self, other: &Self) {
        self.workgroup_memory_explicit_layout |= other.workgroup_memory_explicit_layout;
        self.workgroup_memory_explicit_layout_scalar_block_layout |=
            other.workgroup_memory_explicit_layout_scalar_block_layout;
        self.workgroup_memory_explicit_layout8_bit_access |=
            other.workgroup_memory_explicit_layout8_bit_access;
        self.workgroup_memory_explicit_layout16_bit_access |=
            other.workgroup_memory_explicit_layout16_bit_access;
    }
}
impl FeatureStruct for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.ycbcr2plane444_formats & other.ycbcr2plane444_formats) == other.ycbcr2plane444_formats
    }
    fn union_fields(&mut self, other: &Self) {
        self.ycbcr2plane444_formats |= other.ycbcr2plane444_formats;
    }
}
impl FeatureStruct for PhysicalDeviceYcbcrDegammaFeaturesQCOM<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.ycbcr_degamma & other.ycbcr_degamma) == other.ycbcr_degamma
    }
    fn union_fields(&mut self, other: &Self) {
        self.ycbcr_degamma |= other.ycbcr_degamma;
    }
}
impl FeatureStruct for PhysicalDeviceYcbcrImageArraysFeaturesEXT<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.ycbcr_image_arrays & other.ycbcr_image_arrays) == other.ycbcr_image_arrays
    }
    fn union_fields(&mut self, other: &Self) {
        self.ycbcr_image_arrays |= other.ycbcr_image_arrays;
    }
}
impl FeatureStruct for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'_> {
    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES;
    fn contains_fields(&self, other: &Self) -> bool {
        (self.shader_zero_initialize_workgroup_memory
            & other.shader_zero_initialize_workgroup_memory)
            == other.shader_zero_initialize_workgroup_memory
    }
    fn union_fields(&mut self, other: &Self) {
        self.shader_zero_initialize_workgroup_memory |=
            other.shader_zero_initialize_workgroup_memory;
    }
}
#[doc = r" Set-like operations over physical-device feature structures."]
#[doc = r""]
#[doc = r" A feature is enabled when its `Bool32` field is nonzero. `contains`"]
#[doc = r" checks that every feature enabled in `other` is enabled in `self`,"]
#[doc = r" while `union` enables every feature enabled in `other`."]
pub trait PhysicalDeviceFeaturesExt {
    #[doc = r" Returns true if every feature enabled in `other` is also enabled in `self`."]
    #[doc = r""]
    #[doc = r" For `PhysicalDeviceFeatures2`, extension structures present in"]
    #[doc = r" `other` but absent from `self` are treated as all-disabled."]
    #[doc = r""]
    #[doc = r" # Performance"]
    #[doc = r""]
    #[doc = r" For `PhysicalDeviceFeatures` and individual feature structures this is"]
    #[doc = r" O(number of feature fields). For `PhysicalDeviceFeatures2` it walks both"]
    #[doc = r" `pNext` chains, and each node of `other`'s chain is looked up by"]
    #[doc = r" linear scan in `self`'s chain, i.e. O(n * m) in the chain lengths of"]
    #[doc = r" `self` and `other`. Avoid calling it in hot paths with long chains."]
    fn contains(&self, other: &Self) -> bool;
    #[doc = r" Enables in `self` every feature enabled in `other`."]
    #[doc = r""]
    #[doc = r" For `PhysicalDeviceFeatures2`, missing extension structures are"]
    #[doc = r" appended to `self`'s chain. Appended structures are intentionally"]
    #[doc = r" leaked because the chain may outlive this call."]
    fn union(&mut self, other: &Self);
    #[doc = r" Convenience: `self.union(other); self` (requires `Copy` and `Sized`)."]
    fn unioned(mut self, other: &Self) -> Self
    where
        Self: Sized + Copy,
    {
        self.union(other);
        self
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceFeatures {
    fn contains(&self, other: &Self) -> bool {
        (self.robust_buffer_access & other.robust_buffer_access) == other.robust_buffer_access
            && (self.full_draw_index_uint32 & other.full_draw_index_uint32)
                == other.full_draw_index_uint32
            && (self.image_cube_array & other.image_cube_array) == other.image_cube_array
            && (self.independent_blend & other.independent_blend) == other.independent_blend
            && (self.geometry_shader & other.geometry_shader) == other.geometry_shader
            && (self.tessellation_shader & other.tessellation_shader) == other.tessellation_shader
            && (self.sample_rate_shading & other.sample_rate_shading) == other.sample_rate_shading
            && (self.dual_src_blend & other.dual_src_blend) == other.dual_src_blend
            && (self.logic_op & other.logic_op) == other.logic_op
            && (self.multi_draw_indirect & other.multi_draw_indirect) == other.multi_draw_indirect
            && (self.draw_indirect_first_instance & other.draw_indirect_first_instance)
                == other.draw_indirect_first_instance
            && (self.depth_clamp & other.depth_clamp) == other.depth_clamp
            && (self.depth_bias_clamp & other.depth_bias_clamp) == other.depth_bias_clamp
            && (self.fill_mode_non_solid & other.fill_mode_non_solid) == other.fill_mode_non_solid
            && (self.depth_bounds & other.depth_bounds) == other.depth_bounds
            && (self.wide_lines & other.wide_lines) == other.wide_lines
            && (self.large_points & other.large_points) == other.large_points
            && (self.alpha_to_one & other.alpha_to_one) == other.alpha_to_one
            && (self.multi_viewport & other.multi_viewport) == other.multi_viewport
            && (self.sampler_anisotropy & other.sampler_anisotropy) == other.sampler_anisotropy
            && (self.texture_compression_etc2 & other.texture_compression_etc2)
                == other.texture_compression_etc2
            && (self.texture_compression_astc_ldr & other.texture_compression_astc_ldr)
                == other.texture_compression_astc_ldr
            && (self.texture_compression_bc & other.texture_compression_bc)
                == other.texture_compression_bc
            && (self.occlusion_query_precise & other.occlusion_query_precise)
                == other.occlusion_query_precise
            && (self.pipeline_statistics_query & other.pipeline_statistics_query)
                == other.pipeline_statistics_query
            && (self.vertex_pipeline_stores_and_atomics & other.vertex_pipeline_stores_and_atomics)
                == other.vertex_pipeline_stores_and_atomics
            && (self.fragment_stores_and_atomics & other.fragment_stores_and_atomics)
                == other.fragment_stores_and_atomics
            && (self.shader_tessellation_and_geometry_point_size
                & other.shader_tessellation_and_geometry_point_size)
                == other.shader_tessellation_and_geometry_point_size
            && (self.shader_image_gather_extended & other.shader_image_gather_extended)
                == other.shader_image_gather_extended
            && (self.shader_storage_image_extended_formats
                & other.shader_storage_image_extended_formats)
                == other.shader_storage_image_extended_formats
            && (self.shader_storage_image_multisample & other.shader_storage_image_multisample)
                == other.shader_storage_image_multisample
            && (self.shader_storage_image_read_without_format
                & other.shader_storage_image_read_without_format)
                == other.shader_storage_image_read_without_format
            && (self.shader_storage_image_write_without_format
                & other.shader_storage_image_write_without_format)
                == other.shader_storage_image_write_without_format
            && (self.shader_uniform_buffer_array_dynamic_indexing
                & other.shader_uniform_buffer_array_dynamic_indexing)
                == other.shader_uniform_buffer_array_dynamic_indexing
            && (self.shader_sampled_image_array_dynamic_indexing
                & other.shader_sampled_image_array_dynamic_indexing)
                == other.shader_sampled_image_array_dynamic_indexing
            && (self.shader_storage_buffer_array_dynamic_indexing
                & other.shader_storage_buffer_array_dynamic_indexing)
                == other.shader_storage_buffer_array_dynamic_indexing
            && (self.shader_storage_image_array_dynamic_indexing
                & other.shader_storage_image_array_dynamic_indexing)
                == other.shader_storage_image_array_dynamic_indexing
            && (self.shader_clip_distance & other.shader_clip_distance)
                == other.shader_clip_distance
            && (self.shader_cull_distance & other.shader_cull_distance)
                == other.shader_cull_distance
            && (self.shader_float64 & other.shader_float64) == other.shader_float64
            && (self.shader_int64 & other.shader_int64) == other.shader_int64
            && (self.shader_int16 & other.shader_int16) == other.shader_int16
            && (self.shader_resource_residency & other.shader_resource_residency)
                == other.shader_resource_residency
            && (self.shader_resource_min_lod & other.shader_resource_min_lod)
                == other.shader_resource_min_lod
            && (self.sparse_binding & other.sparse_binding) == other.sparse_binding
            && (self.sparse_residency_buffer & other.sparse_residency_buffer)
                == other.sparse_residency_buffer
            && (self.sparse_residency_image2_d & other.sparse_residency_image2_d)
                == other.sparse_residency_image2_d
            && (self.sparse_residency_image3_d & other.sparse_residency_image3_d)
                == other.sparse_residency_image3_d
            && (self.sparse_residency2_samples & other.sparse_residency2_samples)
                == other.sparse_residency2_samples
            && (self.sparse_residency4_samples & other.sparse_residency4_samples)
                == other.sparse_residency4_samples
            && (self.sparse_residency8_samples & other.sparse_residency8_samples)
                == other.sparse_residency8_samples
            && (self.sparse_residency16_samples & other.sparse_residency16_samples)
                == other.sparse_residency16_samples
            && (self.sparse_residency_aliased & other.sparse_residency_aliased)
                == other.sparse_residency_aliased
            && (self.variable_multisample_rate & other.variable_multisample_rate)
                == other.variable_multisample_rate
            && (self.inherited_queries & other.inherited_queries) == other.inherited_queries
    }
    fn union(&mut self, other: &Self) {
        self.robust_buffer_access |= other.robust_buffer_access;
        self.full_draw_index_uint32 |= other.full_draw_index_uint32;
        self.image_cube_array |= other.image_cube_array;
        self.independent_blend |= other.independent_blend;
        self.geometry_shader |= other.geometry_shader;
        self.tessellation_shader |= other.tessellation_shader;
        self.sample_rate_shading |= other.sample_rate_shading;
        self.dual_src_blend |= other.dual_src_blend;
        self.logic_op |= other.logic_op;
        self.multi_draw_indirect |= other.multi_draw_indirect;
        self.draw_indirect_first_instance |= other.draw_indirect_first_instance;
        self.depth_clamp |= other.depth_clamp;
        self.depth_bias_clamp |= other.depth_bias_clamp;
        self.fill_mode_non_solid |= other.fill_mode_non_solid;
        self.depth_bounds |= other.depth_bounds;
        self.wide_lines |= other.wide_lines;
        self.large_points |= other.large_points;
        self.alpha_to_one |= other.alpha_to_one;
        self.multi_viewport |= other.multi_viewport;
        self.sampler_anisotropy |= other.sampler_anisotropy;
        self.texture_compression_etc2 |= other.texture_compression_etc2;
        self.texture_compression_astc_ldr |= other.texture_compression_astc_ldr;
        self.texture_compression_bc |= other.texture_compression_bc;
        self.occlusion_query_precise |= other.occlusion_query_precise;
        self.pipeline_statistics_query |= other.pipeline_statistics_query;
        self.vertex_pipeline_stores_and_atomics |= other.vertex_pipeline_stores_and_atomics;
        self.fragment_stores_and_atomics |= other.fragment_stores_and_atomics;
        self.shader_tessellation_and_geometry_point_size |=
            other.shader_tessellation_and_geometry_point_size;
        self.shader_image_gather_extended |= other.shader_image_gather_extended;
        self.shader_storage_image_extended_formats |= other.shader_storage_image_extended_formats;
        self.shader_storage_image_multisample |= other.shader_storage_image_multisample;
        self.shader_storage_image_read_without_format |=
            other.shader_storage_image_read_without_format;
        self.shader_storage_image_write_without_format |=
            other.shader_storage_image_write_without_format;
        self.shader_uniform_buffer_array_dynamic_indexing |=
            other.shader_uniform_buffer_array_dynamic_indexing;
        self.shader_sampled_image_array_dynamic_indexing |=
            other.shader_sampled_image_array_dynamic_indexing;
        self.shader_storage_buffer_array_dynamic_indexing |=
            other.shader_storage_buffer_array_dynamic_indexing;
        self.shader_storage_image_array_dynamic_indexing |=
            other.shader_storage_image_array_dynamic_indexing;
        self.shader_clip_distance |= other.shader_clip_distance;
        self.shader_cull_distance |= other.shader_cull_distance;
        self.shader_float64 |= other.shader_float64;
        self.shader_int64 |= other.shader_int64;
        self.shader_int16 |= other.shader_int16;
        self.shader_resource_residency |= other.shader_resource_residency;
        self.shader_resource_min_lod |= other.shader_resource_min_lod;
        self.sparse_binding |= other.sparse_binding;
        self.sparse_residency_buffer |= other.sparse_residency_buffer;
        self.sparse_residency_image2_d |= other.sparse_residency_image2_d;
        self.sparse_residency_image3_d |= other.sparse_residency_image3_d;
        self.sparse_residency2_samples |= other.sparse_residency2_samples;
        self.sparse_residency4_samples |= other.sparse_residency4_samples;
        self.sparse_residency8_samples |= other.sparse_residency8_samples;
        self.sparse_residency16_samples |= other.sparse_residency16_samples;
        self.sparse_residency_aliased |= other.sparse_residency_aliased;
        self.variable_multisample_rate |= other.variable_multisample_rate;
        self.inherited_queries |= other.inherited_queries;
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevice16BitStorageFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevice4444FormatsFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevice8BitStorageFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceASTCDecodeFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceAccelerationStructureFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceAddressBindingReportFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceAmigoProfilingFeaturesSEC<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceBorderColorSwizzleFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceBufferDeviceAddressFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceBufferDeviceAddressFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceCoherentMemoryFeaturesAMD<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceColorWriteEnableFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceConditionalRenderingFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceCooperativeMatrixFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceCooperativeMatrixFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceCopyMemoryIndirectFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceCornerSampledImageFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceCoverageReductionModeFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceCubicClampFeaturesQCOM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceCubicWeightsFeaturesQCOM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceCudaKernelLaunchFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceCustomBorderColorFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDepthBiasControlFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDepthClampZeroOneFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDepthClipControlFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDepthClipEnableFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDescriptorBufferFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDescriptorIndexingFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDeviceMemoryReportFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDiagnosticsConfigFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDisplacementMicromapFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDynamicRenderingFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceExclusiveScissorFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceExtendedDynamicState2FeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceExtendedDynamicState3FeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceExtendedDynamicStateFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceExternalFormatResolveFeaturesANDROID<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceFaultFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceFragmentDensityMap2FeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceFragmentDensityMapFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceFragmentShadingRateFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceFrameBoundaryFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceGlobalPriorityQueryFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceHostImageCopyFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceHostQueryResetFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceImageCompressionControlFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceImageProcessing2FeaturesQCOM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceImageProcessingFeaturesQCOM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceImageRobustnessFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceImageViewMinLodFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceImagelessFramebufferFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceIndexTypeUint8FeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceInheritedViewportScissorFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceInlineUniformBlockFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceInvocationMaskFeaturesHUAWEI<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceLegacyDitheringFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceLineRasterizationFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceLinearColorAttachmentFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMaintenance4Features<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMaintenance5FeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMaintenance6FeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMapMemoryPlacedFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMemoryDecompressionFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMemoryPriorityFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMeshShaderFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMeshShaderFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMultiDrawFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMultiviewFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceNestedCommandBufferFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceOpacityMicromapFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceOpticalFlowFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePerStageDescriptorSetFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePerformanceQueryFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePipelineCreationCacheControlFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePipelinePropertiesFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePipelineProtectedAccessFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePipelineRobustnessFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePortabilitySubsetFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePresentBarrierFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePresentIdFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePresentWaitFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDevicePrivateDataFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceProtectedMemoryFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceProvokingVertexFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRawAccessChainsFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRayQueryFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRayTracingMaintenance1FeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRayTracingMotionBlurFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRayTracingPipelineFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRayTracingPositionFetchFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRayTracingValidationFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRenderPassStripedFeaturesARM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceRobustness2FeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceSamplerYcbcrConversionFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceScalarBlockLayoutFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceSchedulingControlsFeaturesARM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderAtomicFloatFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderAtomicInt64Features<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderClockFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderDrawParametersFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderEnqueueFeaturesAMDX<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderExpectAssumeFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderFloat16Int8Features<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderFloatControls2FeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderImageFootprintFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderIntegerDotProductFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderModuleIdentifierFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderObjectFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderQuadControlFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderSMBuiltinsFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderSubgroupRotateFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderTerminateInvocationFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShaderTileImageFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceShadingRateImageFeaturesNV<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceSubgroupSizeControlFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceSubpassShadingFeaturesHUAWEI<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceSwapchainMaintenance1FeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceSynchronization2Features<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceTextureCompressionASTCHDRFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceTilePropertiesFeaturesQCOM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceTimelineSemaphoreFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceTransformFeedbackFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceUniformBufferStandardLayoutFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceVariablePointersFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceVertexAttributeDivisorFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceVideoMaintenance1FeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceVulkan11Features<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceVulkan12Features<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceVulkan13Features<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceVulkanMemoryModelFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceYcbcrDegammaFeaturesQCOM<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceYcbcrImageArraysFeaturesEXT<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl PhysicalDeviceFeaturesExt for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'_> {
    fn contains(&self, other: &Self) -> bool {
        self.contains_fields(other)
    }
    fn union(&mut self, other: &Self) {
        self.union_fields(other)
    }
}
impl<'a> PhysicalDeviceFeaturesExt for PhysicalDeviceFeatures2<'a> {
    fn contains(&self, other: &Self) -> bool {
        self.features.contains(&other.features)
            && unsafe { chain_contains(self.p_next, other.p_next) }
    }
    fn union(&mut self, other: &Self) {
        self.features.union(&other.features);
        unsafe { chain_union(&mut self.p_next, other.p_next) };
    }
}
#[repr(C)]
struct Header {
    s_type: StructureType,
    p_next: *mut Header,
}
macro_rules ! for_each_feature { ($ next : expr , $ f : path , $ context : expr) => { { let mut next = ($ next) . cast :: < Header > () ; while ! next . is_null () { match unsafe { (* next) . s_type } { < PhysicalDevice16BitStorageFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevice16BitStorageFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDevice4444FormatsFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevice4444FormatsFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDevice8BitStorageFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevice8BitStorageFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceASTCDecodeFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceASTCDecodeFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceAccelerationStructureFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceAccelerationStructureFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceAddressBindingReportFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceAddressBindingReportFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceAmigoProfilingFeaturesSEC < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceAmigoProfilingFeaturesSEC < 'static >> () , $ context) } { break ; } } < PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceBlendOperationAdvancedFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceBlendOperationAdvancedFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceBorderColorSwizzleFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceBorderColorSwizzleFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceBufferDeviceAddressFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceBufferDeviceAddressFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceBufferDeviceAddressFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceBufferDeviceAddressFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceClusterCullingShaderFeaturesHUAWEI < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceClusterCullingShaderFeaturesHUAWEI < 'static >> () , $ context) } { break ; } } < PhysicalDeviceCoherentMemoryFeaturesAMD < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceCoherentMemoryFeaturesAMD < 'static >> () , $ context) } { break ; } } < PhysicalDeviceColorWriteEnableFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceColorWriteEnableFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceConditionalRenderingFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceConditionalRenderingFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceCooperativeMatrixFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceCooperativeMatrixFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceCooperativeMatrixFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceCooperativeMatrixFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceCopyMemoryIndirectFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceCopyMemoryIndirectFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceCornerSampledImageFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceCornerSampledImageFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceCoverageReductionModeFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceCoverageReductionModeFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceCubicClampFeaturesQCOM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceCubicClampFeaturesQCOM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceCubicWeightsFeaturesQCOM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceCubicWeightsFeaturesQCOM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceCudaKernelLaunchFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceCudaKernelLaunchFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceCustomBorderColorFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceCustomBorderColorFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDepthBiasControlFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDepthBiasControlFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDepthClampZeroOneFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDepthClampZeroOneFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDepthClipControlFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDepthClipControlFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDepthClipEnableFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDepthClipEnableFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDescriptorBufferFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDescriptorBufferFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDescriptorIndexingFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDescriptorIndexingFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDescriptorPoolOverallocationFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDescriptorPoolOverallocationFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDeviceGeneratedCommandsFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDeviceGeneratedCommandsFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDeviceMemoryReportFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDeviceMemoryReportFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDiagnosticsConfigFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDiagnosticsConfigFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDisplacementMicromapFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDisplacementMicromapFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDynamicRenderingFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDynamicRenderingFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceExclusiveScissorFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceExclusiveScissorFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceExtendedDynamicState2FeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceExtendedDynamicState2FeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceExtendedDynamicState3FeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceExtendedDynamicState3FeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceExtendedDynamicStateFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceExtendedDynamicStateFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceExternalFormatResolveFeaturesANDROID < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceExternalFormatResolveFeaturesANDROID < 'static >> () , $ context) } { break ; } } < PhysicalDeviceExternalMemoryRDMAFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceExternalMemoryRDMAFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX < 'static >> () , $ context) } { break ; } } < PhysicalDeviceFaultFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceFaultFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceFragmentDensityMap2FeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceFragmentDensityMap2FeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceFragmentDensityMapFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceFragmentDensityMapFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceFragmentShaderBarycentricFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceFragmentShaderBarycentricFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceFragmentShaderInterlockFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceFragmentShaderInterlockFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceFragmentShadingRateEnumsFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceFragmentShadingRateEnumsFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceFragmentShadingRateFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceFragmentShadingRateFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceFrameBoundaryFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceFrameBoundaryFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceGlobalPriorityQueryFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceGlobalPriorityQueryFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceHostImageCopyFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceHostImageCopyFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceHostQueryResetFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceHostQueryResetFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceImage2DViewOf3DFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceImage2DViewOf3DFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceImageCompressionControlFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceImageCompressionControlFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceImageProcessing2FeaturesQCOM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceImageProcessing2FeaturesQCOM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceImageProcessingFeaturesQCOM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceImageProcessingFeaturesQCOM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceImageRobustnessFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceImageRobustnessFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceImageSlicedViewOf3DFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceImageSlicedViewOf3DFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceImageViewMinLodFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceImageViewMinLodFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceImagelessFramebufferFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceImagelessFramebufferFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceIndexTypeUint8FeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceIndexTypeUint8FeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceInheritedViewportScissorFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceInheritedViewportScissorFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceInlineUniformBlockFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceInlineUniformBlockFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceInvocationMaskFeaturesHUAWEI < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceInvocationMaskFeaturesHUAWEI < 'static >> () , $ context) } { break ; } } < PhysicalDeviceLegacyDitheringFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceLegacyDitheringFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceLineRasterizationFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceLineRasterizationFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceLinearColorAttachmentFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceLinearColorAttachmentFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMaintenance4Features < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMaintenance4Features < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMaintenance5FeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMaintenance5FeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMaintenance6FeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMaintenance6FeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMapMemoryPlacedFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMapMemoryPlacedFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMemoryDecompressionFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMemoryDecompressionFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMemoryPriorityFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMemoryPriorityFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMeshShaderFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMeshShaderFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMeshShaderFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMeshShaderFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMultiDrawFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMultiDrawFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMultiviewFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMultiviewFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceMutableDescriptorTypeFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceMutableDescriptorTypeFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceNestedCommandBufferFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceNestedCommandBufferFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceNonSeamlessCubeMapFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceNonSeamlessCubeMapFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceOpacityMicromapFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceOpacityMicromapFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceOpticalFlowFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceOpticalFlowFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDevicePerStageDescriptorSetFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePerStageDescriptorSetFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDevicePerformanceQueryFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePerformanceQueryFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDevicePipelineCreationCacheControlFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePipelineCreationCacheControlFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDevicePipelineExecutablePropertiesFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePipelineExecutablePropertiesFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDevicePipelinePropertiesFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePipelinePropertiesFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDevicePipelineProtectedAccessFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePipelineProtectedAccessFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDevicePipelineRobustnessFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePipelineRobustnessFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDevicePortabilitySubsetFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePortabilitySubsetFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDevicePresentBarrierFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePresentBarrierFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDevicePresentIdFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePresentIdFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDevicePresentWaitFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePresentWaitFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDevicePrivateDataFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDevicePrivateDataFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceProtectedMemoryFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceProtectedMemoryFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceProvokingVertexFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceProvokingVertexFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRGBA10X6FormatsFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRGBA10X6FormatsFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRawAccessChainsFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRawAccessChainsFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRayQueryFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRayQueryFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRayTracingInvocationReorderFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRayTracingInvocationReorderFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRayTracingMaintenance1FeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRayTracingMaintenance1FeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRayTracingMotionBlurFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRayTracingMotionBlurFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRayTracingPipelineFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRayTracingPipelineFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRayTracingPositionFetchFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRayTracingPositionFetchFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRayTracingValidationFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRayTracingValidationFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRelaxedLineRasterizationFeaturesIMG < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRelaxedLineRasterizationFeaturesIMG < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRenderPassStripedFeaturesARM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRenderPassStripedFeaturesARM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRepresentativeFragmentTestFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRepresentativeFragmentTestFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceRobustness2FeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceRobustness2FeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceSamplerYcbcrConversionFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceSamplerYcbcrConversionFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceScalarBlockLayoutFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceScalarBlockLayoutFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceSchedulingControlsFeaturesARM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceSchedulingControlsFeaturesARM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceSeparateDepthStencilLayoutsFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceSeparateDepthStencilLayoutsFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderAtomicFloat2FeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderAtomicFloat2FeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderAtomicFloatFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderAtomicFloatFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderAtomicInt64Features < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderAtomicInt64Features < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderClockFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderClockFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderCoreBuiltinsFeaturesARM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderCoreBuiltinsFeaturesARM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderDemoteToHelperInvocationFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderDemoteToHelperInvocationFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderDrawParametersFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderDrawParametersFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderEnqueueFeaturesAMDX < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderEnqueueFeaturesAMDX < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderExpectAssumeFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderExpectAssumeFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderFloat16Int8Features < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderFloat16Int8Features < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderFloatControls2FeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderFloatControls2FeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderImageAtomicInt64FeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderImageAtomicInt64FeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderImageFootprintFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderImageFootprintFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderIntegerDotProductFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderIntegerDotProductFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderModuleIdentifierFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderModuleIdentifierFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderObjectFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderObjectFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderQuadControlFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderQuadControlFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderSMBuiltinsFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderSMBuiltinsFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderSubgroupExtendedTypesFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderSubgroupExtendedTypesFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderSubgroupRotateFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderSubgroupRotateFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderTerminateInvocationFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderTerminateInvocationFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShaderTileImageFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShaderTileImageFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceShadingRateImageFeaturesNV < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceShadingRateImageFeaturesNV < 'static >> () , $ context) } { break ; } } < PhysicalDeviceSubgroupSizeControlFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceSubgroupSizeControlFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceSubpassMergeFeedbackFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceSubpassMergeFeedbackFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceSubpassShadingFeaturesHUAWEI < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceSubpassShadingFeaturesHUAWEI < 'static >> () , $ context) } { break ; } } < PhysicalDeviceSwapchainMaintenance1FeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceSwapchainMaintenance1FeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceSynchronization2Features < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceSynchronization2Features < 'static >> () , $ context) } { break ; } } < PhysicalDeviceTexelBufferAlignmentFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceTexelBufferAlignmentFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceTextureCompressionASTCHDRFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceTextureCompressionASTCHDRFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceTilePropertiesFeaturesQCOM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceTilePropertiesFeaturesQCOM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceTimelineSemaphoreFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceTimelineSemaphoreFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceTransformFeedbackFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceTransformFeedbackFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceUniformBufferStandardLayoutFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceUniformBufferStandardLayoutFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceVariablePointersFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceVariablePointersFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceVertexAttributeDivisorFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceVertexAttributeDivisorFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceVertexInputDynamicStateFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceVertexInputDynamicStateFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceVideoMaintenance1FeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceVideoMaintenance1FeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceVulkan11Features < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceVulkan11Features < 'static >> () , $ context) } { break ; } } < PhysicalDeviceVulkan12Features < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceVulkan12Features < 'static >> () , $ context) } { break ; } } < PhysicalDeviceVulkan13Features < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceVulkan13Features < 'static >> () , $ context) } { break ; } } < PhysicalDeviceVulkanMemoryModelFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceVulkanMemoryModelFeatures < 'static >> () , $ context) } { break ; } } < PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR < 'static >> () , $ context) } { break ; } } < PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceYcbcrDegammaFeaturesQCOM < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceYcbcrDegammaFeaturesQCOM < 'static >> () , $ context) } { break ; } } < PhysicalDeviceYcbcrImageArraysFeaturesEXT < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceYcbcrImageArraysFeaturesEXT < 'static >> () , $ context) } { break ; } } < PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures < 'static > as FeatureStruct > :: STRUCTURE_TYPE => { if ! unsafe { $ f (& * next . cast :: < PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures < 'static >> () , $ context) } { break ; } } _ => { } } next = unsafe { (* next) . p_next } ; } } } ; }
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn chain_contains(self_next: *const c_void, other_next: *const c_void) -> bool {
    let mut valid = true;
    for_each_feature!(
        other_next,
        check_feature_node,
        (self_next, &mut valid as *mut bool)
    );
    valid
}
unsafe fn check_feature_node<T: FeatureStruct + Default>(
    node: &T,
    context: (*const c_void, *mut bool),
) -> bool {
    let (self_next, valid) = context;
    if unsafe { !contains_feature_node(node, self_next) } {
        unsafe { *valid = false };
        return false;
    }
    true
}
unsafe fn contains_feature_node<T: FeatureStruct + Default>(
    node: &T,
    self_next: *const c_void,
) -> bool {
    let mut current = self_next.cast::<Header>();
    while !current.is_null() {
        let header = unsafe { &*current };
        if header.s_type == T::STRUCTURE_TYPE {
            return unsafe { (&*current.cast::<T>()).contains_fields(node) };
        }
        current = header.p_next.cast_const();
    }
    T::default().contains_fields(node)
}
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn chain_union(self_next: &mut *mut c_void, other_next: *const c_void) {
    for_each_feature!(
        other_next,
        union_feature_node,
        self_next as *mut *mut c_void
    );
}
unsafe fn union_feature_node<T: FeatureStruct>(node: &T, self_next: *mut *mut c_void) -> bool {
    let root = unsafe { &mut *self_next };
    let mut current = (*root).cast::<Header>();
    while !current.is_null() {
        let header = unsafe { &mut *current };
        if header.s_type == T::STRUCTURE_TYPE {
            unsafe { (&mut *current.cast::<T>()).union_fields(node) };
            return true;
        }
        current = header.p_next;
    }
    let leaked = Box::into_raw(Box::new(*node));
    unsafe { (*leaked.cast::<Header>()).p_next = ptr::null_mut() };
    let appended = leaked.cast::<Header>();
    if (*root).is_null() {
        *root = appended.cast::<c_void>();
    } else {
        let mut tail = (*root).cast::<Header>();
        while !unsafe { (*tail).p_next }.is_null() {
            tail = unsafe { (*tail).p_next };
        }
        unsafe { (*tail).p_next = appended };
    }
    true
}
