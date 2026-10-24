extern crate std;

#[test]
fn test_contains() {
    let mut features2 = ash::vk::PhysicalDeviceFeatures2::default();

    let mut storage_features16 = ash::vk::PhysicalDevice16BitStorageFeatures {
        storage_buffer16_bit_access: ash::vk::TRUE,
        ..Default::default()
    };

    let mut storage_features8 = ash::vk::PhysicalDevice8BitStorageFeatures {
        storage_push_constant8: ash::vk::TRUE,
        ..Default::default()
    };

    features2 = features2.push_next(&mut storage_features16);
    features2 = features2.push_next(&mut storage_features8);

    let mut other = ash::vk::PhysicalDeviceFeatures2::default();

    let mut other_features16 = ash::vk::PhysicalDevice16BitStorageFeatures {
        storage_buffer16_bit_access: ash::vk::TRUE,
        ..Default::default()
    };

    let mut other_features8 = ash::vk::PhysicalDevice8BitStorageFeatures {
        storage_push_constant8: ash::vk::TRUE,
        ..Default::default()
    };

    other = other.push_next(&mut other_features8);
    other = other.push_next(&mut other_features16);

    use crate::features::PhysicalDeviceFeaturesExt;
    assert!(features2.contains(&other));
}
