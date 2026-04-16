use ash::vk::{DeviceSize, Format, ImageAspectFlags};
pub trait FormatExt {
    #[doc = r" The size of a block of the format in bytes. For uncompressed"]
    #[doc = r" formats, this is the size of a texel."]
    fn block_size(self) -> DeviceSize;
    #[doc = r" The dimensions of a block of the format in texels. For"]
    #[doc = r" uncompressed formats, this is always (1, 1, 1)."]
    fn block_extent(self) -> (u8, u8, u8);
    #[doc = r" The number of texels in a block of the format. For uncompressed"]
    #[doc = r" formats, this is always 1."]
    fn texels_per_block(self) -> u8;
    #[doc = r" For packed formats, this describes how the components are packed"]
    #[doc = r" into byte-multiples."]
    #[doc = r" Together with the block size and component bits, this can be"]
    #[doc = r" used to determine the bit offset of each component within a"]
    #[doc = r" block."]
    #[doc = r" This follows the same logic as the packing of bit fields in C in GCC/Clang:"]
    #[doc = r" Namely, [`Format::R10X6G10X6_UNORM_2PACK16`] has a block"]
    #[doc = r" size of 4 bytes, a `packed` of 16 bits, and a two components"]
    #[doc = r" [`ComponentName::R`] and [`ComponentName::G`] with 10 bits"]
    #[doc = r" each. As the format name suggests, the two channels are each"]
    #[doc = r" stored in 16-bit words. Thus, the red channel is stored in the"]
    #[doc = r" 10 most significant bits of the first 16-bit word, and the green"]
    #[doc = r" channel is stored in the 10 most significant bits of the second"]
    #[doc = r" 16-bit word. the 6 least significant bits of each word are"]
    #[doc = r" unused."]
    #[doc = r""]
    #[doc = r" ```ignore"]
    #[doc = r" fn component_bit_offset(packed: u8, cursor: &mut u16, component_size: u8) -> u16 {"]
    #[doc = r"     let remaining_bits = cursor.next_multiple_of(packed) - *cursor;"]
    #[doc = r"     if component_size <= remaining_bits {"]
    #[doc = r"         let offset = *cursor;"]
    #[doc = r"         *cursor += component_size;"]
    #[doc = r"         offset"]
    #[doc = r"     } else {"]
    #[doc = r"         let offset = *cursor.next_multiple_of(packed);"]
    #[doc = r"         *cursor = offset + component_size;"]
    #[doc = r"         offset"]
    #[doc = r"     }"]
    #[doc = r" }"]
    #[doc = r" ```"]
    fn packed(self) -> Option<u8>;
    #[doc = r" The format class of the format. Formats in the same class are"]
    #[doc = r" comatible with each other. Images created with the"]
    #[doc = r" [`MUTABLE_FORMAT`](vk::ImageCreateFlags::MUTABLE_FORMAT) flag"]
    #[doc = r" may be used to create image views of any format in the same"]
    #[doc = r" format class as the image's format, or for multi-planar formats,"]
    #[doc = r" image views for specific planes of the image."]
    fn format_class(self) -> FormatClass;
    #[doc = r" The chroma subsampling scheme of the format, if the format is a"]
    #[doc = r" chroma subsampled format."]
    fn chroma(self) -> Option<Chroma>;
    #[doc = r" The compression scheme of the format, if the format is a compressed format."]
    fn compression(self) -> Option<Compression>;
    #[doc = r" The components (channels) of the format."]
    fn components(self) -> &'static [Component];
    #[doc = r" For multi-planar formats, the planes of the format. For"]
    #[doc = r" single-plane formats, this returns an empty slice."]
    fn planes(self) -> &'static [Plane];
    #[doc = r" The aspect flags corresponding to the format. For most formats,"]
    #[doc = r" this will simply be [`ImageAspectFlags::COLOR`]."]
    fn aspect_flags(self) -> ImageAspectFlags;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatClass {
    Bit10Plane2Chroma420,
    Bit10Plane2Chroma422,
    Bit10Plane2Chroma444,
    Bit10Plane3Chroma420,
    Bit10Plane3Chroma422,
    Bit10Plane3Chroma444,
    Bit12Plane2Chroma420,
    Bit12Plane2Chroma422,
    Bit12Plane2Chroma444,
    Bit12Plane3Chroma420,
    Bit12Plane3Chroma422,
    Bit12Plane3Chroma444,
    Bit128,
    Bit16,
    Bit16Plane2Chroma420,
    Bit16Plane2Chroma422,
    Bit16Plane2Chroma444,
    Bit16Plane3Chroma420,
    Bit16Plane3Chroma422,
    Bit16Plane3Chroma444,
    Bit192,
    Bit24,
    Bit256,
    Bit32,
    Bit32B8g8r8g8,
    Bit32G8b8g8r8,
    Bit48,
    Bit64,
    Bit64B10g10r10g10,
    Bit64B12g12r12g12,
    Bit64B16g16r16g16,
    Bit64G10b10g10r10,
    Bit64G12b12g12r12,
    Bit64G16b16g16r16,
    Bit64R10g10b10a10,
    Bit64R12g12b12a12,
    Bit8,
    Bit8Plane2Chroma420,
    Bit8Plane2Chroma422,
    Bit8Plane2Chroma444,
    Bit8Plane3Chroma420,
    Bit8Plane3Chroma422,
    Bit8Plane3Chroma444,
    Bit8Alpha,
    Bit96,
    Astc10x10,
    Astc10x5,
    Astc10x6,
    Astc10x8,
    Astc12x10,
    Astc12x12,
    Astc4x4,
    Astc5x4,
    Astc5x5,
    Astc6x5,
    Astc6x6,
    Astc8x5,
    Astc8x6,
    Astc8x8,
    Bc1Rgb,
    Bc1Rgba,
    Bc2,
    Bc3,
    Bc4,
    Bc5,
    Bc6h,
    Bc7,
    D16,
    D16s8,
    D24,
    D24s8,
    D32,
    D32s8,
    EacR,
    EacRg,
    Etc2EacRgba,
    Etc2Rgb,
    Etc2Rgba,
    Pvrtc12bpp,
    Pvrtc14bpp,
    Pvrtc22bpp,
    Pvrtc24bpp,
    S8,
}
#[doc = r" The Chroma subsampling scheme of a [`Format`]."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Chroma {
    Chroma420,
    Chroma422,
    Chroma444,
}
#[doc = r" The compression scheme of a compressed [`Format`]."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Compression {
    Bc,
    Etc2,
    Eac,
    AstcLdr,
    AstcHdr,
    Pvrtc,
}
#[doc = r" The numeric format of a [`Component`] of a [`Format`]."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericFormat {
    UInt,
    SInt,
    UNorm,
    SNorm,
    SRgb,
    UScaled,
    SScaled,
    SFloat,
    UFloat,
}
#[doc = r" The channel name of a [`Component`] of a [`Format`]."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComponentName {
    #[doc = r" Red channel."]
    R,
    #[doc = r" Green channel."]
    G,
    #[doc = r" Blue channel."]
    B,
    #[doc = r" Alpha channel."]
    A,
    #[doc = r" Depth channel."]
    D,
    #[doc = r" Stencil channel."]
    S,
}
#[doc = r" A component of a [`Format`]."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Component {
    pub name: ComponentName,
    pub numeric_format: NumericFormat,
    pub bits: u8,
    pub plane_index: Option<u8>,
}
#[doc = r" A distinct plane of a multi-planar [`Format`]."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Plane {
    pub width_divisor: u8,
    pub height_divisor: u8,
    #[doc = r" [`Format`] that is compatible with this plane."]
    pub compatible_format: Format,
}
impl FormatExt for Format {
    fn block_size(self) -> DeviceSize {
        match self {
            Self::R4G4_UNORM_PACK8
            | Self::A8_UNORM_KHR
            | Self::R8_UNORM
            | Self::R8_SNORM
            | Self::R8_USCALED
            | Self::R8_SSCALED
            | Self::R8_UINT
            | Self::R8_SINT
            | Self::R8_SRGB
            | Self::S8_UINT => 1u64,
            Self::R4G4B4A4_UNORM_PACK16
            | Self::B4G4R4A4_UNORM_PACK16
            | Self::R5G6B5_UNORM_PACK16
            | Self::B5G6R5_UNORM_PACK16
            | Self::R5G5B5A1_UNORM_PACK16
            | Self::B5G5R5A1_UNORM_PACK16
            | Self::A1R5G5B5_UNORM_PACK16
            | Self::A1B5G5R5_UNORM_PACK16_KHR
            | Self::R8G8_UNORM
            | Self::R8G8_SNORM
            | Self::R8G8_USCALED
            | Self::R8G8_SSCALED
            | Self::R8G8_UINT
            | Self::R8G8_SINT
            | Self::R8G8_SRGB
            | Self::R16_UNORM
            | Self::R16_SNORM
            | Self::R16_USCALED
            | Self::R16_SSCALED
            | Self::R16_UINT
            | Self::R16_SINT
            | Self::R16_SFLOAT
            | Self::D16_UNORM
            | Self::R10X6_UNORM_PACK16
            | Self::R12X4_UNORM_PACK16
            | Self::A4R4G4B4_UNORM_PACK16
            | Self::A4B4G4R4_UNORM_PACK16 => 2u64,
            Self::R8G8B8_UNORM
            | Self::R8G8B8_SNORM
            | Self::R8G8B8_USCALED
            | Self::R8G8B8_SSCALED
            | Self::R8G8B8_UINT
            | Self::R8G8B8_SINT
            | Self::R8G8B8_SRGB
            | Self::B8G8R8_UNORM
            | Self::B8G8R8_SNORM
            | Self::B8G8R8_USCALED
            | Self::B8G8R8_SSCALED
            | Self::B8G8R8_UINT
            | Self::B8G8R8_SINT
            | Self::B8G8R8_SRGB
            | Self::D16_UNORM_S8_UINT
            | Self::G8_B8_R8_3PLANE_420_UNORM
            | Self::G8_B8R8_2PLANE_420_UNORM
            | Self::G8_B8_R8_3PLANE_422_UNORM
            | Self::G8_B8R8_2PLANE_422_UNORM
            | Self::G8_B8_R8_3PLANE_444_UNORM
            | Self::G8_B8R8_2PLANE_444_UNORM => 3u64,
            Self::R8G8B8A8_UNORM
            | Self::R8G8B8A8_SNORM
            | Self::R8G8B8A8_USCALED
            | Self::R8G8B8A8_SSCALED
            | Self::R8G8B8A8_UINT
            | Self::R8G8B8A8_SINT
            | Self::R8G8B8A8_SRGB
            | Self::B8G8R8A8_UNORM
            | Self::B8G8R8A8_SNORM
            | Self::B8G8R8A8_USCALED
            | Self::B8G8R8A8_SSCALED
            | Self::B8G8R8A8_UINT
            | Self::B8G8R8A8_SINT
            | Self::B8G8R8A8_SRGB
            | Self::A8B8G8R8_UNORM_PACK32
            | Self::A8B8G8R8_SNORM_PACK32
            | Self::A8B8G8R8_USCALED_PACK32
            | Self::A8B8G8R8_SSCALED_PACK32
            | Self::A8B8G8R8_UINT_PACK32
            | Self::A8B8G8R8_SINT_PACK32
            | Self::A8B8G8R8_SRGB_PACK32
            | Self::A2R10G10B10_UNORM_PACK32
            | Self::A2R10G10B10_SNORM_PACK32
            | Self::A2R10G10B10_USCALED_PACK32
            | Self::A2R10G10B10_SSCALED_PACK32
            | Self::A2R10G10B10_UINT_PACK32
            | Self::A2R10G10B10_SINT_PACK32
            | Self::A2B10G10R10_UNORM_PACK32
            | Self::A2B10G10R10_SNORM_PACK32
            | Self::A2B10G10R10_USCALED_PACK32
            | Self::A2B10G10R10_SSCALED_PACK32
            | Self::A2B10G10R10_UINT_PACK32
            | Self::A2B10G10R10_SINT_PACK32
            | Self::R16G16_UNORM
            | Self::R16G16_SNORM
            | Self::R16G16_USCALED
            | Self::R16G16_SSCALED
            | Self::R16G16_UINT
            | Self::R16G16_SINT
            | Self::R16G16_SFLOAT
            | Self::R32_UINT
            | Self::R32_SINT
            | Self::R32_SFLOAT
            | Self::B10G11R11_UFLOAT_PACK32
            | Self::E5B9G9R9_UFLOAT_PACK32
            | Self::X8_D24_UNORM_PACK32
            | Self::D32_SFLOAT
            | Self::D24_UNORM_S8_UINT
            | Self::G8B8G8R8_422_UNORM
            | Self::B8G8R8G8_422_UNORM
            | Self::R10X6G10X6_UNORM_2PACK16
            | Self::R12X4G12X4_UNORM_2PACK16
            | Self::R16G16_S10_5_NV => 4u64,
            Self::D32_SFLOAT_S8_UINT => 5u64,
            Self::R16G16B16_UNORM
            | Self::R16G16B16_SNORM
            | Self::R16G16B16_USCALED
            | Self::R16G16B16_SSCALED
            | Self::R16G16B16_UINT
            | Self::R16G16B16_SINT
            | Self::R16G16B16_SFLOAT
            | Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16
            | Self::G16_B16_R16_3PLANE_420_UNORM
            | Self::G16_B16R16_2PLANE_420_UNORM
            | Self::G16_B16_R16_3PLANE_422_UNORM
            | Self::G16_B16R16_2PLANE_422_UNORM
            | Self::G16_B16_R16_3PLANE_444_UNORM
            | Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16
            | Self::G16_B16R16_2PLANE_444_UNORM => 6u64,
            Self::R16G16B16A16_UNORM
            | Self::R16G16B16A16_SNORM
            | Self::R16G16B16A16_USCALED
            | Self::R16G16B16A16_SSCALED
            | Self::R16G16B16A16_UINT
            | Self::R16G16B16A16_SINT
            | Self::R16G16B16A16_SFLOAT
            | Self::R32G32_UINT
            | Self::R32G32_SINT
            | Self::R32G32_SFLOAT
            | Self::R64_UINT
            | Self::R64_SINT
            | Self::R64_SFLOAT
            | Self::BC1_RGB_UNORM_BLOCK
            | Self::BC1_RGB_SRGB_BLOCK
            | Self::BC1_RGBA_UNORM_BLOCK
            | Self::BC1_RGBA_SRGB_BLOCK
            | Self::BC4_UNORM_BLOCK
            | Self::BC4_SNORM_BLOCK
            | Self::ETC2_R8G8B8_UNORM_BLOCK
            | Self::ETC2_R8G8B8_SRGB_BLOCK
            | Self::ETC2_R8G8B8A1_UNORM_BLOCK
            | Self::ETC2_R8G8B8A1_SRGB_BLOCK
            | Self::EAC_R11_UNORM_BLOCK
            | Self::EAC_R11_SNORM_BLOCK
            | Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16
            | Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16
            | Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16
            | Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16
            | Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16
            | Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16
            | Self::G16B16G16R16_422_UNORM
            | Self::B16G16R16G16_422_UNORM
            | Self::PVRTC1_2BPP_UNORM_BLOCK_IMG
            | Self::PVRTC1_4BPP_UNORM_BLOCK_IMG
            | Self::PVRTC2_2BPP_UNORM_BLOCK_IMG
            | Self::PVRTC2_4BPP_UNORM_BLOCK_IMG
            | Self::PVRTC1_2BPP_SRGB_BLOCK_IMG
            | Self::PVRTC1_4BPP_SRGB_BLOCK_IMG
            | Self::PVRTC2_2BPP_SRGB_BLOCK_IMG
            | Self::PVRTC2_4BPP_SRGB_BLOCK_IMG => 8u64,
            Self::R32G32B32_UINT | Self::R32G32B32_SINT | Self::R32G32B32_SFLOAT => 12u64,
            Self::R32G32B32A32_UINT
            | Self::R32G32B32A32_SINT
            | Self::R32G32B32A32_SFLOAT
            | Self::R64G64_UINT
            | Self::R64G64_SINT
            | Self::R64G64_SFLOAT
            | Self::BC2_UNORM_BLOCK
            | Self::BC2_SRGB_BLOCK
            | Self::BC3_UNORM_BLOCK
            | Self::BC3_SRGB_BLOCK
            | Self::BC5_UNORM_BLOCK
            | Self::BC5_SNORM_BLOCK
            | Self::BC6H_UFLOAT_BLOCK
            | Self::BC6H_SFLOAT_BLOCK
            | Self::BC7_UNORM_BLOCK
            | Self::BC7_SRGB_BLOCK
            | Self::ETC2_R8G8B8A8_UNORM_BLOCK
            | Self::ETC2_R8G8B8A8_SRGB_BLOCK
            | Self::EAC_R11G11_UNORM_BLOCK
            | Self::EAC_R11G11_SNORM_BLOCK
            | Self::ASTC_4X4_UNORM_BLOCK
            | Self::ASTC_4X4_SRGB_BLOCK
            | Self::ASTC_5X4_UNORM_BLOCK
            | Self::ASTC_5X4_SRGB_BLOCK
            | Self::ASTC_5X5_UNORM_BLOCK
            | Self::ASTC_5X5_SRGB_BLOCK
            | Self::ASTC_6X5_UNORM_BLOCK
            | Self::ASTC_6X5_SRGB_BLOCK
            | Self::ASTC_6X6_UNORM_BLOCK
            | Self::ASTC_6X6_SRGB_BLOCK
            | Self::ASTC_8X5_UNORM_BLOCK
            | Self::ASTC_8X5_SRGB_BLOCK
            | Self::ASTC_8X6_UNORM_BLOCK
            | Self::ASTC_8X6_SRGB_BLOCK
            | Self::ASTC_8X8_UNORM_BLOCK
            | Self::ASTC_8X8_SRGB_BLOCK
            | Self::ASTC_10X5_UNORM_BLOCK
            | Self::ASTC_10X5_SRGB_BLOCK
            | Self::ASTC_10X6_UNORM_BLOCK
            | Self::ASTC_10X6_SRGB_BLOCK
            | Self::ASTC_10X8_UNORM_BLOCK
            | Self::ASTC_10X8_SRGB_BLOCK
            | Self::ASTC_10X10_UNORM_BLOCK
            | Self::ASTC_10X10_SRGB_BLOCK
            | Self::ASTC_12X10_UNORM_BLOCK
            | Self::ASTC_12X10_SRGB_BLOCK
            | Self::ASTC_12X12_UNORM_BLOCK
            | Self::ASTC_12X12_SRGB_BLOCK
            | Self::ASTC_4X4_SFLOAT_BLOCK
            | Self::ASTC_5X4_SFLOAT_BLOCK
            | Self::ASTC_5X5_SFLOAT_BLOCK
            | Self::ASTC_6X5_SFLOAT_BLOCK
            | Self::ASTC_6X6_SFLOAT_BLOCK
            | Self::ASTC_8X5_SFLOAT_BLOCK
            | Self::ASTC_8X6_SFLOAT_BLOCK
            | Self::ASTC_8X8_SFLOAT_BLOCK
            | Self::ASTC_10X5_SFLOAT_BLOCK
            | Self::ASTC_10X6_SFLOAT_BLOCK
            | Self::ASTC_10X8_SFLOAT_BLOCK
            | Self::ASTC_10X10_SFLOAT_BLOCK
            | Self::ASTC_12X10_SFLOAT_BLOCK
            | Self::ASTC_12X12_SFLOAT_BLOCK => 16u64,
            Self::R64G64B64_UINT | Self::R64G64B64_SINT | Self::R64G64B64_SFLOAT => 24u64,
            Self::R64G64B64A64_UINT | Self::R64G64B64A64_SINT | Self::R64G64B64A64_SFLOAT => 32u64,
            _ => panic!("Unknown format vk::Format({:?})", self.as_raw()),
        }
    }
    fn block_extent(self) -> (u8, u8, u8) {
        match self {
            Self::G8B8G8R8_422_UNORM
            | Self::B8G8R8G8_422_UNORM
            | Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16
            | Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16
            | Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16
            | Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16
            | Self::G16B16G16R16_422_UNORM
            | Self::B16G16R16G16_422_UNORM => (2u8, 1u8, 1u8),
            Self::BC1_RGB_UNORM_BLOCK
            | Self::BC1_RGB_SRGB_BLOCK
            | Self::BC1_RGBA_UNORM_BLOCK
            | Self::BC1_RGBA_SRGB_BLOCK
            | Self::BC2_UNORM_BLOCK
            | Self::BC2_SRGB_BLOCK
            | Self::BC3_UNORM_BLOCK
            | Self::BC3_SRGB_BLOCK
            | Self::BC4_UNORM_BLOCK
            | Self::BC4_SNORM_BLOCK
            | Self::BC5_UNORM_BLOCK
            | Self::BC5_SNORM_BLOCK
            | Self::BC6H_UFLOAT_BLOCK
            | Self::BC6H_SFLOAT_BLOCK
            | Self::BC7_UNORM_BLOCK
            | Self::BC7_SRGB_BLOCK
            | Self::ETC2_R8G8B8_UNORM_BLOCK
            | Self::ETC2_R8G8B8_SRGB_BLOCK
            | Self::ETC2_R8G8B8A1_UNORM_BLOCK
            | Self::ETC2_R8G8B8A1_SRGB_BLOCK
            | Self::ETC2_R8G8B8A8_UNORM_BLOCK
            | Self::ETC2_R8G8B8A8_SRGB_BLOCK
            | Self::EAC_R11_UNORM_BLOCK
            | Self::EAC_R11_SNORM_BLOCK
            | Self::EAC_R11G11_UNORM_BLOCK
            | Self::EAC_R11G11_SNORM_BLOCK
            | Self::ASTC_4X4_UNORM_BLOCK
            | Self::ASTC_4X4_SRGB_BLOCK
            | Self::PVRTC1_4BPP_UNORM_BLOCK_IMG
            | Self::PVRTC2_4BPP_UNORM_BLOCK_IMG
            | Self::PVRTC1_4BPP_SRGB_BLOCK_IMG
            | Self::PVRTC2_4BPP_SRGB_BLOCK_IMG
            | Self::ASTC_4X4_SFLOAT_BLOCK => (4u8, 4u8, 1u8),
            Self::ASTC_5X4_UNORM_BLOCK
            | Self::ASTC_5X4_SRGB_BLOCK
            | Self::ASTC_5X4_SFLOAT_BLOCK => (5u8, 4u8, 1u8),
            Self::ASTC_5X5_UNORM_BLOCK
            | Self::ASTC_5X5_SRGB_BLOCK
            | Self::ASTC_5X5_SFLOAT_BLOCK => (5u8, 5u8, 1u8),
            Self::ASTC_6X5_UNORM_BLOCK
            | Self::ASTC_6X5_SRGB_BLOCK
            | Self::ASTC_6X5_SFLOAT_BLOCK => (6u8, 5u8, 1u8),
            Self::ASTC_6X6_UNORM_BLOCK
            | Self::ASTC_6X6_SRGB_BLOCK
            | Self::ASTC_6X6_SFLOAT_BLOCK => (6u8, 6u8, 1u8),
            Self::PVRTC1_2BPP_UNORM_BLOCK_IMG
            | Self::PVRTC2_2BPP_UNORM_BLOCK_IMG
            | Self::PVRTC1_2BPP_SRGB_BLOCK_IMG
            | Self::PVRTC2_2BPP_SRGB_BLOCK_IMG => (8u8, 4u8, 1u8),
            Self::ASTC_8X5_UNORM_BLOCK
            | Self::ASTC_8X5_SRGB_BLOCK
            | Self::ASTC_8X5_SFLOAT_BLOCK => (8u8, 5u8, 1u8),
            Self::ASTC_8X6_UNORM_BLOCK
            | Self::ASTC_8X6_SRGB_BLOCK
            | Self::ASTC_8X6_SFLOAT_BLOCK => (8u8, 6u8, 1u8),
            Self::ASTC_8X8_UNORM_BLOCK
            | Self::ASTC_8X8_SRGB_BLOCK
            | Self::ASTC_8X8_SFLOAT_BLOCK => (8u8, 8u8, 1u8),
            Self::ASTC_10X5_UNORM_BLOCK
            | Self::ASTC_10X5_SRGB_BLOCK
            | Self::ASTC_10X5_SFLOAT_BLOCK => (10u8, 5u8, 1u8),
            Self::ASTC_10X6_UNORM_BLOCK
            | Self::ASTC_10X6_SRGB_BLOCK
            | Self::ASTC_10X6_SFLOAT_BLOCK => (10u8, 6u8, 1u8),
            Self::ASTC_10X8_UNORM_BLOCK
            | Self::ASTC_10X8_SRGB_BLOCK
            | Self::ASTC_10X8_SFLOAT_BLOCK => (10u8, 8u8, 1u8),
            Self::ASTC_10X10_UNORM_BLOCK
            | Self::ASTC_10X10_SRGB_BLOCK
            | Self::ASTC_10X10_SFLOAT_BLOCK => (10u8, 10u8, 1u8),
            Self::ASTC_12X10_UNORM_BLOCK
            | Self::ASTC_12X10_SRGB_BLOCK
            | Self::ASTC_12X10_SFLOAT_BLOCK => (12u8, 10u8, 1u8),
            Self::ASTC_12X12_UNORM_BLOCK
            | Self::ASTC_12X12_SRGB_BLOCK
            | Self::ASTC_12X12_SFLOAT_BLOCK => (12u8, 12u8, 1u8),
            _ => (1u8, 1u8, 1u8),
        }
    }
    fn texels_per_block(self) -> u8 {
        match self {
            Self::R4G4_UNORM_PACK8
            | Self::R4G4B4A4_UNORM_PACK16
            | Self::B4G4R4A4_UNORM_PACK16
            | Self::R5G6B5_UNORM_PACK16
            | Self::B5G6R5_UNORM_PACK16
            | Self::R5G5B5A1_UNORM_PACK16
            | Self::B5G5R5A1_UNORM_PACK16
            | Self::A1R5G5B5_UNORM_PACK16
            | Self::A1B5G5R5_UNORM_PACK16_KHR
            | Self::A8_UNORM_KHR
            | Self::R8_UNORM
            | Self::R8_SNORM
            | Self::R8_USCALED
            | Self::R8_SSCALED
            | Self::R8_UINT
            | Self::R8_SINT
            | Self::R8_SRGB
            | Self::R8G8_UNORM
            | Self::R8G8_SNORM
            | Self::R8G8_USCALED
            | Self::R8G8_SSCALED
            | Self::R8G8_UINT
            | Self::R8G8_SINT
            | Self::R8G8_SRGB
            | Self::R8G8B8_UNORM
            | Self::R8G8B8_SNORM
            | Self::R8G8B8_USCALED
            | Self::R8G8B8_SSCALED
            | Self::R8G8B8_UINT
            | Self::R8G8B8_SINT
            | Self::R8G8B8_SRGB
            | Self::B8G8R8_UNORM
            | Self::B8G8R8_SNORM
            | Self::B8G8R8_USCALED
            | Self::B8G8R8_SSCALED
            | Self::B8G8R8_UINT
            | Self::B8G8R8_SINT
            | Self::B8G8R8_SRGB
            | Self::R8G8B8A8_UNORM
            | Self::R8G8B8A8_SNORM
            | Self::R8G8B8A8_USCALED
            | Self::R8G8B8A8_SSCALED
            | Self::R8G8B8A8_UINT
            | Self::R8G8B8A8_SINT
            | Self::R8G8B8A8_SRGB
            | Self::B8G8R8A8_UNORM
            | Self::B8G8R8A8_SNORM
            | Self::B8G8R8A8_USCALED
            | Self::B8G8R8A8_SSCALED
            | Self::B8G8R8A8_UINT
            | Self::B8G8R8A8_SINT
            | Self::B8G8R8A8_SRGB
            | Self::A8B8G8R8_UNORM_PACK32
            | Self::A8B8G8R8_SNORM_PACK32
            | Self::A8B8G8R8_USCALED_PACK32
            | Self::A8B8G8R8_SSCALED_PACK32
            | Self::A8B8G8R8_UINT_PACK32
            | Self::A8B8G8R8_SINT_PACK32
            | Self::A8B8G8R8_SRGB_PACK32
            | Self::A2R10G10B10_UNORM_PACK32
            | Self::A2R10G10B10_SNORM_PACK32
            | Self::A2R10G10B10_USCALED_PACK32
            | Self::A2R10G10B10_SSCALED_PACK32
            | Self::A2R10G10B10_UINT_PACK32
            | Self::A2R10G10B10_SINT_PACK32
            | Self::A2B10G10R10_UNORM_PACK32
            | Self::A2B10G10R10_SNORM_PACK32
            | Self::A2B10G10R10_USCALED_PACK32
            | Self::A2B10G10R10_SSCALED_PACK32
            | Self::A2B10G10R10_UINT_PACK32
            | Self::A2B10G10R10_SINT_PACK32
            | Self::R16_UNORM
            | Self::R16_SNORM
            | Self::R16_USCALED
            | Self::R16_SSCALED
            | Self::R16_UINT
            | Self::R16_SINT
            | Self::R16_SFLOAT
            | Self::R16G16_UNORM
            | Self::R16G16_SNORM
            | Self::R16G16_USCALED
            | Self::R16G16_SSCALED
            | Self::R16G16_UINT
            | Self::R16G16_SINT
            | Self::R16G16_SFLOAT
            | Self::R16G16B16_UNORM
            | Self::R16G16B16_SNORM
            | Self::R16G16B16_USCALED
            | Self::R16G16B16_SSCALED
            | Self::R16G16B16_UINT
            | Self::R16G16B16_SINT
            | Self::R16G16B16_SFLOAT
            | Self::R16G16B16A16_UNORM
            | Self::R16G16B16A16_SNORM
            | Self::R16G16B16A16_USCALED
            | Self::R16G16B16A16_SSCALED
            | Self::R16G16B16A16_UINT
            | Self::R16G16B16A16_SINT
            | Self::R16G16B16A16_SFLOAT
            | Self::R32_UINT
            | Self::R32_SINT
            | Self::R32_SFLOAT
            | Self::R32G32_UINT
            | Self::R32G32_SINT
            | Self::R32G32_SFLOAT
            | Self::R32G32B32_UINT
            | Self::R32G32B32_SINT
            | Self::R32G32B32_SFLOAT
            | Self::R32G32B32A32_UINT
            | Self::R32G32B32A32_SINT
            | Self::R32G32B32A32_SFLOAT
            | Self::R64_UINT
            | Self::R64_SINT
            | Self::R64_SFLOAT
            | Self::R64G64_UINT
            | Self::R64G64_SINT
            | Self::R64G64_SFLOAT
            | Self::R64G64B64_UINT
            | Self::R64G64B64_SINT
            | Self::R64G64B64_SFLOAT
            | Self::R64G64B64A64_UINT
            | Self::R64G64B64A64_SINT
            | Self::R64G64B64A64_SFLOAT
            | Self::B10G11R11_UFLOAT_PACK32
            | Self::E5B9G9R9_UFLOAT_PACK32
            | Self::D16_UNORM
            | Self::X8_D24_UNORM_PACK32
            | Self::D32_SFLOAT
            | Self::S8_UINT
            | Self::D16_UNORM_S8_UINT
            | Self::D24_UNORM_S8_UINT
            | Self::D32_SFLOAT_S8_UINT
            | Self::G8B8G8R8_422_UNORM
            | Self::B8G8R8G8_422_UNORM
            | Self::G8_B8_R8_3PLANE_420_UNORM
            | Self::G8_B8R8_2PLANE_420_UNORM
            | Self::G8_B8_R8_3PLANE_422_UNORM
            | Self::G8_B8R8_2PLANE_422_UNORM
            | Self::G8_B8_R8_3PLANE_444_UNORM
            | Self::R10X6_UNORM_PACK16
            | Self::R10X6G10X6_UNORM_2PACK16
            | Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16
            | Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16
            | Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16
            | Self::R12X4_UNORM_PACK16
            | Self::R12X4G12X4_UNORM_2PACK16
            | Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16
            | Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16
            | Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16
            | Self::G16B16G16R16_422_UNORM
            | Self::B16G16R16G16_422_UNORM
            | Self::G16_B16_R16_3PLANE_420_UNORM
            | Self::G16_B16R16_2PLANE_420_UNORM
            | Self::G16_B16_R16_3PLANE_422_UNORM
            | Self::G16_B16R16_2PLANE_422_UNORM
            | Self::G16_B16_R16_3PLANE_444_UNORM
            | Self::PVRTC1_2BPP_UNORM_BLOCK_IMG
            | Self::PVRTC1_4BPP_UNORM_BLOCK_IMG
            | Self::PVRTC2_2BPP_UNORM_BLOCK_IMG
            | Self::PVRTC2_4BPP_UNORM_BLOCK_IMG
            | Self::PVRTC1_2BPP_SRGB_BLOCK_IMG
            | Self::PVRTC1_4BPP_SRGB_BLOCK_IMG
            | Self::PVRTC2_2BPP_SRGB_BLOCK_IMG
            | Self::PVRTC2_4BPP_SRGB_BLOCK_IMG
            | Self::G8_B8R8_2PLANE_444_UNORM
            | Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16
            | Self::G16_B16R16_2PLANE_444_UNORM
            | Self::A4R4G4B4_UNORM_PACK16
            | Self::A4B4G4R4_UNORM_PACK16
            | Self::R16G16_S10_5_NV => 1u8,
            Self::BC1_RGB_UNORM_BLOCK
            | Self::BC1_RGB_SRGB_BLOCK
            | Self::BC1_RGBA_UNORM_BLOCK
            | Self::BC1_RGBA_SRGB_BLOCK
            | Self::BC2_UNORM_BLOCK
            | Self::BC2_SRGB_BLOCK
            | Self::BC3_UNORM_BLOCK
            | Self::BC3_SRGB_BLOCK
            | Self::BC4_UNORM_BLOCK
            | Self::BC4_SNORM_BLOCK
            | Self::BC5_UNORM_BLOCK
            | Self::BC5_SNORM_BLOCK
            | Self::BC6H_UFLOAT_BLOCK
            | Self::BC6H_SFLOAT_BLOCK
            | Self::BC7_UNORM_BLOCK
            | Self::BC7_SRGB_BLOCK
            | Self::ETC2_R8G8B8_UNORM_BLOCK
            | Self::ETC2_R8G8B8_SRGB_BLOCK
            | Self::ETC2_R8G8B8A1_UNORM_BLOCK
            | Self::ETC2_R8G8B8A1_SRGB_BLOCK
            | Self::ETC2_R8G8B8A8_UNORM_BLOCK
            | Self::ETC2_R8G8B8A8_SRGB_BLOCK
            | Self::EAC_R11_UNORM_BLOCK
            | Self::EAC_R11_SNORM_BLOCK
            | Self::EAC_R11G11_UNORM_BLOCK
            | Self::EAC_R11G11_SNORM_BLOCK
            | Self::ASTC_4X4_UNORM_BLOCK
            | Self::ASTC_4X4_SRGB_BLOCK
            | Self::ASTC_4X4_SFLOAT_BLOCK => 16u8,
            Self::ASTC_5X4_UNORM_BLOCK
            | Self::ASTC_5X4_SRGB_BLOCK
            | Self::ASTC_5X4_SFLOAT_BLOCK => 20u8,
            Self::ASTC_5X5_UNORM_BLOCK
            | Self::ASTC_5X5_SRGB_BLOCK
            | Self::ASTC_5X5_SFLOAT_BLOCK => 25u8,
            Self::ASTC_6X5_UNORM_BLOCK
            | Self::ASTC_6X5_SRGB_BLOCK
            | Self::ASTC_6X5_SFLOAT_BLOCK => 30u8,
            Self::ASTC_6X6_UNORM_BLOCK
            | Self::ASTC_6X6_SRGB_BLOCK
            | Self::ASTC_6X6_SFLOAT_BLOCK => 36u8,
            Self::ASTC_8X5_UNORM_BLOCK
            | Self::ASTC_8X5_SRGB_BLOCK
            | Self::ASTC_8X5_SFLOAT_BLOCK => 40u8,
            Self::ASTC_8X6_UNORM_BLOCK
            | Self::ASTC_8X6_SRGB_BLOCK
            | Self::ASTC_8X6_SFLOAT_BLOCK => 48u8,
            Self::ASTC_10X5_UNORM_BLOCK
            | Self::ASTC_10X5_SRGB_BLOCK
            | Self::ASTC_10X5_SFLOAT_BLOCK => 50u8,
            Self::ASTC_10X6_UNORM_BLOCK
            | Self::ASTC_10X6_SRGB_BLOCK
            | Self::ASTC_10X6_SFLOAT_BLOCK => 60u8,
            Self::ASTC_8X8_UNORM_BLOCK
            | Self::ASTC_8X8_SRGB_BLOCK
            | Self::ASTC_8X8_SFLOAT_BLOCK => 64u8,
            Self::ASTC_10X8_UNORM_BLOCK
            | Self::ASTC_10X8_SRGB_BLOCK
            | Self::ASTC_10X8_SFLOAT_BLOCK => 80u8,
            Self::ASTC_10X10_UNORM_BLOCK
            | Self::ASTC_10X10_SRGB_BLOCK
            | Self::ASTC_10X10_SFLOAT_BLOCK => 100u8,
            Self::ASTC_12X10_UNORM_BLOCK
            | Self::ASTC_12X10_SRGB_BLOCK
            | Self::ASTC_12X10_SFLOAT_BLOCK => 120u8,
            Self::ASTC_12X12_UNORM_BLOCK
            | Self::ASTC_12X12_SRGB_BLOCK
            | Self::ASTC_12X12_SFLOAT_BLOCK => 144u8,
            _ => panic!("Unknown format vk::Format({:?})", self.as_raw()),
        }
    }
    fn packed(self) -> Option<u8> {
        match self {
            Self::R4G4_UNORM_PACK8 => Some(8u8),
            Self::R4G4B4A4_UNORM_PACK16
            | Self::B4G4R4A4_UNORM_PACK16
            | Self::R5G6B5_UNORM_PACK16
            | Self::B5G6R5_UNORM_PACK16
            | Self::R5G5B5A1_UNORM_PACK16
            | Self::B5G5R5A1_UNORM_PACK16
            | Self::A1R5G5B5_UNORM_PACK16
            | Self::A1B5G5R5_UNORM_PACK16_KHR
            | Self::R10X6_UNORM_PACK16
            | Self::R10X6G10X6_UNORM_2PACK16
            | Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16
            | Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16
            | Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16
            | Self::R12X4_UNORM_PACK16
            | Self::R12X4G12X4_UNORM_2PACK16
            | Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16
            | Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16
            | Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16
            | Self::A4R4G4B4_UNORM_PACK16
            | Self::A4B4G4R4_UNORM_PACK16 => Some(16u8),
            Self::A8B8G8R8_UNORM_PACK32
            | Self::A8B8G8R8_SNORM_PACK32
            | Self::A8B8G8R8_USCALED_PACK32
            | Self::A8B8G8R8_SSCALED_PACK32
            | Self::A8B8G8R8_UINT_PACK32
            | Self::A8B8G8R8_SINT_PACK32
            | Self::A8B8G8R8_SRGB_PACK32
            | Self::A2R10G10B10_UNORM_PACK32
            | Self::A2R10G10B10_SNORM_PACK32
            | Self::A2R10G10B10_USCALED_PACK32
            | Self::A2R10G10B10_SSCALED_PACK32
            | Self::A2R10G10B10_UINT_PACK32
            | Self::A2R10G10B10_SINT_PACK32
            | Self::A2B10G10R10_UNORM_PACK32
            | Self::A2B10G10R10_SNORM_PACK32
            | Self::A2B10G10R10_USCALED_PACK32
            | Self::A2B10G10R10_SSCALED_PACK32
            | Self::A2B10G10R10_UINT_PACK32
            | Self::A2B10G10R10_SINT_PACK32
            | Self::B10G11R11_UFLOAT_PACK32
            | Self::E5B9G9R9_UFLOAT_PACK32
            | Self::X8_D24_UNORM_PACK32 => Some(32u8),
            _ => None,
        }
    }
    fn format_class(self) -> FormatClass {
        match self {
            Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => FormatClass::Bit10Plane2Chroma420,
            Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => FormatClass::Bit10Plane2Chroma422,
            Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 => FormatClass::Bit10Plane2Chroma444,
            Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => FormatClass::Bit10Plane3Chroma420,
            Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => FormatClass::Bit10Plane3Chroma422,
            Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => FormatClass::Bit10Plane3Chroma444,
            Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => FormatClass::Bit12Plane2Chroma420,
            Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => FormatClass::Bit12Plane2Chroma422,
            Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 => FormatClass::Bit12Plane2Chroma444,
            Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => FormatClass::Bit12Plane3Chroma420,
            Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => FormatClass::Bit12Plane3Chroma422,
            Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => FormatClass::Bit12Plane3Chroma444,
            Self::R32G32B32A32_UINT
            | Self::R32G32B32A32_SINT
            | Self::R32G32B32A32_SFLOAT
            | Self::R64G64_UINT
            | Self::R64G64_SINT
            | Self::R64G64_SFLOAT => FormatClass::Bit128,
            Self::R4G4B4A4_UNORM_PACK16
            | Self::B4G4R4A4_UNORM_PACK16
            | Self::R5G6B5_UNORM_PACK16
            | Self::B5G6R5_UNORM_PACK16
            | Self::R5G5B5A1_UNORM_PACK16
            | Self::B5G5R5A1_UNORM_PACK16
            | Self::A1R5G5B5_UNORM_PACK16
            | Self::A1B5G5R5_UNORM_PACK16_KHR
            | Self::R8G8_UNORM
            | Self::R8G8_SNORM
            | Self::R8G8_USCALED
            | Self::R8G8_SSCALED
            | Self::R8G8_UINT
            | Self::R8G8_SINT
            | Self::R8G8_SRGB
            | Self::R16_UNORM
            | Self::R16_SNORM
            | Self::R16_USCALED
            | Self::R16_SSCALED
            | Self::R16_UINT
            | Self::R16_SINT
            | Self::R16_SFLOAT
            | Self::R10X6_UNORM_PACK16
            | Self::R12X4_UNORM_PACK16
            | Self::A4R4G4B4_UNORM_PACK16
            | Self::A4B4G4R4_UNORM_PACK16 => FormatClass::Bit16,
            Self::G16_B16R16_2PLANE_420_UNORM => FormatClass::Bit16Plane2Chroma420,
            Self::G16_B16R16_2PLANE_422_UNORM => FormatClass::Bit16Plane2Chroma422,
            Self::G16_B16R16_2PLANE_444_UNORM => FormatClass::Bit16Plane2Chroma444,
            Self::G16_B16_R16_3PLANE_420_UNORM => FormatClass::Bit16Plane3Chroma420,
            Self::G16_B16_R16_3PLANE_422_UNORM => FormatClass::Bit16Plane3Chroma422,
            Self::G16_B16_R16_3PLANE_444_UNORM => FormatClass::Bit16Plane3Chroma444,
            Self::R64G64B64_UINT | Self::R64G64B64_SINT | Self::R64G64B64_SFLOAT => {
                FormatClass::Bit192
            }
            Self::R8G8B8_UNORM
            | Self::R8G8B8_SNORM
            | Self::R8G8B8_USCALED
            | Self::R8G8B8_SSCALED
            | Self::R8G8B8_UINT
            | Self::R8G8B8_SINT
            | Self::R8G8B8_SRGB
            | Self::B8G8R8_UNORM
            | Self::B8G8R8_SNORM
            | Self::B8G8R8_USCALED
            | Self::B8G8R8_SSCALED
            | Self::B8G8R8_UINT
            | Self::B8G8R8_SINT
            | Self::B8G8R8_SRGB => FormatClass::Bit24,
            Self::R64G64B64A64_UINT | Self::R64G64B64A64_SINT | Self::R64G64B64A64_SFLOAT => {
                FormatClass::Bit256
            }
            Self::R8G8B8A8_UNORM
            | Self::R8G8B8A8_SNORM
            | Self::R8G8B8A8_USCALED
            | Self::R8G8B8A8_SSCALED
            | Self::R8G8B8A8_UINT
            | Self::R8G8B8A8_SINT
            | Self::R8G8B8A8_SRGB
            | Self::B8G8R8A8_UNORM
            | Self::B8G8R8A8_SNORM
            | Self::B8G8R8A8_USCALED
            | Self::B8G8R8A8_SSCALED
            | Self::B8G8R8A8_UINT
            | Self::B8G8R8A8_SINT
            | Self::B8G8R8A8_SRGB
            | Self::A8B8G8R8_UNORM_PACK32
            | Self::A8B8G8R8_SNORM_PACK32
            | Self::A8B8G8R8_USCALED_PACK32
            | Self::A8B8G8R8_SSCALED_PACK32
            | Self::A8B8G8R8_UINT_PACK32
            | Self::A8B8G8R8_SINT_PACK32
            | Self::A8B8G8R8_SRGB_PACK32
            | Self::A2R10G10B10_UNORM_PACK32
            | Self::A2R10G10B10_SNORM_PACK32
            | Self::A2R10G10B10_USCALED_PACK32
            | Self::A2R10G10B10_SSCALED_PACK32
            | Self::A2R10G10B10_UINT_PACK32
            | Self::A2R10G10B10_SINT_PACK32
            | Self::A2B10G10R10_UNORM_PACK32
            | Self::A2B10G10R10_SNORM_PACK32
            | Self::A2B10G10R10_USCALED_PACK32
            | Self::A2B10G10R10_SSCALED_PACK32
            | Self::A2B10G10R10_UINT_PACK32
            | Self::A2B10G10R10_SINT_PACK32
            | Self::R16G16_UNORM
            | Self::R16G16_SNORM
            | Self::R16G16_USCALED
            | Self::R16G16_SSCALED
            | Self::R16G16_UINT
            | Self::R16G16_SINT
            | Self::R16G16_SFLOAT
            | Self::R32_UINT
            | Self::R32_SINT
            | Self::R32_SFLOAT
            | Self::B10G11R11_UFLOAT_PACK32
            | Self::E5B9G9R9_UFLOAT_PACK32
            | Self::R10X6G10X6_UNORM_2PACK16
            | Self::R12X4G12X4_UNORM_2PACK16
            | Self::R16G16_S10_5_NV => FormatClass::Bit32,
            Self::B8G8R8G8_422_UNORM => FormatClass::Bit32B8g8r8g8,
            Self::G8B8G8R8_422_UNORM => FormatClass::Bit32G8b8g8r8,
            Self::R16G16B16_UNORM
            | Self::R16G16B16_SNORM
            | Self::R16G16B16_USCALED
            | Self::R16G16B16_SSCALED
            | Self::R16G16B16_UINT
            | Self::R16G16B16_SINT
            | Self::R16G16B16_SFLOAT => FormatClass::Bit48,
            Self::R16G16B16A16_UNORM
            | Self::R16G16B16A16_SNORM
            | Self::R16G16B16A16_USCALED
            | Self::R16G16B16A16_SSCALED
            | Self::R16G16B16A16_UINT
            | Self::R16G16B16A16_SINT
            | Self::R16G16B16A16_SFLOAT
            | Self::R32G32_UINT
            | Self::R32G32_SINT
            | Self::R32G32_SFLOAT
            | Self::R64_UINT
            | Self::R64_SINT
            | Self::R64_SFLOAT => FormatClass::Bit64,
            Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => FormatClass::Bit64B10g10r10g10,
            Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => FormatClass::Bit64B12g12r12g12,
            Self::B16G16R16G16_422_UNORM => FormatClass::Bit64B16g16r16g16,
            Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => FormatClass::Bit64G10b10g10r10,
            Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => FormatClass::Bit64G12b12g12r12,
            Self::G16B16G16R16_422_UNORM => FormatClass::Bit64G16b16g16r16,
            Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => FormatClass::Bit64R10g10b10a10,
            Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => FormatClass::Bit64R12g12b12a12,
            Self::R4G4_UNORM_PACK8
            | Self::R8_UNORM
            | Self::R8_SNORM
            | Self::R8_USCALED
            | Self::R8_SSCALED
            | Self::R8_UINT
            | Self::R8_SINT
            | Self::R8_SRGB => FormatClass::Bit8,
            Self::G8_B8R8_2PLANE_420_UNORM => FormatClass::Bit8Plane2Chroma420,
            Self::G8_B8R8_2PLANE_422_UNORM => FormatClass::Bit8Plane2Chroma422,
            Self::G8_B8R8_2PLANE_444_UNORM => FormatClass::Bit8Plane2Chroma444,
            Self::G8_B8_R8_3PLANE_420_UNORM => FormatClass::Bit8Plane3Chroma420,
            Self::G8_B8_R8_3PLANE_422_UNORM => FormatClass::Bit8Plane3Chroma422,
            Self::G8_B8_R8_3PLANE_444_UNORM => FormatClass::Bit8Plane3Chroma444,
            Self::A8_UNORM_KHR => FormatClass::Bit8Alpha,
            Self::R32G32B32_UINT | Self::R32G32B32_SINT | Self::R32G32B32_SFLOAT => {
                FormatClass::Bit96
            }
            Self::ASTC_10X10_UNORM_BLOCK
            | Self::ASTC_10X10_SRGB_BLOCK
            | Self::ASTC_10X10_SFLOAT_BLOCK => FormatClass::Astc10x10,
            Self::ASTC_10X5_UNORM_BLOCK
            | Self::ASTC_10X5_SRGB_BLOCK
            | Self::ASTC_10X5_SFLOAT_BLOCK => FormatClass::Astc10x5,
            Self::ASTC_10X6_UNORM_BLOCK
            | Self::ASTC_10X6_SRGB_BLOCK
            | Self::ASTC_10X6_SFLOAT_BLOCK => FormatClass::Astc10x6,
            Self::ASTC_10X8_UNORM_BLOCK
            | Self::ASTC_10X8_SRGB_BLOCK
            | Self::ASTC_10X8_SFLOAT_BLOCK => FormatClass::Astc10x8,
            Self::ASTC_12X10_UNORM_BLOCK
            | Self::ASTC_12X10_SRGB_BLOCK
            | Self::ASTC_12X10_SFLOAT_BLOCK => FormatClass::Astc12x10,
            Self::ASTC_12X12_UNORM_BLOCK
            | Self::ASTC_12X12_SRGB_BLOCK
            | Self::ASTC_12X12_SFLOAT_BLOCK => FormatClass::Astc12x12,
            Self::ASTC_4X4_UNORM_BLOCK
            | Self::ASTC_4X4_SRGB_BLOCK
            | Self::ASTC_4X4_SFLOAT_BLOCK => FormatClass::Astc4x4,
            Self::ASTC_5X4_UNORM_BLOCK
            | Self::ASTC_5X4_SRGB_BLOCK
            | Self::ASTC_5X4_SFLOAT_BLOCK => FormatClass::Astc5x4,
            Self::ASTC_5X5_UNORM_BLOCK
            | Self::ASTC_5X5_SRGB_BLOCK
            | Self::ASTC_5X5_SFLOAT_BLOCK => FormatClass::Astc5x5,
            Self::ASTC_6X5_UNORM_BLOCK
            | Self::ASTC_6X5_SRGB_BLOCK
            | Self::ASTC_6X5_SFLOAT_BLOCK => FormatClass::Astc6x5,
            Self::ASTC_6X6_UNORM_BLOCK
            | Self::ASTC_6X6_SRGB_BLOCK
            | Self::ASTC_6X6_SFLOAT_BLOCK => FormatClass::Astc6x6,
            Self::ASTC_8X5_UNORM_BLOCK
            | Self::ASTC_8X5_SRGB_BLOCK
            | Self::ASTC_8X5_SFLOAT_BLOCK => FormatClass::Astc8x5,
            Self::ASTC_8X6_UNORM_BLOCK
            | Self::ASTC_8X6_SRGB_BLOCK
            | Self::ASTC_8X6_SFLOAT_BLOCK => FormatClass::Astc8x6,
            Self::ASTC_8X8_UNORM_BLOCK
            | Self::ASTC_8X8_SRGB_BLOCK
            | Self::ASTC_8X8_SFLOAT_BLOCK => FormatClass::Astc8x8,
            Self::BC1_RGB_UNORM_BLOCK | Self::BC1_RGB_SRGB_BLOCK => FormatClass::Bc1Rgb,
            Self::BC1_RGBA_UNORM_BLOCK | Self::BC1_RGBA_SRGB_BLOCK => FormatClass::Bc1Rgba,
            Self::BC2_UNORM_BLOCK | Self::BC2_SRGB_BLOCK => FormatClass::Bc2,
            Self::BC3_UNORM_BLOCK | Self::BC3_SRGB_BLOCK => FormatClass::Bc3,
            Self::BC4_UNORM_BLOCK | Self::BC4_SNORM_BLOCK => FormatClass::Bc4,
            Self::BC5_UNORM_BLOCK | Self::BC5_SNORM_BLOCK => FormatClass::Bc5,
            Self::BC6H_UFLOAT_BLOCK | Self::BC6H_SFLOAT_BLOCK => FormatClass::Bc6h,
            Self::BC7_UNORM_BLOCK | Self::BC7_SRGB_BLOCK => FormatClass::Bc7,
            Self::D16_UNORM => FormatClass::D16,
            Self::D16_UNORM_S8_UINT => FormatClass::D16s8,
            Self::X8_D24_UNORM_PACK32 => FormatClass::D24,
            Self::D24_UNORM_S8_UINT => FormatClass::D24s8,
            Self::D32_SFLOAT => FormatClass::D32,
            Self::D32_SFLOAT_S8_UINT => FormatClass::D32s8,
            Self::EAC_R11_UNORM_BLOCK | Self::EAC_R11_SNORM_BLOCK => FormatClass::EacR,
            Self::EAC_R11G11_UNORM_BLOCK | Self::EAC_R11G11_SNORM_BLOCK => FormatClass::EacRg,
            Self::ETC2_R8G8B8A8_UNORM_BLOCK | Self::ETC2_R8G8B8A8_SRGB_BLOCK => {
                FormatClass::Etc2EacRgba
            }
            Self::ETC2_R8G8B8_UNORM_BLOCK | Self::ETC2_R8G8B8_SRGB_BLOCK => FormatClass::Etc2Rgb,
            Self::ETC2_R8G8B8A1_UNORM_BLOCK | Self::ETC2_R8G8B8A1_SRGB_BLOCK => {
                FormatClass::Etc2Rgba
            }
            Self::PVRTC1_2BPP_UNORM_BLOCK_IMG | Self::PVRTC1_2BPP_SRGB_BLOCK_IMG => {
                FormatClass::Pvrtc12bpp
            }
            Self::PVRTC1_4BPP_UNORM_BLOCK_IMG | Self::PVRTC1_4BPP_SRGB_BLOCK_IMG => {
                FormatClass::Pvrtc14bpp
            }
            Self::PVRTC2_2BPP_UNORM_BLOCK_IMG | Self::PVRTC2_2BPP_SRGB_BLOCK_IMG => {
                FormatClass::Pvrtc22bpp
            }
            Self::PVRTC2_4BPP_UNORM_BLOCK_IMG | Self::PVRTC2_4BPP_SRGB_BLOCK_IMG => {
                FormatClass::Pvrtc24bpp
            }
            Self::S8_UINT => FormatClass::S8,
            _ => panic!("Unknown format vk::Format({:?})", self.as_raw()),
        }
    }
    fn chroma(self) -> Option<Chroma> {
        match self {
            Self::G8_B8_R8_3PLANE_420_UNORM
            | Self::G8_B8R8_2PLANE_420_UNORM
            | Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
            | Self::G16_B16_R16_3PLANE_420_UNORM
            | Self::G16_B16R16_2PLANE_420_UNORM => Some(Chroma::Chroma420),
            Self::G8B8G8R8_422_UNORM
            | Self::B8G8R8G8_422_UNORM
            | Self::G8_B8_R8_3PLANE_422_UNORM
            | Self::G8_B8R8_2PLANE_422_UNORM
            | Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16
            | Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
            | Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16
            | Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
            | Self::G16B16G16R16_422_UNORM
            | Self::B16G16R16G16_422_UNORM
            | Self::G16_B16_R16_3PLANE_422_UNORM
            | Self::G16_B16R16_2PLANE_422_UNORM => Some(Chroma::Chroma422),
            Self::G8_B8_R8_3PLANE_444_UNORM
            | Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16
            | Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16
            | Self::G16_B16_R16_3PLANE_444_UNORM
            | Self::G8_B8R8_2PLANE_444_UNORM
            | Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16
            | Self::G16_B16R16_2PLANE_444_UNORM => Some(Chroma::Chroma444),
            _ => None,
        }
    }
    fn compression(self) -> Option<Compression> {
        match self {
            Self::BC1_RGB_UNORM_BLOCK
            | Self::BC1_RGB_SRGB_BLOCK
            | Self::BC1_RGBA_UNORM_BLOCK
            | Self::BC1_RGBA_SRGB_BLOCK
            | Self::BC2_UNORM_BLOCK
            | Self::BC2_SRGB_BLOCK
            | Self::BC3_UNORM_BLOCK
            | Self::BC3_SRGB_BLOCK
            | Self::BC4_UNORM_BLOCK
            | Self::BC4_SNORM_BLOCK
            | Self::BC5_UNORM_BLOCK
            | Self::BC5_SNORM_BLOCK
            | Self::BC6H_UFLOAT_BLOCK
            | Self::BC6H_SFLOAT_BLOCK
            | Self::BC7_UNORM_BLOCK
            | Self::BC7_SRGB_BLOCK => Some(Compression::Bc),
            Self::ETC2_R8G8B8_UNORM_BLOCK
            | Self::ETC2_R8G8B8_SRGB_BLOCK
            | Self::ETC2_R8G8B8A1_UNORM_BLOCK
            | Self::ETC2_R8G8B8A1_SRGB_BLOCK
            | Self::ETC2_R8G8B8A8_UNORM_BLOCK
            | Self::ETC2_R8G8B8A8_SRGB_BLOCK => Some(Compression::Etc2),
            Self::EAC_R11_UNORM_BLOCK
            | Self::EAC_R11_SNORM_BLOCK
            | Self::EAC_R11G11_UNORM_BLOCK
            | Self::EAC_R11G11_SNORM_BLOCK => Some(Compression::Eac),
            Self::ASTC_4X4_UNORM_BLOCK
            | Self::ASTC_4X4_SRGB_BLOCK
            | Self::ASTC_5X4_UNORM_BLOCK
            | Self::ASTC_5X4_SRGB_BLOCK
            | Self::ASTC_5X5_UNORM_BLOCK
            | Self::ASTC_5X5_SRGB_BLOCK
            | Self::ASTC_6X5_UNORM_BLOCK
            | Self::ASTC_6X5_SRGB_BLOCK
            | Self::ASTC_6X6_UNORM_BLOCK
            | Self::ASTC_6X6_SRGB_BLOCK
            | Self::ASTC_8X5_UNORM_BLOCK
            | Self::ASTC_8X5_SRGB_BLOCK
            | Self::ASTC_8X6_UNORM_BLOCK
            | Self::ASTC_8X6_SRGB_BLOCK
            | Self::ASTC_8X8_UNORM_BLOCK
            | Self::ASTC_8X8_SRGB_BLOCK
            | Self::ASTC_10X5_UNORM_BLOCK
            | Self::ASTC_10X5_SRGB_BLOCK
            | Self::ASTC_10X6_UNORM_BLOCK
            | Self::ASTC_10X6_SRGB_BLOCK
            | Self::ASTC_10X8_UNORM_BLOCK
            | Self::ASTC_10X8_SRGB_BLOCK
            | Self::ASTC_10X10_UNORM_BLOCK
            | Self::ASTC_10X10_SRGB_BLOCK
            | Self::ASTC_12X10_UNORM_BLOCK
            | Self::ASTC_12X10_SRGB_BLOCK
            | Self::ASTC_12X12_UNORM_BLOCK
            | Self::ASTC_12X12_SRGB_BLOCK => Some(Compression::AstcLdr),
            Self::ASTC_4X4_SFLOAT_BLOCK
            | Self::ASTC_5X4_SFLOAT_BLOCK
            | Self::ASTC_5X5_SFLOAT_BLOCK
            | Self::ASTC_6X5_SFLOAT_BLOCK
            | Self::ASTC_6X6_SFLOAT_BLOCK
            | Self::ASTC_8X5_SFLOAT_BLOCK
            | Self::ASTC_8X6_SFLOAT_BLOCK
            | Self::ASTC_8X8_SFLOAT_BLOCK
            | Self::ASTC_10X5_SFLOAT_BLOCK
            | Self::ASTC_10X6_SFLOAT_BLOCK
            | Self::ASTC_10X8_SFLOAT_BLOCK
            | Self::ASTC_10X10_SFLOAT_BLOCK
            | Self::ASTC_12X10_SFLOAT_BLOCK
            | Self::ASTC_12X12_SFLOAT_BLOCK => Some(Compression::AstcHdr),
            Self::PVRTC1_2BPP_UNORM_BLOCK_IMG
            | Self::PVRTC1_4BPP_UNORM_BLOCK_IMG
            | Self::PVRTC2_2BPP_UNORM_BLOCK_IMG
            | Self::PVRTC2_4BPP_UNORM_BLOCK_IMG
            | Self::PVRTC1_2BPP_SRGB_BLOCK_IMG
            | Self::PVRTC1_4BPP_SRGB_BLOCK_IMG
            | Self::PVRTC2_2BPP_SRGB_BLOCK_IMG
            | Self::PVRTC2_4BPP_SRGB_BLOCK_IMG => Some(Compression::Pvrtc),
            _ => None,
        }
    }
    fn components(self) -> &'static [Component] {
        match self {
            Self::R8_UINT => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::UInt,
                bits: 8u8,
                plane_index: None,
            }],
            Self::R8G8_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8A8_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R16_UINT => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::UInt,
                bits: 16u8,
                plane_index: None,
            }],
            Self::R16G16_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16A16_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UInt,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R32_UINT => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::UInt,
                bits: 32u8,
                plane_index: None,
            }],
            Self::R32G32_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 32u8,
                    plane_index: None,
                },
            ],
            Self::R32G32B32_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 32u8,
                    plane_index: None,
                },
            ],
            Self::R32G32B32A32_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UInt,
                    bits: 32u8,
                    plane_index: None,
                },
            ],
            Self::R64_UINT => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::UInt,
                bits: 64u8,
                plane_index: None,
            }],
            Self::R64G64_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 64u8,
                    plane_index: None,
                },
            ],
            Self::R64G64B64_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 64u8,
                    plane_index: None,
                },
            ],
            Self::R64G64B64A64_UINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UInt,
                    bits: 64u8,
                    plane_index: None,
                },
            ],
            Self::R8_SINT => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SInt,
                bits: 8u8,
                plane_index: None,
            }],
            Self::R8G8_SINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8_SINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8A8_SINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R16_SINT => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SInt,
                bits: 16u8,
                plane_index: None,
            }],
            Self::R16G16_SINT | Self::R16G16_S10_5_NV => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16_SINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16A16_SINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SInt,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R32_SINT => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SInt,
                bits: 32u8,
                plane_index: None,
            }],
            Self::R32G32_SINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 32u8,
                    plane_index: None,
                },
            ],
            Self::R32G32B32_SINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 32u8,
                    plane_index: None,
                },
            ],
            Self::R32G32B32A32_SINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SInt,
                    bits: 32u8,
                    plane_index: None,
                },
            ],
            Self::R64_SINT => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SInt,
                bits: 64u8,
                plane_index: None,
            }],
            Self::R64G64_SINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 64u8,
                    plane_index: None,
                },
            ],
            Self::R64G64B64_SINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 64u8,
                    plane_index: None,
                },
            ],
            Self::R64G64B64A64_SINT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SInt,
                    bits: 64u8,
                    plane_index: None,
                },
            ],
            Self::R4G4_UNORM_PACK8 => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
            ],
            Self::R4G4B4A4_UNORM_PACK16 => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
            ],
            Self::R5G5B5A1_UNORM_PACK16 => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 1u8,
                    plane_index: None,
                },
            ],
            Self::R5G6B5_UNORM_PACK16 => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 6u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
            ],
            Self::R8_UNORM => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::UNorm,
                bits: 8u8,
                plane_index: None,
            }],
            Self::R8G8_UNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8_UNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8A8_UNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R10X6_UNORM_PACK16 => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::UNorm,
                bits: 10u8,
                plane_index: None,
            }],
            Self::R10X6G10X6_UNORM_2PACK16 => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::EAC_R11_UNORM_BLOCK => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::UNorm,
                bits: 11u8,
                plane_index: None,
            }],
            Self::EAC_R11G11_UNORM_BLOCK => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 11u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 11u8,
                    plane_index: None,
                },
            ],
            Self::R12X4_UNORM_PACK16 => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::UNorm,
                bits: 12u8,
                plane_index: None,
            }],
            Self::R12X4G12X4_UNORM_2PACK16 => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
            ],
            Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
            ],
            Self::R16_UNORM => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::UNorm,
                bits: 16u8,
                plane_index: None,
            }],
            Self::R16G16_UNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16_UNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16A16_UNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R8_SNORM => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SNorm,
                bits: 8u8,
                plane_index: None,
            }],
            Self::R8G8_SNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8_SNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8A8_SNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::EAC_R11_SNORM_BLOCK => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SNorm,
                bits: 11u8,
                plane_index: None,
            }],
            Self::EAC_R11G11_SNORM_BLOCK => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 11u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 11u8,
                    plane_index: None,
                },
            ],
            Self::R16_SNORM => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SNorm,
                bits: 16u8,
                plane_index: None,
            }],
            Self::R16G16_SNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16_SNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SNorm,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16A16_SNORM => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SNorm,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R8_SRGB => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SRgb,
                bits: 8u8,
                plane_index: None,
            }],
            Self::R8G8_SRGB => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8_SRGB => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8A8_SRGB => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8_USCALED => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::UScaled,
                bits: 8u8,
                plane_index: None,
            }],
            Self::R8G8_USCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8_USCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8A8_USCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R16_USCALED => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::UScaled,
                bits: 16u8,
                plane_index: None,
            }],
            Self::R16G16_USCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UScaled,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16_USCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UScaled,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16A16_USCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UScaled,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R8_SSCALED => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SScaled,
                bits: 8u8,
                plane_index: None,
            }],
            Self::R8G8_SSCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8_SSCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R8G8B8A8_SSCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::R16_SSCALED => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SScaled,
                bits: 16u8,
                plane_index: None,
            }],
            Self::R16G16_SSCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SScaled,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16_SSCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SScaled,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16A16_SSCALED => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SScaled,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SScaled,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16_SFLOAT => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SFloat,
                bits: 16u8,
                plane_index: None,
            }],
            Self::R16G16_SFLOAT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SFloat,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SFloat,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16_SFLOAT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SFloat,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SFloat,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SFloat,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R16G16B16A16_SFLOAT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SFloat,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SFloat,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SFloat,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SFloat,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::R32_SFLOAT => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SFloat,
                bits: 32u8,
                plane_index: None,
            }],
            Self::R32G32_SFLOAT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SFloat,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SFloat,
                    bits: 32u8,
                    plane_index: None,
                },
            ],
            Self::R32G32B32_SFLOAT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SFloat,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SFloat,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SFloat,
                    bits: 32u8,
                    plane_index: None,
                },
            ],
            Self::R32G32B32A32_SFLOAT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SFloat,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SFloat,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SFloat,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SFloat,
                    bits: 32u8,
                    plane_index: None,
                },
            ],
            Self::R64_SFLOAT => &[Component {
                name: ComponentName::R,
                numeric_format: NumericFormat::SFloat,
                bits: 64u8,
                plane_index: None,
            }],
            Self::R64G64_SFLOAT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SFloat,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SFloat,
                    bits: 64u8,
                    plane_index: None,
                },
            ],
            Self::R64G64B64_SFLOAT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SFloat,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SFloat,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SFloat,
                    bits: 64u8,
                    plane_index: None,
                },
            ],
            Self::R64G64B64A64_SFLOAT => &[
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SFloat,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SFloat,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SFloat,
                    bits: 64u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SFloat,
                    bits: 64u8,
                    plane_index: None,
                },
            ],
            Self::G8B8G8R8_422_UNORM => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::G8_B8R8_2PLANE_420_UNORM
            | Self::G8_B8R8_2PLANE_422_UNORM
            | Self::G8_B8R8_2PLANE_444_UNORM => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: Some(0u8),
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: Some(1u8),
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: Some(1u8),
                },
            ],
            Self::G8_B8_R8_3PLANE_420_UNORM
            | Self::G8_B8_R8_3PLANE_422_UNORM
            | Self::G8_B8_R8_3PLANE_444_UNORM => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: Some(0u8),
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: Some(1u8),
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: Some(2u8),
                },
            ],
            Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: Some(0u8),
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: Some(1u8),
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: Some(1u8),
                },
            ],
            Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: Some(0u8),
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: Some(1u8),
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: Some(2u8),
                },
            ],
            Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
            ],
            Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: Some(0u8),
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: Some(1u8),
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: Some(1u8),
                },
            ],
            Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: Some(0u8),
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: Some(1u8),
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: Some(2u8),
                },
            ],
            Self::G16B16G16R16_422_UNORM => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::G16_B16R16_2PLANE_420_UNORM
            | Self::G16_B16R16_2PLANE_422_UNORM
            | Self::G16_B16R16_2PLANE_444_UNORM => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: Some(0u8),
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: Some(1u8),
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: Some(1u8),
                },
            ],
            Self::G16_B16_R16_3PLANE_420_UNORM
            | Self::G16_B16_R16_3PLANE_422_UNORM
            | Self::G16_B16_R16_3PLANE_444_UNORM => &[
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: Some(0u8),
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: Some(1u8),
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: Some(2u8),
                },
            ],
            Self::B8G8R8_UINT => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8A8_UINT => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8_SINT => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8A8_SINT => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B4G4R4A4_UNORM_PACK16 => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
            ],
            Self::B5G5R5A1_UNORM_PACK16 => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 1u8,
                    plane_index: None,
                },
            ],
            Self::B5G6R5_UNORM_PACK16 => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 6u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8_UNORM => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8G8_422_UNORM => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8A8_UNORM => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 12u8,
                    plane_index: None,
                },
            ],
            Self::B16G16R16G16_422_UNORM => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8_SNORM => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8A8_SNORM => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8_SRGB => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8A8_SRGB => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8_USCALED => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8A8_USCALED => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8_SSCALED => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::B8G8R8A8_SSCALED => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::E5B9G9R9_UFLOAT_PACK32 => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UFloat,
                    bits: 9u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UFloat,
                    bits: 9u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UFloat,
                    bits: 9u8,
                    plane_index: None,
                },
            ],
            Self::B10G11R11_UFLOAT_PACK32 => &[
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UFloat,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UFloat,
                    bits: 11u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UFloat,
                    bits: 11u8,
                    plane_index: None,
                },
            ],
            Self::A2R10G10B10_UINT_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UInt,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A2B10G10R10_UINT_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UInt,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A8B8G8R8_UINT_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::A2R10G10B10_SINT_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SInt,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A2B10G10R10_SINT_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SInt,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A8B8G8R8_SINT_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::A1R5G5B5_UNORM_PACK16 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 1u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
            ],
            Self::A1B5G5R5_UNORM_PACK16_KHR => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 1u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 5u8,
                    plane_index: None,
                },
            ],
            Self::A2R10G10B10_UNORM_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A2B10G10R10_UNORM_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A4R4G4B4_UNORM_PACK16 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
            ],
            Self::A4B4G4R4_UNORM_PACK16 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 4u8,
                    plane_index: None,
                },
            ],
            Self::A8_UNORM_KHR => &[Component {
                name: ComponentName::A,
                numeric_format: NumericFormat::UNorm,
                bits: 8u8,
                plane_index: None,
            }],
            Self::A8B8G8R8_UNORM_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::A2R10G10B10_SNORM_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SNorm,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SNorm,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A2B10G10R10_SNORM_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SNorm,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A8B8G8R8_SNORM_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SNorm,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::A8B8G8R8_SRGB_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SRgb,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::A2R10G10B10_USCALED_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UScaled,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UScaled,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UScaled,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UScaled,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A2B10G10R10_USCALED_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UScaled,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UScaled,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UScaled,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UScaled,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A8B8G8R8_USCALED_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::UScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::A2R10G10B10_SSCALED_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SScaled,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SScaled,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SScaled,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SScaled,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A2B10G10R10_SSCALED_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SScaled,
                    bits: 2u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SScaled,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SScaled,
                    bits: 10u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SScaled,
                    bits: 10u8,
                    plane_index: None,
                },
            ],
            Self::A8B8G8R8_SSCALED_PACK32 => &[
                Component {
                    name: ComponentName::A,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::B,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::G,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::R,
                    numeric_format: NumericFormat::SScaled,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::D16_UNORM => &[Component {
                name: ComponentName::D,
                numeric_format: NumericFormat::UNorm,
                bits: 16u8,
                plane_index: None,
            }],
            Self::D16_UNORM_S8_UINT => &[
                Component {
                    name: ComponentName::D,
                    numeric_format: NumericFormat::UNorm,
                    bits: 16u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::S,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::X8_D24_UNORM_PACK32 => &[Component {
                name: ComponentName::D,
                numeric_format: NumericFormat::UNorm,
                bits: 24u8,
                plane_index: None,
            }],
            Self::D24_UNORM_S8_UINT => &[
                Component {
                    name: ComponentName::D,
                    numeric_format: NumericFormat::UNorm,
                    bits: 24u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::S,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::D32_SFLOAT => &[Component {
                name: ComponentName::D,
                numeric_format: NumericFormat::SFloat,
                bits: 32u8,
                plane_index: None,
            }],
            Self::D32_SFLOAT_S8_UINT => &[
                Component {
                    name: ComponentName::D,
                    numeric_format: NumericFormat::SFloat,
                    bits: 32u8,
                    plane_index: None,
                },
                Component {
                    name: ComponentName::S,
                    numeric_format: NumericFormat::UInt,
                    bits: 8u8,
                    plane_index: None,
                },
            ],
            Self::S8_UINT => &[Component {
                name: ComponentName::S,
                numeric_format: NumericFormat::UInt,
                bits: 8u8,
                plane_index: None,
            }],
            _ => panic!("Unknown format vk::Format({:?})", self.as_raw()),
        }
    }
    fn planes(self) -> &'static [Plane] {
        match self {
            Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6G10X6_UNORM_2PACK16,
                },
            ],
            Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
            ],
            Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6G10X6_UNORM_2PACK16,
                },
            ],
            Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
            ],
            Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R10X6G10X6_UNORM_2PACK16,
                },
            ],
            Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R10X6_UNORM_PACK16,
                },
            ],
            Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4G12X4_UNORM_2PACK16,
                },
            ],
            Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
            ],
            Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4G12X4_UNORM_2PACK16,
                },
            ],
            Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
            ],
            Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R12X4G12X4_UNORM_2PACK16,
                },
            ],
            Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R12X4_UNORM_PACK16,
                },
            ],
            Self::G16_B16R16_2PLANE_444_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16_UNORM,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16G16_UNORM,
                },
            ],
            Self::G16_B16_R16_3PLANE_444_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16_UNORM,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16_UNORM,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16_UNORM,
                },
            ],
            Self::G16_B16R16_2PLANE_422_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16G16_UNORM,
                },
            ],
            Self::G16_B16_R16_3PLANE_422_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16_UNORM,
                },
            ],
            Self::G16_B16R16_2PLANE_420_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R16G16_UNORM,
                },
            ],
            Self::G16_B16_R16_3PLANE_420_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R16_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R16_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R16_UNORM,
                },
            ],
            Self::G8_B8R8_2PLANE_444_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8_UNORM,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8G8_UNORM,
                },
            ],
            Self::G8_B8_R8_3PLANE_444_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8_UNORM,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8_UNORM,
                },
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8_UNORM,
                },
            ],
            Self::G8_B8R8_2PLANE_422_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8G8_UNORM,
                },
            ],
            Self::G8_B8_R8_3PLANE_422_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8_UNORM,
                },
            ],
            Self::G8_B8R8_2PLANE_420_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R8G8_UNORM,
                },
            ],
            Self::G8_B8_R8_3PLANE_420_UNORM => &[
                Plane {
                    width_divisor: 1u8,
                    height_divisor: 1u8,
                    compatible_format: Format::R8_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R8_UNORM,
                },
                Plane {
                    width_divisor: 2u8,
                    height_divisor: 2u8,
                    compatible_format: Format::R8_UNORM,
                },
            ],
            _ => &[],
        }
    }
    fn aspect_flags(self) -> ImageAspectFlags {
        match self {
            Self::R4G4_UNORM_PACK8
            | Self::R4G4B4A4_UNORM_PACK16
            | Self::B4G4R4A4_UNORM_PACK16
            | Self::R5G6B5_UNORM_PACK16
            | Self::B5G6R5_UNORM_PACK16
            | Self::R5G5B5A1_UNORM_PACK16
            | Self::B5G5R5A1_UNORM_PACK16
            | Self::A1R5G5B5_UNORM_PACK16
            | Self::A1B5G5R5_UNORM_PACK16_KHR
            | Self::A8_UNORM_KHR
            | Self::R8_UNORM
            | Self::R8_SNORM
            | Self::R8_USCALED
            | Self::R8_SSCALED
            | Self::R8_UINT
            | Self::R8_SINT
            | Self::R8_SRGB
            | Self::R8G8_UNORM
            | Self::R8G8_SNORM
            | Self::R8G8_USCALED
            | Self::R8G8_SSCALED
            | Self::R8G8_UINT
            | Self::R8G8_SINT
            | Self::R8G8_SRGB
            | Self::R8G8B8_UNORM
            | Self::R8G8B8_SNORM
            | Self::R8G8B8_USCALED
            | Self::R8G8B8_SSCALED
            | Self::R8G8B8_UINT
            | Self::R8G8B8_SINT
            | Self::R8G8B8_SRGB
            | Self::B8G8R8_UNORM
            | Self::B8G8R8_SNORM
            | Self::B8G8R8_USCALED
            | Self::B8G8R8_SSCALED
            | Self::B8G8R8_UINT
            | Self::B8G8R8_SINT
            | Self::B8G8R8_SRGB
            | Self::R8G8B8A8_UNORM
            | Self::R8G8B8A8_SNORM
            | Self::R8G8B8A8_USCALED
            | Self::R8G8B8A8_SSCALED
            | Self::R8G8B8A8_UINT
            | Self::R8G8B8A8_SINT
            | Self::R8G8B8A8_SRGB
            | Self::B8G8R8A8_UNORM
            | Self::B8G8R8A8_SNORM
            | Self::B8G8R8A8_USCALED
            | Self::B8G8R8A8_SSCALED
            | Self::B8G8R8A8_UINT
            | Self::B8G8R8A8_SINT
            | Self::B8G8R8A8_SRGB
            | Self::A8B8G8R8_UNORM_PACK32
            | Self::A8B8G8R8_SNORM_PACK32
            | Self::A8B8G8R8_USCALED_PACK32
            | Self::A8B8G8R8_SSCALED_PACK32
            | Self::A8B8G8R8_UINT_PACK32
            | Self::A8B8G8R8_SINT_PACK32
            | Self::A8B8G8R8_SRGB_PACK32
            | Self::A2R10G10B10_UNORM_PACK32
            | Self::A2R10G10B10_SNORM_PACK32
            | Self::A2R10G10B10_USCALED_PACK32
            | Self::A2R10G10B10_SSCALED_PACK32
            | Self::A2R10G10B10_UINT_PACK32
            | Self::A2R10G10B10_SINT_PACK32
            | Self::A2B10G10R10_UNORM_PACK32
            | Self::A2B10G10R10_SNORM_PACK32
            | Self::A2B10G10R10_USCALED_PACK32
            | Self::A2B10G10R10_SSCALED_PACK32
            | Self::A2B10G10R10_UINT_PACK32
            | Self::A2B10G10R10_SINT_PACK32
            | Self::R16_UNORM
            | Self::R16_SNORM
            | Self::R16_USCALED
            | Self::R16_SSCALED
            | Self::R16_UINT
            | Self::R16_SINT
            | Self::R16_SFLOAT
            | Self::R16G16_UNORM
            | Self::R16G16_SNORM
            | Self::R16G16_USCALED
            | Self::R16G16_SSCALED
            | Self::R16G16_UINT
            | Self::R16G16_SINT
            | Self::R16G16_SFLOAT
            | Self::R16G16B16_UNORM
            | Self::R16G16B16_SNORM
            | Self::R16G16B16_USCALED
            | Self::R16G16B16_SSCALED
            | Self::R16G16B16_UINT
            | Self::R16G16B16_SINT
            | Self::R16G16B16_SFLOAT
            | Self::R16G16B16A16_UNORM
            | Self::R16G16B16A16_SNORM
            | Self::R16G16B16A16_USCALED
            | Self::R16G16B16A16_SSCALED
            | Self::R16G16B16A16_UINT
            | Self::R16G16B16A16_SINT
            | Self::R16G16B16A16_SFLOAT
            | Self::R32_UINT
            | Self::R32_SINT
            | Self::R32_SFLOAT
            | Self::R32G32_UINT
            | Self::R32G32_SINT
            | Self::R32G32_SFLOAT
            | Self::R32G32B32_UINT
            | Self::R32G32B32_SINT
            | Self::R32G32B32_SFLOAT
            | Self::R32G32B32A32_UINT
            | Self::R32G32B32A32_SINT
            | Self::R32G32B32A32_SFLOAT
            | Self::R64_UINT
            | Self::R64_SINT
            | Self::R64_SFLOAT
            | Self::R64G64_UINT
            | Self::R64G64_SINT
            | Self::R64G64_SFLOAT
            | Self::R64G64B64_UINT
            | Self::R64G64B64_SINT
            | Self::R64G64B64_SFLOAT
            | Self::R64G64B64A64_UINT
            | Self::R64G64B64A64_SINT
            | Self::R64G64B64A64_SFLOAT
            | Self::B10G11R11_UFLOAT_PACK32
            | Self::E5B9G9R9_UFLOAT_PACK32
            | Self::EAC_R11_UNORM_BLOCK
            | Self::EAC_R11_SNORM_BLOCK
            | Self::EAC_R11G11_UNORM_BLOCK
            | Self::EAC_R11G11_SNORM_BLOCK
            | Self::G8B8G8R8_422_UNORM
            | Self::B8G8R8G8_422_UNORM
            | Self::R10X6_UNORM_PACK16
            | Self::R10X6G10X6_UNORM_2PACK16
            | Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16
            | Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16
            | Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16
            | Self::R12X4_UNORM_PACK16
            | Self::R12X4G12X4_UNORM_2PACK16
            | Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16
            | Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16
            | Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16
            | Self::G16B16G16R16_422_UNORM
            | Self::B16G16R16G16_422_UNORM
            | Self::A4R4G4B4_UNORM_PACK16
            | Self::A4B4G4R4_UNORM_PACK16
            | Self::R16G16_S10_5_NV => ImageAspectFlags::COLOR,
            Self::D16_UNORM | Self::X8_D24_UNORM_PACK32 | Self::D32_SFLOAT => {
                ImageAspectFlags::DEPTH
            }
            Self::S8_UINT => ImageAspectFlags::STENCIL,
            Self::D16_UNORM_S8_UINT | Self::D24_UNORM_S8_UINT | Self::D32_SFLOAT_S8_UINT => {
                ImageAspectFlags::DEPTH | ImageAspectFlags::STENCIL
            }
            Self::G8_B8R8_2PLANE_420_UNORM
            | Self::G8_B8R8_2PLANE_422_UNORM
            | Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
            | Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
            | Self::G16_B16R16_2PLANE_420_UNORM
            | Self::G16_B16R16_2PLANE_422_UNORM
            | Self::G8_B8R8_2PLANE_444_UNORM
            | Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16
            | Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16
            | Self::G16_B16R16_2PLANE_444_UNORM => {
                ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1
            }
            Self::G8_B8_R8_3PLANE_420_UNORM
            | Self::G8_B8_R8_3PLANE_422_UNORM
            | Self::G8_B8_R8_3PLANE_444_UNORM
            | Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
            | Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
            | Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16
            | Self::G16_B16_R16_3PLANE_420_UNORM
            | Self::G16_B16_R16_3PLANE_422_UNORM
            | Self::G16_B16_R16_3PLANE_444_UNORM => {
                ImageAspectFlags::COLOR
                    | ImageAspectFlags::PLANE_0
                    | ImageAspectFlags::PLANE_1
                    | ImageAspectFlags::PLANE_2
            }
            _ => panic!("Unknown format vk::Format({:?})", self.as_raw()),
        }
    }
}
