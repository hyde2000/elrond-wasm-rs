use elrond_wasm::elrond_codec::*;
// use elrond_wasm::elrond_codec_derive::*;
use elrond_wasm::{BoxedBytes, Vec};

/// Copied from elrond-wasm serialization tests.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct SerExample1 {
	pub int: u16,
	pub seq: Vec<u8>,
	pub another_byte: u8,
	pub uint_32: u32,
	pub uint_64: u64,
	pub boxed_bytes: BoxedBytes,
}