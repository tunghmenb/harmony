#![doc = include_str!("../README.md")]

pub mod chat;
mod concurrent_loading;
mod encoding;
mod registry;
mod tiktoken;
pub mod tiktoken_ext;

pub use concurrent_loading::{load_harmony_encoding_from_file, load_harmony_encoding_safe};
pub use encoding::{HarmonyEncoding, StreamableParser};
pub use registry::load_harmony_encoding;
pub use registry::HarmonyEncodingName;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "python-binding")]
mod py_module;

#[cfg(feature = "wasm-binding")]
mod wasm_module;
