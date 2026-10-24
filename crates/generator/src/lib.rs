use std::{collections::BTreeMap, path::Path, str::FromStr};

use heck::{ToSnakeCase, ToUpperCamelCase};
use quote::{ToTokens, format_ident, quote};

macro_rules! get_variant {
    ($variant:path) => {
        |enum_| match enum_ {
            $variant(inner) => Some(inner),
            _ => None,
        }
    };
    ($variant:path { $($member:ident),+ }) => {
        |enum_| match enum_ {
            #[allow(clippy::double_parens, reason = "Occurs when only one member is queried")]
            $variant { $($member),+, .. } => Some(( $($member),+ )),
            _ => None,
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Chroma {
    Chroma420,
    Chroma422,
    Chroma444,
}

impl FromStr for Chroma {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "420" => Ok(Self::Chroma420),
            "422" => Ok(Self::Chroma422),
            "444" => Ok(Self::Chroma444),
            _ => Err(()),
        }
    }
}

impl ToTokens for Chroma {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = match self {
            Self::Chroma420 => format_ident!("Chroma420"),
            Self::Chroma422 => format_ident!("Chroma422"),
            Self::Chroma444 => format_ident!("Chroma444"),
        };
        ident.to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Compression {
    Bc,
    Etc2,
    Eac,
    AstcLdr,
    AstcHdr,
    Pvrtc,
}

impl FromStr for Compression {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BC" => Ok(Self::Bc),
            "ETC2" => Ok(Self::Etc2),
            "EAC" => Ok(Self::Eac),
            "ASTC LDR" => Ok(Self::AstcLdr),
            "ASTC HDR" => Ok(Self::AstcHdr),
            "PVRTC" => Ok(Self::Pvrtc),
            _ => Err(()),
        }
    }
}

impl ToTokens for Compression {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = match self {
            Self::Bc => format_ident!("Bc"),
            Self::Etc2 => format_ident!("Etc2"),
            Self::Eac => format_ident!("Eac"),
            Self::AstcLdr => format_ident!("AstcLdr"),
            Self::AstcHdr => format_ident!("AstcHdr"),
            Self::Pvrtc => format_ident!("Pvrtc"),
        };
        ident.to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NumericFormat {
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

impl FromStr for NumericFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UINT" => Ok(Self::UInt),
            "SINT" => Ok(Self::SInt),
            "UNORM" => Ok(Self::UNorm),
            "SNORM" => Ok(Self::SNorm),
            "SRGB" => Ok(Self::SRgb),
            "USCALED" => Ok(Self::UScaled),
            "SSCALED" => Ok(Self::SScaled),
            "SFLOAT" => Ok(Self::SFloat),
            "UFLOAT" => Ok(Self::UFloat),
            _ => Err(()),
        }
    }
}

impl ToTokens for NumericFormat {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = match self {
            Self::UInt => format_ident!("UInt"),
            Self::SInt => format_ident!("SInt"),
            Self::UNorm => format_ident!("UNorm"),
            Self::SNorm => format_ident!("SNorm"),
            Self::SRgb => format_ident!("SRgb"),
            Self::UScaled => format_ident!("UScaled"),
            Self::SScaled => format_ident!("SScaled"),
            Self::SFloat => format_ident!("SFloat"),
            Self::UFloat => format_ident!("UFloat"),
        };
        ident.to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ComponentName {
    R,
    G,
    B,
    A,
    D,
    S,
}

impl FromStr for ComponentName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::R),
            "G" => Ok(Self::G),
            "B" => Ok(Self::B),
            "A" => Ok(Self::A),
            "D" => Ok(Self::D),
            "S" => Ok(Self::S),
            _ => Err(()),
        }
    }
}

impl ToTokens for ComponentName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = match self {
            Self::R => format_ident!("R"),
            Self::G => format_ident!("G"),
            Self::B => format_ident!("B"),
            Self::A => format_ident!("A"),
            Self::D => format_ident!("D"),
            Self::S => format_ident!("S"),
        };
        ident.to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Component {
    name: ComponentName,
    numeric_format: NumericFormat,
    bits: u8,
    plane_index: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Plane<'a> {
    index: u8,
    width_divisor: u8,
    height_divisor: u8,
    compatible_format: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct AspectMask(u8);

impl core::ops::BitOr for AspectMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for AspectMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl From<ComponentName> for AspectMask {
    fn from(name: ComponentName) -> Self {
        match name {
            ComponentName::R | ComponentName::G | ComponentName::B | ComponentName::A => {
                Self::COLOR
            }
            ComponentName::D => Self::DEPTH,
            ComponentName::S => Self::STENCIL,
        }
    }
}

impl AspectMask {
    const COLOR: Self = Self(0b0000_0001);
    const DEPTH: Self = Self(0b0000_0010);
    const STENCIL: Self = Self(0b0000_0100);
    const PLANE_0: Self = Self(0b0000_1000);
    const PLANE_1: Self = Self(0b0001_0000);
    const PLANE_2: Self = Self(0b0010_0000);

    fn is_empty(self) -> bool {
        self.0 == 0
    }

    fn plane(i: u8) -> Self {
        match i {
            0 => Self::PLANE_0,
            1 => Self::PLANE_1,
            2 => Self::PLANE_2,
            _ => panic!("Invalid plane index: {i}"),
        }
    }

    fn iter(self) -> impl Iterator<Item = Self> {
        struct BitIter {
            bits: u8,
            pos: u8,
        }
        impl Iterator for BitIter {
            type Item = u8;
            fn next(&mut self) -> Option<Self::Item> {
                if self.pos >= u8::BITS as u8 || self.bits == 0 {
                    None
                } else {
                    let step = self.bits.trailing_zeros() as u8 + 1;
                    self.bits >>= step;
                    self.pos += step;
                    Some(1 << (self.pos - 1))
                }
            }
        }

        BitIter {
            bits: self.0,
            pos: 0,
        }
        .map(AspectMask)
    }
}

pub fn write_source_code<P: AsRef<Path>>(vk_headers_dir: &Path, src_dir: P) {
    let vk_xml = vk_headers_dir.join("registry/vk.xml");
    use std::fs::File;
    use std::io::Write;
    let (spec2, errors) = vk_parse::parse_file(&vk_xml).expect("Invalid xml file");
    if !errors.is_empty() {
        eprintln!("vk_parse encountered one or more errors while parsing: {errors:?}")
    }

    let formats = spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Formats))
        .flat_map(|formats| &formats.children)
        .collect::<Vec<_>>();

    let mut block_sizes = BTreeMap::<u8, Vec<&str>>::new();
    let mut texels_per_block = BTreeMap::<u8, Vec<&str>>::new();
    let mut packed = BTreeMap::<u8, Vec<&str>>::new();
    let mut chromas = BTreeMap::<Chroma, Vec<&str>>::new();
    let mut compressions = BTreeMap::<Compression, Vec<&str>>::new();
    let mut classes = BTreeMap::<&str, Vec<&str>>::new();
    let mut block_extents = BTreeMap::<(u8, u8, u8), Vec<&str>>::new();

    let mut components = BTreeMap::<Vec<Component>, Vec<&str>>::new();
    let mut planes = BTreeMap::<Vec<Plane>, Vec<&str>>::new();
    let mut aspects = BTreeMap::<AspectMask, Vec<&str>>::new();

    formats.iter().for_each(|format| {
        let name = &format.name;

        block_sizes.entry(format.blockSize).or_default().push(name);
        texels_per_block
            .entry(format.texelsPerBlock)
            .or_default()
            .push(name);

        classes.entry(&format.class).or_default().push(name);

        if let Some(packing) = format.packed {
            packed.entry(packing).or_default().push(name);
        }

        if let Some(chroma) = format
            .chroma
            .as_ref()
            .and_then(|c| Chroma::from_str(c).ok())
        {
            chromas.entry(chroma).or_default().push(name);
        }

        if let Some(compression) = format
            .compressed
            .as_ref()
            .and_then(|c| Compression::from_str(c).ok())
        {
            compressions.entry(compression).or_default().push(name);
        }

        if let Some(block_extent) = &format.blockExtent {
            let split = block_extent.split(',');
            if let Some(&[x, y, z]) = split
                .map(|s| s.trim().parse::<u8>())
                .collect::<Result<Box<[_]>, _>>()
                .ok()
                .as_deref()
            {
                block_extents.entry((x, y, z)).or_default().push(name);
            }
        }

        let mut comps = Vec::new();
        let mut plns = Vec::new();
        let mut aspect_mask = AspectMask(0);
        format.children.iter().for_each(|child| match child {
            vk_parse::FormatChild::Component {
                name,
                bits,
                numericFormat,
                planeIndex,
                ..
            } => {
                if let (Ok(name), Ok(numeric_format), Ok(bits)) = (
                    ComponentName::from_str(name),
                    NumericFormat::from_str(numericFormat),
                    u8::from_str(bits),
                ) {
                    aspect_mask |= AspectMask::from(name);
                    comps.push(Component {
                        name,
                        numeric_format,
                        bits,
                        plane_index: *planeIndex,
                    });
                }
            }
            vk_parse::FormatChild::Plane {
                index,
                widthDivisor,
                heightDivisor,
                compatible,
                ..
            } => {
                aspect_mask |= AspectMask::plane(*index);
                plns.push(Plane {
                    index: *index,
                    width_divisor: *widthDivisor,
                    height_divisor: *heightDivisor,
                    compatible_format: compatible,
                });
            }
            _ => (),
        });

        if !comps.is_empty() {
            components.entry(comps).or_default().push(name);
        }

        if !aspect_mask.is_empty() {
            aspects.entry(aspect_mask).or_default().push(name);
        }

        if !plns.is_empty() {
            plns.sort_by_key(|p| p.index);
            planes.entry(plns).or_default().push(name);
        }
    });

    let format_ext = quote! {
        pub trait FormatExt {
            /// The size of a block of the format in bytes. For uncompressed
            /// formats, this is the size of a texel.
            /// For `Format::UNDEFINED`, this returns `0`.
            fn block_size(self) -> DeviceSize;
            /// The dimensions of a block of the format in texels. For
            /// uncompressed formats, this is always (1, 1, 1).
            /// For `Format::UNDEFINED`, this returns `(1, 1, 1)`.
            fn block_extent(self) -> (u8, u8, u8);
            /// The number of texels in a block of the format. For uncompressed
            /// formats, this is always 1.
            /// For `Format::UNDEFINED`, this returns `0`.
            fn texels_per_block(self) -> u8;
            /// For packed formats, this describes how the components are packed
            /// into byte-multiples.
            /// Together with the block size and component bits, this can be
            /// used to determine the bit offset of each component within a
            /// block.
            /// This follows the same logic as the packing of bit fields in C in GCC/Clang:
            /// Namely, [`Format::R10X6G10X6_UNORM_2PACK16`] has a block
            /// size of 4 bytes, a `packed` of 16 bits, and a two components
            /// [`ComponentName::R`] and [`ComponentName::G`] with 10 bits
            /// each. As the format name suggests, the two channels are each
            /// stored in 16-bit words. Thus, the red channel is stored in the
            /// 10 most significant bits of the first 16-bit word, and the green
            /// channel is stored in the 10 most significant bits of the second
            /// 16-bit word. the 6 least significant bits of each word are
            /// unused.
            ///
            /// ```ignore
            /// fn component_bit_offset(packed: u8, cursor: &mut u16, component_size: u8) -> u16 {
            ///     let remaining_bits = cursor.next_multiple_of(packed) - *cursor;
            ///     if component_size <= remaining_bits {
            ///         let offset = *cursor;
            ///         *cursor += component_size;
            ///         offset
            ///     } else {
            ///         let offset = *cursor.next_multiple_of(packed);
            ///         *cursor = offset + component_size;
            ///         offset
            ///     }
            /// }
            /// ```
            /// For `Format::UNDEFINED`, this returns `None`.
            fn packed(self) -> Option<u8>;
            /// The format class of the format. Formats in the same class are
            /// comatible with each other. Images created with the
            /// [`MUTABLE_FORMAT`](vk::ImageCreateFlags::MUTABLE_FORMAT) flag
            /// may be used to create image views of any format in the same
            /// format class as the image's format, or for multi-planar formats,
            /// image views for specific planes of the image.
            /// Panics for `Format::UNDEFINED`, which has no format class.
            fn format_class(self) -> FormatClass;
            /// The chroma subsampling scheme of the format, if the format is a
            /// chroma subsampled format.
            /// For `Format::UNDEFINED`, this returns `None`.
            fn chroma(self) -> Option<Chroma>;
            /// The compression scheme of the format, if the format is a compressed format.
            /// For `Format::UNDEFINED`, this returns `None`.
            fn compression(self) -> Option<Compression>;
            /// The components (channels) of the format.
            /// For `Format::UNDEFINED`, this returns an empty slice.
            fn components(self) -> &'static [Component];
            /// For multi-planar formats, the planes of the format. For
            /// single-plane formats, this returns an empty slice.
            /// For `Format::UNDEFINED`, this returns an empty slice.
            fn planes(self) -> &'static [Plane];
            /// The aspect flags corresponding to the format. For most formats,
            /// this will simply be [`ImageAspectFlags::COLOR`].
            /// For `Format::UNDEFINED`, this returns empty flags.
            fn aspect_flags(self) -> ImageAspectFlags;
        }
    };

    let aspect_flags = aspects.into_iter().map(|(aspects, formats)| {
        let formats = formats
            .iter()
            .map(|f| generator::variant_ident("VkFormat", f))
            .map(|f| quote! { Self::#f });

        let aspect_flags = aspects.iter().map(|aspect| match aspect {
            AspectMask::COLOR => quote! { ImageAspectFlags::COLOR },
            AspectMask::DEPTH => quote! { ImageAspectFlags::DEPTH },
            AspectMask::STENCIL => quote! { ImageAspectFlags::STENCIL },
            AspectMask::PLANE_0 => quote! { ImageAspectFlags::PLANE_0 },
            AspectMask::PLANE_1 => quote! { ImageAspectFlags::PLANE_1 },
            AspectMask::PLANE_2 => quote! { ImageAspectFlags::PLANE_2 },
            _ => panic!("Invalid aspect mask: {:?}", aspect),
        });

        quote! {
            #( #formats )|* => #( #aspect_flags )|*,
        }
    });

    let planes = planes.into_iter().map(|(planes, formats)| {
        let formats = formats
            .iter()
            .map(|f| generator::variant_ident("VkFormat", f))
            .map(|f| quote! { Self::#f });

        let planes = planes.iter().map(|p| {
            let Plane {
                width_divisor,
                height_divisor,
                compatible_format,
                ..
            } = p;

            let compatible_format = generator::variant_ident("VkFormat", compatible_format);

            quote! {
                Plane {
                    width_divisor: #width_divisor,
                    height_divisor: #height_divisor,
                    compatible_format: Format::#compatible_format,
                }
            }
        });

        quote! {
            #( #formats )|* => &[ #( #planes ),* ],
        }
    });

    let components = components.iter().map(|(components, formats)| {
        let formats = formats
            .iter()
            .map(|f| generator::variant_ident("VkFormat", f))
            .map(|f| quote! { Self::#f });

        let components = components.iter().map(|c| {
            let Component {
                name,
                numeric_format,
                bits,
                plane_index,
            } = c;

            let plane_index = plane_index
                .map(|i| quote! { Some(#i) })
                .unwrap_or(quote! { None });

            quote! {
                Component {
                    name: ComponentName::#name,
                    numeric_format: NumericFormat::#numeric_format,
                    bits: #bits,
                    plane_index: #plane_index,
                }
            }
        });

        quote! {
            #( #formats )|* => &[ #( #components ),* ],
        }
    });

    let chromas = chromas.into_iter().map(|(chroma, formats)| {
        let formats = formats
            .iter()
            .map(|f| generator::variant_ident("VkFormat", f))
            .map(|f| quote! { Self::#f });

        quote! {
            #( #formats )|* => Some(Chroma::#chroma),
        }
    });

    let compressions = compressions.into_iter().map(|(compression, formats)| {
        let formats = formats
            .iter()
            .map(|f| generator::variant_ident("VkFormat", f))
            .map(|f| quote! { Self::#f });

        quote! {
            #( #formats )|* => Some(Compression::#compression),
        }
    });

    let (format_class_enum, format_classes) = {
        let classes = classes
            .into_iter()
            .map(|(class, values)| {
                let split = class.split_whitespace();
                let mut bit: Option<&str> = None;
                let mut plane: Option<&str> = None;
                let mut chroma: Option<&str> = None;
                let mut rest = Vec::new();
                for seg in split {
                    if seg.ends_with("-bit") {
                        bit = Some(seg.trim_end_matches("-bit"));
                    } else if seg.ends_with("-plane") {
                        plane = Some(seg.trim_end_matches("-plane"));
                    } else if Chroma::from_str(seg).is_ok() {
                        chroma = Some(seg);
                    } else {
                        rest.push(seg);
                    }
                }

                (
                    format_ident!(
                        "{}{}{}{}",
                        bit.map(|b| format!("Bit{}", b)).unwrap_or_default(),
                        plane.map(|p| format!("Plane{}", p)).unwrap_or_default(),
                        chroma.map(|c| format!("Chroma{}", c)).unwrap_or_default(),
                        rest.iter()
                            .map(|s| s.to_upper_camel_case())
                            .collect::<String>()
                    ),
                    values,
                )
            })
            .collect::<Vec<_>>();

        let variants = classes.iter().map(|(class, _)| class);

        let enum_ = quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum FormatClass {
                #( #variants ),*
            }
        };

        let format_classes = classes.into_iter().map(|(class, formats)| {
            let formats = formats
                .iter()
                .map(|f| generator::variant_ident("VkFormat", f))
                .map(|f| quote! { Self::#f });

            quote! {
                #( #formats )|* => FormatClass::#class,
            }
        });

        (enum_, format_classes)
    };

    let packed = packed.into_iter().map(|(packing, formats)| {
        let formats = formats
            .iter()
            .map(|f| generator::variant_ident("VkFormat", f))
            .map(|f| quote! { Self::#f });

        quote! {
            #( #formats )|* => Some(#packing),
        }
    });

    let texels_per_block = texels_per_block.into_iter().map(|(texels, formats)| {
        let formats = formats
            .iter()
            .map(|f| generator::variant_ident("VkFormat", f))
            .map(|f| quote! { Self::#f });

        quote! {
            #( #formats )|* => #texels,
        }
    });

    let block_extents = block_extents.into_iter().map(|((x, y, z), formats)| {
        let formats = formats
            .iter()
            .map(|f| generator::variant_ident("VkFormat", f))
            .map(|f| quote! { Self::#f });

        quote! {
            #( #formats )|* => (#x, #y, #z),
        }
    });

    let block_sizes = block_sizes.into_iter().map(|(block_size, formats)| {
        let formats = formats
            .iter()
            .map(|f| generator::variant_ident("VkFormat", f))
            .map(|f| quote! { Self::#f });

        let block_size = block_size as u64;

        quote! {
            #( #formats )|* => #block_size,
        }
    });

    let impls = quote! {
        impl FormatExt for Format {
            fn block_size(self) -> DeviceSize {
                match self {
                    Self::UNDEFINED => 0u64,
                    #(#block_sizes)*
                    _ => panic!("Unknown format vk::Format({:?})", self.as_raw()),
                }
            }
            fn block_extent(self) -> (u8, u8, u8) {
                match self {
                    Self::UNDEFINED => (1u8, 1u8, 1u8),
                    #(#block_extents)*
                    _ => (1u8, 1u8, 1u8),
                }
            }
            fn texels_per_block(self) -> u8 {
                match self {
                    Self::UNDEFINED => 0u8,
                    #(#texels_per_block)*
                    _ => panic!("Unknown format vk::Format({:?})", self.as_raw()),
                }
            }
            fn packed(self) -> Option<u8> {
                match self {
                    Self::UNDEFINED => None,
                    #(#packed)*
                    _ => None,
                }
            }
            fn format_class(self) -> FormatClass {
                match self {
                    Self::UNDEFINED => panic!("Format::UNDEFINED has no format class"),
                    #(#format_classes)*
                    _ => panic!("Unknown format vk::Format({:?})", self.as_raw()),
                }
            }
            fn chroma(self) -> Option<Chroma> {
                match self {
                    Self::UNDEFINED => None,
                    #(#chromas)*
                    _ => None,
                }
            }
            fn compression(self) -> Option<Compression> {
                match self {
                    Self::UNDEFINED => None,
                    #(#compressions)*
                    _ => None,
                }
            }
            fn components(self) -> &'static [Component] {
                match self {
                    Self::UNDEFINED => &[],
                    #(#components)*
                    _ => panic!("Unknown format vk::Format({:?})", self.as_raw()),
                }
            }
            fn planes(self) -> &'static [Plane] {
                match self {
                    Self::UNDEFINED => &[],
                    #(#planes)*
                    _ => &[],
                }
            }
            fn aspect_flags(self) -> ImageAspectFlags {
                match self {
                    Self::UNDEFINED => ImageAspectFlags::empty(),
                    #(#aspect_flags)*
                    _ => panic!("Unknown format vk::Format({:?})", self.as_raw()),
                }
            }
        }
    };

    let types = quote! {
        /// The Chroma subsampling scheme of a [`Format`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Chroma {
            Chroma420,
            Chroma422,
            Chroma444,
        }

        /// The compression scheme of a compressed [`Format`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Compression {
            Bc,
            Etc2,
            Eac,
            AstcLdr,
            AstcHdr,
            Pvrtc,
        }

        /// The numeric format of a [`Component`] of a [`Format`].
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

        /// The channel name of a [`Component`] of a [`Format`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum ComponentName {
            /// Red channel.
            R,
            /// Green channel.
            G,
            /// Blue channel.
            B,
            /// Alpha channel.
            A,
            /// Depth channel.
            D,
            /// Stencil channel.
            S,
        }

        /// A component of a [`Format`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct Component {
            pub name: ComponentName,
            pub numeric_format: NumericFormat,
            pub bits: u8,
            pub plane_index: Option<u8>,
        }

        /// A distinct plane of a multi-planar [`Format`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct Plane {
            pub width_divisor: u8,
            pub height_divisor: u8,
            /// [`Format`] that is compatible with this plane.
            pub compatible_format: Format,
        }
    };

    let source_code = quote! {
        use ash::vk::{Format, ImageAspectFlags, DeviceSize};

        #format_ext

        #format_class_enum

        #types

        #impls
    };

    fn write_formatted(text: &[u8], out: File) -> std::process::Child {
        let mut child = std::process::Command::new("rustfmt")
            .stdin(std::process::Stdio::piped())
            .stdout(out)
            .spawn()
            .expect("Failed to spawn `rustfmt`");
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(text).unwrap();
        drop(stdin);
        child
    }

    let vk_constants_file = File::create(src_dir.as_ref().join("formats.rs")).expect("formats.rs");

    write_formatted(source_code.to_string().as_bytes(), vk_constants_file)
        .wait()
        .expect("Failed to format source code");
}

pub fn write_features_source_code<P: AsRef<Path>>(vk_headers_dir: &Path, src_dir: P) {
    use std::fs::File;
    use std::io::Write;

    #[derive(Debug)]
    struct FeatureStructInfo {
        ident: proc_macro2::Ident,
        fields: Vec<proc_macro2::Ident>,
        structure_type: proc_macro2::Ident,
    }

    fn member_parts(member: &vk_parse::TypeMemberDefinition) -> (Option<&str>, Option<&str>) {
        let mut ty = None;
        let mut name = None;
        for markup in &member.markup {
            match markup {
                vk_parse::TypeMemberMarkup::Type(value) => ty = Some(value.as_str()),
                vk_parse::TypeMemberMarkup::Name(value) => name = Some(value.as_str()),
                _ => {}
            }
        }
        (ty, name)
    }

    let vk_xml = vk_headers_dir.join("registry/vk.xml");
    let (spec2, errors) = vk_parse::parse_file(&vk_xml).expect("Invalid xml file");
    if !errors.is_empty() {
        eprintln!("vk_parse encountered one or more errors while parsing: {errors:?}");
    }

    let types = spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Types))
        .flat_map(|types| &types.children)
        .filter_map(get_variant!(vk_parse::TypesChild::Type));

    let mut base_fields = Vec::new();
    let mut feature_structs = BTreeMap::<String, FeatureStructInfo>::new();
    let tagged_structures = ash_tagged_structures();

    for ty in types {
        if ty.category.as_deref() != Some("struct") || ty.alias.is_some() {
            continue;
        }
        let Some(name) = ty.name.as_deref() else {
            continue;
        };
        let is_base_features = name == "VkPhysicalDeviceFeatures";
        let is_features2 = name == "VkPhysicalDeviceFeatures2";
        let extends_features2 = ty.structextends.as_deref().is_some_and(|extensions| {
            extensions
                .split(',')
                .any(|extension| extension.trim() == "VkPhysicalDeviceFeatures2")
        });
        if !is_base_features && !is_features2 && !extends_features2 {
            continue;
        }

        let vk_parse::TypeSpec::Members(members) = &ty.spec else {
            continue;
        };

        let mut fields = Vec::new();
        let mut invalid_feature_member = false;
        for member in members {
            let vk_parse::TypeMember::Definition(member) = member else {
                continue;
            };
            let (member_type, member_name) = member_parts(member);
            let Some(member_name) = member_name else {
                continue;
            };
            if member_name == "sType" {
                continue;
            }
            if member_name == "pNext" || member.api.is_some() {
                continue;
            }
            if is_features2 {
                continue;
            }

            let is_pointer_or_array =
                member.len.is_some() || member.code.contains('*') || member.code.contains('[');
            if member_type != Some("VkBool32") || is_pointer_or_array {
                invalid_feature_member = true;
                continue;
            }
            fields.push(format_ident!("{}", member_name.to_snake_case()));
        }

        if is_base_features {
            if !invalid_feature_member && !fields.is_empty() {
                base_fields = fields;
            }
            continue;
        }
        if is_features2 || invalid_feature_member || fields.is_empty() {
            continue;
        }

        let ash_name = name.strip_prefix("Vk").unwrap();
        let Some(structure_type) = tagged_structures.get(ash_name) else {
            continue;
        };
        let ident = format_ident!("{ash_name}");
        if ash_type_is_provisional(&ident) {
            continue;
        }
        let structure_type = format_ident!("{structure_type}");
        feature_structs.insert(
            name.to_owned(),
            FeatureStructInfo {
                ident,
                fields,
                structure_type,
            },
        );
    }

    assert!(
        !base_fields.is_empty(),
        "VkPhysicalDeviceFeatures has no valid Bool32 fields"
    );
    assert!(
        !feature_structs.is_empty(),
        "No extension feature structures found"
    );

    let struct_impls = feature_structs.values().map(|info| {
        let ident = &info.ident;
        let structure_type = &info.structure_type;
        let fields = &info.fields;
        let mut contains_fields = fields.iter().map(|field| {
            quote! { (self.#field & other.#field) == other.#field }
        });
        let first_contains = contains_fields.next().expect("feature fields are nonempty");
        let unions = fields
            .iter()
            .map(|field| quote! { self.#field |= other.#field; });
        quote! {
            impl FeatureStruct for #ident<'_> {
                const STRUCTURE_TYPE: StructureType = StructureType::#structure_type;

                fn contains_fields(&self, other: &Self) -> bool {
                    #first_contains #(&& #contains_fields)*
                }

                fn union_fields(&mut self, other: &Self) {
                    #(#unions)*
                }
            }
        }
    });

    let feature_match_arms = feature_structs.values().map(|info| {
        let ident = &info.ident;
        quote! {
            <#ident<'static> as FeatureStruct>::STRUCTURE_TYPE => {
                if !unsafe { $f(&*next.cast::<#ident<'static>>(), $context) } {
                    break;
                }
            }
        }
    });

    let mut base_contains = base_fields.iter().map(|field| {
        quote! { (self.#field & other.#field) == other.#field }
    });
    let first_base_contains = base_contains
        .next()
        .expect("base feature fields are nonempty");
    let base_unions = base_fields
        .iter()
        .map(|field| quote! { self.#field |= other.#field; });
    let feature_delegates = feature_structs.values().map(|info| {
        let ident = &info.ident;
        quote! {
            impl PhysicalDeviceFeaturesExt for #ident<'_> {
                fn contains(&self, other: &Self) -> bool {
                    self.contains_fields(other)
                }

                fn union(&mut self, other: &Self) {
                    self.union_fields(other)
                }
            }
        }
    });

    let source_code = quote! {
        extern crate alloc;

        use alloc::boxed::Box;
        use core::ffi::c_void;
        use core::ptr;
        use ash::vk::*;

        /// Field operations for an extension feature structure in a Vulkan `pNext` chain.
        pub trait FeatureStruct: Copy {
            const STRUCTURE_TYPE: StructureType;
            fn contains_fields(&self, other: &Self) -> bool;
            fn union_fields(&mut self, other: &Self);
        }

        #(#struct_impls)*

        /// Set-like operations over physical-device feature structures.
        ///
        /// A feature is enabled when its `Bool32` field is nonzero. `contains`
        /// checks that every feature enabled in `other` is enabled in `self`,
        /// while `union` enables every feature enabled in `other`.
        pub trait PhysicalDeviceFeaturesExt {
            /// Returns true if every feature enabled in `other` is also enabled in `self`.
            ///
            /// For `PhysicalDeviceFeatures2`, extension structures present in
            /// `other` but absent from `self` are treated as all-disabled.
            ///
            /// # Performance
            ///
            /// For `PhysicalDeviceFeatures` and individual feature structures this is
            /// O(number of feature fields). For `PhysicalDeviceFeatures2` it walks both
            /// `pNext` chains, and each node of `other`'s chain is looked up by
            /// linear scan in `self`'s chain, i.e. O(n * m) in the chain lengths of
            /// `self` and `other`. Avoid calling it in hot paths with long chains.
            fn contains(&self, other: &Self) -> bool;
            /// Enables in `self` every feature enabled in `other`.
            ///
            /// For `PhysicalDeviceFeatures2`, missing extension structures are
            /// appended to `self`'s chain. Appended structures are intentionally
            /// leaked because the chain may outlive this call.
            fn union(&mut self, other: &Self);
            /// Convenience: `self.union(other); self` (requires `Copy` and `Sized`).
            fn unioned(mut self, other: &Self) -> Self where Self: Sized + Copy {
                self.union(other);
                self
            }
        }

        impl PhysicalDeviceFeaturesExt for PhysicalDeviceFeatures {
            fn contains(&self, other: &Self) -> bool {
                #first_base_contains #(&& #base_contains)*
            }

            fn union(&mut self, other: &Self) {
                #(#base_unions)*
            }
        }

        #(#feature_delegates)*

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

        macro_rules! for_each_feature {
            ($next:expr, $f:path, $context:expr) => {{
                let mut next = ($next).cast::<Header>();
                while !next.is_null() {
                    match unsafe { (*next).s_type } {
                        #(#feature_match_arms)*
                        _ => {}
                    }
                    next = unsafe { (*next).p_next };
                }
            }};
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn chain_contains(self_next: *const c_void, other_next: *const c_void) -> bool {
            // SAFETY: Walking requires valid, initialized Vulkan structures with
            // correct s_type values, as guaranteed for feature chains returned
            // by vkGetPhysicalDeviceFeatures2. The chains are only read here.
            let mut valid = true;
        for_each_feature!(other_next, check_feature_node, (self_next, &mut valid as *mut bool));
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
            // SAFETY: Walking requires valid, initialized Vulkan structures with
            // correct s_type values. The source chain is read-only; only nodes
            // reachable from this mutable destination root are written.
            for_each_feature!(other_next, union_feature_node, self_next as *mut *mut c_void);
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

            // The new node is leaked intentionally: a Vulkan chain may outlive
            // this stack frame. The destination tail is writable and its p_next
            // is overwritten; callers must own writable destination chain memory.
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
    };

    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(File::create(src_dir.as_ref().join("features.rs")).expect("features.rs"))
        .spawn()
        .expect("Failed to spawn `rustfmt`");
    child
        .stdin
        .take()
        .expect("Failed to open stdin")
        .write_all(source_code.to_string().as_bytes())
        .unwrap();
    child.wait().expect("Failed to format feature source code");
}

fn ash_type_is_provisional(ident: &proc_macro2::Ident) -> bool {
    let source = include_str!("../../../ash/ash/src/vk/definitions.rs");
    let Some(start) = source.find(&format!("pub struct {ident}")) else {
        return false;
    };
    let preceding = &source[..start];
    let struct_attributes = preceding
        .rfind("#[repr(C)]")
        .map(|attribute| &preceding[attribute..])
        .unwrap_or(preceding);
    struct_attributes.contains("#[cfg(feature = \"provisional\")]")
}

fn ash_tagged_structures() -> std::collections::HashMap<String, String> {
    use std::collections::HashMap;

    let source = include_str!("../../../ash/ash/src/vk/definitions.rs");
    let mut tagged_structures = HashMap::new();
    let lines = source.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        let Some(struct_name) = line
            .trim()
            .strip_prefix("unsafe impl<'a> TaggedStructure<'a> for ")
            .and_then(|tail| tail.strip_suffix("<'a> {"))
        else {
            continue;
        };
        // ash may wrap the const expression across multiple lines, so inspect
        // the remainder of this impl block until its closing brace.
        let const_name = lines[index + 1..]
            .iter()
            .take_while(|line| line.trim() != "}")
            .find_map(|line| {
                let line = line.trim();
                line.split_once("StructureType::")
                    .and_then(|(_, tail)| tail.strip_suffix(';'))
            });
        let Some(const_name) = const_name else {
            continue;
        };
        tagged_structures.insert(struct_name.to_owned(), const_name.to_owned());
    }
    tagged_structures
}
