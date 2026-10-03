//! OS-independent policy for a future Windows A2DP Source.
//!
//! This crate contains no encoder, device discovery, driver, or audio transport.
//! A selected codec is only a candidate: format negotiation and peer acceptance
//! must still succeed before a caller may report an active stream.

#![no_std]

mod codec;

pub use codec::{
    Codec, CodecSelection, FallbackPolicy, SelectionError, SelectionReason, select_codec,
};
