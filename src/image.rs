use crate::bitmap_words;
use crate::boot::map::{TEXT_MEM_ATTRIBUTES, map_region};
use crate::mm::pool::Pool;
use crate::mm::table::Descriptor;
use crate::mm::table_alloc::base_table_ptr;
use crate::platform::PLATFORM;
use crate::sync::Spinlock;

pub const MAX_IMAGES: usize = 8;

pub static IMAGES: Spinlock<Pool<InitializedImage, MAX_IMAGES, { bitmap_words!(MAX_IMAGES) }>> =
    Spinlock::new(Pool::new());

// The physical memory representation of an image before we initialize it.
#[derive(Clone)]
pub struct ImageDesc {
    pub address: u64,
    pub size: usize,
}

// We follow the same pattern as devices. We have the unitialized sort of raw
// representation that we store in `PLATFORM` then we turn store the initialized
// ones here. It's a bit of a tradeoff, like are we better just having partially
// initialized objects and only having one structure vs splitting them up?
pub struct InitializedImage {
    pub entrypoint: u64,
}

// https://www.kernel.org/doc/html/v6.8/arch/arm64/booting.html#call-the-kernel-image
#[repr(C)]
pub struct KernelImageHeader {
    code: [u32; 2],       // Executable code.
    text_offset: u64,     // Image load offset, little endian.
    image_size: u64,      // Effective Image size, little endian.
    flags: u64,           // Kernel flags, little endian.
    reserved_0: [u64; 3], // Reserved.
    magic: u32,           // Magic number, little endian, "ARM\x64".
    reserved_1: u32,      // Reserved (used for PE COFF offset).
}

fn init_image(image: &ImageDesc) -> Result<InitializedImage, &'static str> {
    let begin = image.address;
    let end = begin + image.size as u64;
    let table = base_table_ptr();

    unsafe {
        // NOTE: These mappings are just for the hypervisor, we haven't yet
        // concerned ourself with stage 2 translations...
        map_region(
            table as *mut Descriptor,
            begin,
            end,
            begin,
            &TEXT_MEM_ATTRIBUTES,
        );
    }

    // Validate the header. We only support uncompressed Linux kernel ARM64 boot
    // executable images.
    let header = unsafe { &*(begin as *const KernelImageHeader) };

    if header.magic != 0x644d5241 {
        return Err("Bad kernel image magic number");
    }

    Ok(InitializedImage {
        entrypoint: begin, /* NOTE: If we decide to stop doing identity mappings make sure to
                            * change this to the mapped address. */
    })
}

// Right now just map the images into memory. We still access them from
// `PLATFORM`. Like there's not much else we really need to store about an
// image, it's just a buffer for a virtual machine to use.
pub fn init_images() -> Result<(), &'static str> {
    let mut platform = PLATFORM.lock();
    let mut images = IMAGES.lock();

    for (_, image) in platform.images.iter() {
        images.insert(init_image(image)?);
    }

    Ok(())
}
