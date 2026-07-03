mod decoder;
mod deinterlaced_row;
mod header;
mod lzw;
mod sub_blocks;
mod to_argb;

pub use decoder::decode_gif_argb8888;
