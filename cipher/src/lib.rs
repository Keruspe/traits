//! This crate defines a set of traits which describe the functionality of
//! [block ciphers][1], [block modes][2], and [stream ciphers][3].
//!
//! [1]: https://en.wikipedia.org/wiki/Block_cipher
//! [2]: https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation
//! [3]: https://en.wikipedia.org/wiki/Stream_cipher

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_root_url = "https://docs.rs/cipher/0.4.0"
)]
#![warn(missing_docs, rust_2018_idioms)]

pub use crypto_common;
pub use inout;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "rand_core")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand_core")))]
pub use crypto_common::rand_core;

#[cfg(feature = "block-padding")]
#[cfg_attr(docsrs, doc(cfg(feature = "block-padding")))]
pub use inout::block_padding;

#[cfg(feature = "zeroize")]
pub use zeroize;

#[cfg(feature = "dev")]
pub use blobby;

mod block;
#[cfg(feature = "dev")]
mod dev;
mod errors;
mod stream;
mod stream_core;
mod stream_wrapper;

pub use crate::{block::*, errors::*, stream::*, stream_core::*, stream_wrapper::*};
pub use crypto_common::{
    generic_array,
    typenum::{self, consts},
    AlgorithmName, Block, InnerIvInit, InvalidLength, Iv, IvSizeUser, Key, KeyInit, KeyIvInit,
    KeySizeUser,
};
use generic_array::{ArrayLength, GenericArray};

/// Trait for loading current IV state.
pub trait IvState: IvSizeUser {
    /// Returns current IV state.
    fn iv_state(&self) -> Iv<Self>;
}

/// Types which process blocks in parallel.
pub trait ParBlocksSizeUser: BlockSizeUser {
    /// Number of blocks which can be processed in parallel.
    type ParBlocksSize: ArrayLength<Block<Self>>;
}

/// Parallel blocks on which [`ParBlocksSizeUser`] implementors operate.
pub type ParBlocks<T> = GenericArray<Block<T>, <T as ParBlocksSizeUser>::ParBlocksSize>;
