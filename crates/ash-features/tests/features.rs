use ash::vk::{PhysicalDeviceFeatures, PhysicalDeviceFeatures2, PhysicalDeviceVulkan11Features};
use ash_features::PhysicalDeviceFeaturesExt;

#[test]
fn base_features_contain_and_union() {
    let mut available = PhysicalDeviceFeatures {
        robust_buffer_access: 1,
        geometry_shader: 1,
        ..Default::default()
    };
    let requested = PhysicalDeviceFeatures {
        robust_buffer_access: 1,
        ..Default::default()
    };

    assert!(available.contains(&requested));
    assert!(!requested.contains(&available));

    let additional = PhysicalDeviceFeatures {
        tessellation_shader: 1,
        ..Default::default()
    };
    available.union(&additional);
    assert!(available.contains(&additional));
}

#[test]
fn feature2_chains_compare_and_append_missing_nodes() {
    let mut available_extension = PhysicalDeviceVulkan11Features {
        storage_buffer16_bit_access: 1,
        ..Default::default()
    };
    let mut available = PhysicalDeviceFeatures2::default();
    available.p_next = (&mut available_extension as *mut PhysicalDeviceVulkan11Features<'_>).cast();

    let mut requested_extension = PhysicalDeviceVulkan11Features {
        storage_buffer16_bit_access: 1,
        uniform_and_storage_buffer16_bit_access: 1,
        ..Default::default()
    };
    let mut requested = PhysicalDeviceFeatures2::default();
    requested.p_next = (&mut requested_extension as *mut PhysicalDeviceVulkan11Features<'_>).cast();

    assert!(!available.contains(&requested));
    available.union(&requested);
    assert!(available.contains(&requested));

    let mut destination = PhysicalDeviceFeatures2::default();
    destination.union(&requested);
    assert!(destination.contains(&requested));
}
