use ash::vk::{self, DeviceSize, Format};

use crate::{Component, ComponentName, FormatExt, NumericFormat};

/// Convenience utilities for formats implementing [`FormatExt`].
pub trait FormatHelpers: FormatExt {
    /// Returns the byte pitch of a row of `width` texels.
    ///
    /// This is not meaningful for multi-planar formats because their `block_size`
    /// sums all planes; use [`plane_row_pitch`](Self::plane_row_pitch) instead.
    /// For `Format::UNDEFINED`, this returns `0`.
    ///
    /// # Panics
    ///
    /// Panics for multi-planar formats; use `plane_row_pitch` instead.
    fn row_pitch(self, width: u32) -> DeviceSize
    where
        Self: Copy,
    {
        assert!(
            self.planes().is_empty(),
            "row_pitch is not meaningful for multi-planar formats; use plane_row_pitch"
        );
        u64::from(width.div_ceil(u32::from(self.block_extent().0))) * self.block_size()
    }

    /// Returns the byte pitch for a row of the selected plane's `width` texels.
    ///
    /// Plane widths are subsampled by the plane's width divisor before computing
    /// the compatible format's block pitch. For `Format::UNDEFINED`, this panics
    /// because it has no planes.
    ///
    /// # Panics
    ///
    /// Panics for single-plane formats (use [`row_pitch`](Self::row_pitch)) or
    /// when `plane_index` is out of range. Single-plane formats should use
    /// [`layer_size`](Self::layer_size).
    fn plane_row_pitch(self, plane_index: u8, width: u32) -> DeviceSize
    where
        Self: Copy,
    {
        let plane = self.planes().get(usize::from(plane_index)).expect(
            "plane_row_pitch plane_index is out of range; use row_pitch for single-plane formats",
        );
        let plane_width = width.div_ceil(u32::from(plane.width_divisor));
        let compatible = plane.compatible_format;
        u64::from(plane_width.div_ceil(u32::from(compatible.block_extent().0)))
            * compatible.block_size()
    }

    /// Returns the byte size of one layer for this extent.
    ///
    /// For multi-planar formats the sizes of all subsampled planes are summed.
    /// For `Format::UNDEFINED`, this returns `0`.
    fn layer_size(self, extent: [u32; 3]) -> DeviceSize
    where
        Self: Copy,
    {
        if self.planes().is_empty() {
            extent_block_size(extent, self.block_extent(), self.block_size())
        } else {
            self.planes()
                .iter()
                .map(|plane| {
                    let plane_extent = [
                        extent[0].div_ceil(u32::from(plane.width_divisor)),
                        extent[1].div_ceil(u32::from(plane.height_divisor)),
                        extent[2],
                    ];
                    let compatible = plane.compatible_format;
                    extent_block_size(
                        plane_extent,
                        compatible.block_extent(),
                        compatible.block_size(),
                    )
                })
                .sum()
        }
    }

    /// Returns the byte size of a single plane for this extent.
    ///
    /// For `Format::UNDEFINED`, this panics because it has no planes.
    ///
    /// # Panics
    ///
    /// Panics for single-plane formats or when `plane_index` is out of range.
    fn plane_layer_size(self, plane_index: u8, extent: [u32; 3]) -> DeviceSize
    where
        Self: Copy,
    {
        let plane = self.planes().get(usize::from(plane_index)).expect(
            "plane_layer_size plane_index is out of range; use layer_size for single-plane formats",
        );
        let plane_extent = [
            extent[0].div_ceil(u32::from(plane.width_divisor)),
            extent[1].div_ceil(u32::from(plane.height_divisor)),
            extent[2],
        ];
        let compatible = plane.compatible_format;
        extent_block_size(
            plane_extent,
            compatible.block_extent(),
            compatible.block_size(),
        )
    }

    /// Creates a Vulkan clear color value suitable for color images.
    ///
    /// Depth/stencil clears use [`vk::ClearDepthStencilValue`] instead. For
    /// `Format::UNDEFINED` and depth/stencil formats, this returns `Float32`.
    /// Assumes that `color` is in the range `[0.0, 1.0]`.
    fn clear_value(self, color: [f32; 4]) -> vk::ClearColorValue
    where
        Self: Copy,
    {
        let components = self.components();
        let nf = components
            .iter()
            .map(|c| c.numeric_format)
            .next()
            .unwrap_or(NumericFormat::UNorm);

        struct ColorValues([f32; 4]);

        impl core::ops::Index<ComponentName> for ColorValues {
            type Output = f32;

            fn index(&self, index: ComponentName) -> &Self::Output {
                match index {
                    ComponentName::R => &self.0[0],
                    ComponentName::G => &self.0[1],
                    ComponentName::B => &self.0[2],
                    ComponentName::A => &self.0[3],
                    _ => panic!("Invalid component name for color value"),
                }
            }
        }

        let color = ColorValues(color);

        match nf {
            NumericFormat::UInt => {
                let mut components = components.iter().map(|c| {
                    let value = color[c.name];
                    let normalized = value * ((1i32 << c.bits) - 1) as f32;
                    normalized as u32
                });

                vk::ClearColorValue {
                    uint32: core::array::from_fn(|_| components.next().unwrap_or(0)),
                }
            }
            NumericFormat::SInt => {
                let mut components = components.iter().map(|c| {
                    let value = color[c.name];
                    let normalized = value * ((1i32 << c.bits) - 1) as f32;
                    normalized as i32 - 128
                });

                vk::ClearColorValue {
                    int32: core::array::from_fn(|_| components.next().unwrap_or(0)),
                }
            }
            NumericFormat::UNorm => {
                let mut components = components.iter().map(|c| color[c.name]);

                vk::ClearColorValue {
                    float32: core::array::from_fn(|_| components.next().unwrap_or(0.0)),
                }
            }
            NumericFormat::SNorm => {
                let mut components = components.iter().map(|c| color[c.name] * 2.0 - 1.0);

                vk::ClearColorValue {
                    float32: core::array::from_fn(|_| components.next().unwrap_or(0.0)),
                }
            }
            NumericFormat::SRgb => {
                let mut components = components.iter().map(|c| color[c.name]);

                vk::ClearColorValue {
                    float32: core::array::from_fn(|_| components.next().unwrap_or(0.0)),
                }
            }
            NumericFormat::UScaled => {
                let mut components = components.iter().map(|c| {
                    let value = color[c.name];
                    value * ((1u32 << c.bits) - 1) as f32
                });

                vk::ClearColorValue {
                    float32: core::array::from_fn(|_| components.next().unwrap_or(0.0)),
                }
            }
            NumericFormat::SScaled => {
                let mut components = components.iter().map(|c| {
                    let value = color[c.name];
                    value * ((1i32 << (c.bits - 1)) - 1) as f32
                });

                vk::ClearColorValue {
                    float32: core::array::from_fn(|_| components.next().unwrap_or(0.0)),
                }
            }
            NumericFormat::SFloat => {
                let mut components = components.iter().map(|c| color[c.name] * 2.0 - 1.0);

                vk::ClearColorValue {
                    float32: core::array::from_fn(|_| components.next().unwrap_or(0.0)),
                }
            }
            NumericFormat::UFloat => {
                let mut components = components.iter().map(|c| color[c.name]);

                vk::ClearColorValue {
                    float32: core::array::from_fn(|_| components.next().unwrap_or(0.0)),
                }
            }
        }
    }

    /// Returns the component's bit offset from the least-significant bit of its plane's block.
    ///
    /// For byte-aligned formats, the byte offset is the returned value divided
    /// by eight; the component width is [`Component::bits`](crate::Component::bits).
    /// Offsets are derived from the components listed for the format, except
    /// for formats with unnamed padding at the most-significant end of the
    /// block: `X8_D24_UNORM_PACK32` and `E5B9G9R9_UFLOAT_PACK32`. Their offsets
    /// are hardcoded to match the actual Vulkan layouts.
    /// Returns `None` when the component is absent. For `Format::UNDEFINED`,
    /// this returns `None`.
    fn component_offset(self, component: ComponentName) -> Option<u16>
    where
        Self: Copy + PartialEq<Format>,
    {
        let components = self.components();
        let target = components.iter().find(|c| c.name == component)?;

        // These packed formats contain unnamed padding/exponent components at
        // the most-significant end of the block, which are not represented in
        // `components()` and therefore cannot be derived by the generic algorithm.
        // X8_D24 has X8 padding in bits 31..24 and D in bits 23..0.
        // E5B9G9R9 has E in bits 31..27, B in 26..18, G in 17..9, and R in 8..0.
        if self == Format::X8_D24_UNORM_PACK32 && component == ComponentName::D {
            return Some(0);
        }
        if self == Format::E5B9G9R9_UFLOAT_PACK32 {
            return match component {
                ComponentName::B => Some(18),
                ComponentName::G => Some(9),
                ComponentName::R => Some(0),
                _ => None,
            };
        }

        if let Some(index) = target.plane_index {
            let plane = &self.planes()[usize::from(index)];
            let compatible = plane.compatible_format;
            component_offset_in_group(
                components.iter().filter(|c| c.plane_index == Some(index)),
                component,
                (compatible.block_size() * 8) as u16,
                compatible.packed(),
            )
        } else {
            component_offset_in_group(
                components.iter().filter(|c| c.plane_index.is_none()),
                component,
                (self.block_size() * 8) as u16,
                self.packed(),
            )
        }
    }

    /// Returns whether this format is structurally eligible for a vertex input attribute.
    ///
    /// This is a structural check only; actual device support must still be
    /// checked with `vkGetPhysicalDeviceFormatProperties` and
    /// `VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT`. `Format::UNDEFINED` returns `false`.
    fn is_vertex_format(self) -> bool
    where
        Self: Copy + PartialEq<Format>,
    {
        if !self.planes().is_empty()
            || self.chroma().is_some()
            || self.compression().is_some()
            || self == Format::R16G16_S10_5_NV
        {
            return false;
        }
        let components = self.components();
        !components.is_empty()
            && components.iter().all(|component| {
                !matches!(component.name, ComponentName::D | ComponentName::S)
                    && matches!(
                        component.numeric_format,
                        NumericFormat::UNorm
                            | NumericFormat::SNorm
                            | NumericFormat::UScaled
                            | NumericFormat::SScaled
                            | NumericFormat::UInt
                            | NumericFormat::SInt
                            | NumericFormat::SFloat
                            | NumericFormat::UFloat
                    )
                    && (matches!(component.bits, 8 | 16 | 32 | 64) || self.packed() == Some(32))
            })
    }

    /// Constructs a Vulkan vertex input attribute description for this format.
    ///
    /// `self` should satisfy [`is_vertex_format`](Self::is_vertex_format), and
    /// device support should be checked separately. For `Format::UNDEFINED`,
    /// the returned description uses `Format::UNDEFINED`, which is not a valid
    /// vertex attribute format.
    fn vertex_attribute(
        self,
        location: u32,
        binding: u32,
        offset: u32,
    ) -> vk::VertexInputAttributeDescription
    where
        Self: Copy + Into<Format>,
    {
        vk::VertexInputAttributeDescription {
            location,
            binding,
            format: self.into(),
            offset,
        }
    }

    /// Returns whether any component has the sRGB numeric format.
    ///
    /// `Format::UNDEFINED` returns `false`.
    fn is_srgb(self) -> bool
    where
        Self: Copy,
    {
        self.components()
            .iter()
            .any(|component| component.numeric_format == NumericFormat::SRgb)
    }

    /// Returns whether this format has a depth component.
    ///
    /// `Format::UNDEFINED` returns `false`.
    fn has_depth(self) -> bool
    where
        Self: Copy,
    {
        self.components()
            .iter()
            .any(|component| component.name == ComponentName::D)
    }

    /// Returns whether this format has a stencil component.
    ///
    /// `Format::UNDEFINED` returns `false`.
    fn has_stencil(self) -> bool
    where
        Self: Copy,
    {
        self.components()
            .iter()
            .any(|component| component.name == ComponentName::S)
    }

    /// Returns whether this format has a depth or stencil component.
    ///
    /// `Format::UNDEFINED` returns `false`.
    fn is_depth_stencil(self) -> bool
    where
        Self: Copy,
    {
        self.has_depth() || self.has_stencil()
    }

    /// Returns whether this format uses block compression.
    ///
    /// `Format::UNDEFINED` returns `false`.
    fn is_compressed(self) -> bool
    where
        Self: Copy,
    {
        self.compression().is_some()
    }

    /// Returns whether this format has multiple planes.
    ///
    /// `Format::UNDEFINED` returns `false`.
    fn is_multi_planar(self) -> bool
    where
        Self: Copy,
    {
        !self.planes().is_empty()
    }
}

impl<T: FormatExt> FormatHelpers for T {}

fn component_offset_in_group<'a>(
    group: impl Iterator<Item = &'a Component>,
    component: ComponentName,
    unit_bits: u16,
    packing: Option<u8>,
) -> Option<u16> {
    let mut cursor = 0u16;
    for current in group {
        let packed_bits = packing.map(u16::from).unwrap_or(unit_bits);
        let remaining = cursor.next_multiple_of(packed_bits) - cursor;
        let offset = if u16::from(current.bits) <= remaining {
            let offset = cursor;
            cursor += u16::from(current.bits);
            offset
        } else {
            let offset = cursor.next_multiple_of(packed_bits);
            cursor = offset + u16::from(current.bits);
            offset
        };
        if current.name == component {
            return Some(if let Some(packed) = packing {
                let packed = u16::from(packed);
                (offset / packed) * packed + (packed - (offset % packed) - u16::from(current.bits))
            } else {
                offset
            });
        }
    }
    None
}

fn extent_block_size(
    extent: [u32; 3],
    block_extent: (u8, u8, u8),
    block_size: DeviceSize,
) -> DeviceSize {
    u64::from(extent[0].div_ceil(u32::from(block_extent.0)))
        * u64::from(extent[1].div_ceil(u32::from(block_extent.1)))
        * u64::from(extent[2].div_ceil(u32::from(block_extent.2)))
        * block_size
}

#[cfg(test)]
mod tests {
    use super::FormatHelpers;
    use crate::{ComponentName, FormatExt};
    use ash::vk::{ClearColorValue, Format};

    fn clear_float(value: ClearColorValue) -> [f32; 4] {
        unsafe { value.float32 }
    }

    fn clear_uint(value: ClearColorValue) -> [u32; 4] {
        unsafe { value.uint32 }
    }

    fn clear_int(value: ClearColorValue) -> [i32; 4] {
        unsafe { value.int32 }
    }

    #[test]
    fn component_offsets_cover_packed_and_planar_formats() {
        let offsets = [
            (Format::A1R5G5B5_UNORM_PACK16, ComponentName::A, 15),
            (Format::A1R5G5B5_UNORM_PACK16, ComponentName::R, 10),
            (Format::A1R5G5B5_UNORM_PACK16, ComponentName::G, 5),
            (Format::A1R5G5B5_UNORM_PACK16, ComponentName::B, 0),
            (Format::R10X6G10X6_UNORM_2PACK16, ComponentName::R, 6),
            (Format::R10X6G10X6_UNORM_2PACK16, ComponentName::G, 22),
            (Format::R8G8B8A8_UNORM, ComponentName::R, 0),
            (Format::R8G8B8A8_UNORM, ComponentName::G, 8),
            (Format::R8G8B8A8_UNORM, ComponentName::B, 16),
            (Format::R8G8B8A8_UNORM, ComponentName::A, 24),
            (Format::D24_UNORM_S8_UINT, ComponentName::D, 0),
            (Format::D24_UNORM_S8_UINT, ComponentName::S, 24),
            (Format::B10G11R11_UFLOAT_PACK32, ComponentName::B, 22),
            (Format::B10G11R11_UFLOAT_PACK32, ComponentName::G, 11),
            (Format::B10G11R11_UFLOAT_PACK32, ComponentName::R, 0),
            (Format::X8_D24_UNORM_PACK32, ComponentName::D, 0),
            (Format::E5B9G9R9_UFLOAT_PACK32, ComponentName::B, 18),
            (Format::E5B9G9R9_UFLOAT_PACK32, ComponentName::G, 9),
            (Format::E5B9G9R9_UFLOAT_PACK32, ComponentName::R, 0),
            (Format::A8B8G8R8_UNORM_PACK32, ComponentName::A, 24),
            (Format::A8B8G8R8_UNORM_PACK32, ComponentName::B, 16),
            (Format::A8B8G8R8_UNORM_PACK32, ComponentName::G, 8),
            (Format::A8B8G8R8_UNORM_PACK32, ComponentName::R, 0),
            (Format::R8G8_UNORM, ComponentName::R, 0),
            (Format::R8G8_UNORM, ComponentName::G, 8),
            (Format::G8_B8R8_2PLANE_420_UNORM, ComponentName::R, 8),
            (Format::G8_B8R8_2PLANE_420_UNORM, ComponentName::G, 0),
            (Format::G8_B8R8_2PLANE_420_UNORM, ComponentName::B, 0),
            (
                Format::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16,
                ComponentName::G,
                6,
            ),
            (
                Format::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16,
                ComponentName::B,
                6,
            ),
            (
                Format::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16,
                ComponentName::R,
                22,
            ),
            (Format::G8_B8_R8_3PLANE_420_UNORM, ComponentName::G, 0),
            (Format::G8_B8_R8_3PLANE_420_UNORM, ComponentName::B, 0),
            (Format::G8_B8_R8_3PLANE_420_UNORM, ComponentName::R, 0),
        ];
        for (format, component, expected) in offsets {
            assert_eq!(format.component_offset(component), Some(expected));
        }
        assert_eq!(Format::R8G8_UNORM.component_offset(ComponentName::B), None);
    }

    #[test]
    fn row_and_layer_sizes_respect_blocks_and_planes() {
        assert_eq!(Format::R8G8B8A8_UNORM.row_pitch(3), 12);
        assert_eq!(Format::BC1_RGB_UNORM_BLOCK.row_pitch(4), 8);
        assert_eq!(Format::BC1_RGB_UNORM_BLOCK.row_pitch(5), 16);
        assert_eq!(Format::BC1_RGB_UNORM_BLOCK.layer_size([8, 8, 1]), 32);
        assert_eq!(Format::BC1_RGB_UNORM_BLOCK.layer_size([4, 4, 1]), 8);

        let yuv = Format::G8_B8R8_2PLANE_420_UNORM;
        assert_eq!(yuv.plane_layer_size(0, [3, 3, 1]), 9);
        assert_eq!(yuv.plane_layer_size(1, [3, 3, 1]), 8);
        assert_eq!(yuv.layer_size([3, 3, 1]), 17);
        assert_eq!(yuv.layer_size([4, 4, 1]), 24);
    }

    #[test]
    fn clear_value_uses_numeric_format() {
        let color = [1.0, 0.5, 0.75, 0.25];
        assert_eq!(
            clear_float(Format::R8G8B8A8_UNORM.clear_value(color)),
            color
        );
        assert_eq!(
            clear_uint(Format::R8G8_UINT.clear_value(color)),
            [255, 127, 0, 0]
        );
        assert_eq!(
            clear_int(Format::R8G8_SINT.clear_value(color)),
            [127, -1, 0, 0]
        );
        assert_eq!(
            clear_int(Format::A2R10G10B10_UINT_PACK32.clear_value(color)),
            [0, 1023, 511, 767]
        );
    }

    #[test]
    fn format_predicates_match_format_properties() {
        assert!(!Format::B8G8R8A8_UNORM.is_srgb());
        assert!(Format::B8G8R8A8_SRGB.is_srgb());
        assert!(Format::D24_UNORM_S8_UINT.has_depth());
        assert!(Format::D24_UNORM_S8_UINT.has_stencil());
        assert!(Format::D24_UNORM_S8_UINT.is_depth_stencil());
        assert!(Format::BC1_RGB_UNORM_BLOCK.is_compressed());
        assert!(Format::G8_B8R8_2PLANE_420_UNORM.is_multi_planar());
    }

    #[test]
    fn vertex_format_eligibility_is_structural() {
        for format in [
            Format::R8G8B8A8_UNORM,
            Format::A2R10G10B10_UNORM_PACK32,
            Format::B10G11R11_UFLOAT_PACK32,
            Format::E5B9G9R9_UFLOAT_PACK32,
            Format::R32G32B32_SFLOAT,
            Format::R64G64_SFLOAT,
        ] {
            assert!(format.is_vertex_format(), "{format:?}");
        }
        for format in [
            Format::R8G8B8A8_SRGB,
            Format::R10X6G10X6_UNORM_2PACK16,
            Format::BC1_RGB_UNORM_BLOCK,
            Format::D32_SFLOAT,
            Format::G8B8G8R8_422_UNORM,
            Format::R16G16_S10_5_NV,
        ] {
            assert!(!format.is_vertex_format(), "{format:?}");
        }
        let attribute = Format::R8G8B8A8_UNORM.vertex_attribute(2, 3, 4);
        assert_eq!(attribute.location, 2);
        assert_eq!(attribute.binding, 3);
        assert_eq!(attribute.format, Format::R8G8B8A8_UNORM);
        assert_eq!(attribute.offset, 4);
    }

    #[test]
    fn undefined_has_documented_empty_semantics() {
        let undefined = Format::UNDEFINED;
        assert_eq!(undefined.block_size(), 0);
        assert_eq!(undefined.block_extent(), (1, 1, 1));
        assert_eq!(undefined.texels_per_block(), 0);
        assert_eq!(undefined.packed(), None);
        assert_eq!(undefined.chroma(), None);
        assert_eq!(undefined.compression(), None);
        assert!(undefined.components().is_empty());
        assert!(undefined.planes().is_empty());
        assert!(undefined.aspect_flags().is_empty());
        assert!(!undefined.is_vertex_format());
        assert_eq!(clear_float(undefined.clear_value([0.0; 4])), [0.0; 4]);
        assert_eq!(undefined.layer_size([16, 16, 1]), 0);
    }

    #[test]
    #[should_panic(expected = "Format::UNDEFINED has no format class")]
    fn undefined_has_no_format_class() {
        Format::UNDEFINED.format_class();
    }
}
