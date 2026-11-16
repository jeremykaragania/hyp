pub const MAX_IMAGES: usize = 8;

#[derive(Clone, Debug)]
pub struct Image {
    pub address: u64,
    pub size: usize,
}
