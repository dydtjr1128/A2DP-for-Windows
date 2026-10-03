use core::fmt;

/// Codec identities in the project roadmap, not a list of implemented encoders.
///
/// Rust discriminants are not Bluetooth codec IDs or a stable IPC/driver ABI.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Codec {
    /// The mandatory A2DP baseline codec.
    Sbc,
    /// Advanced Audio Coding; integration and distribution conditions are pending.
    Aac,
    /// LDAC; integration and hardware validation are pending.
    Ldac,
    /// Classic aptX, distinct from Adaptive, HD, and Low Latency.
    Aptx,
    /// aptX HD; not interchangeable with classic aptX.
    AptxHd,
    /// aptX Low Latency; protocol and distribution review are pending.
    AptxLowLatency,
}

impl Codec {
    /// All roadmap identities. This must not be used as a local encoder inventory.
    pub const ALL: [Self; 6] = [
        Self::Sbc,
        Self::Aac,
        Self::Ldac,
        Self::Aptx,
        Self::AptxHd,
        Self::AptxLowLatency,
    ];
}

impl fmt::Display for Codec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Sbc => "SBC",
            Self::Aac => "AAC",
            Self::Ldac => "LDAC",
            Self::Aptx => "aptX",
            Self::AptxHd => "aptX HD",
            Self::AptxLowLatency => "aptX Low Latency",
        })
    }
}

/// Whether SBC may be selected when no preferred codec is common to both sides.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FallbackPolicy {
    /// Fail instead of silently changing the user's requested codec.
    #[default]
    Disabled,
    /// Permit SBC only when a local encoder and the peer both support it.
    AllowSbc,
}

/// Why a codec candidate was selected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionReason {
    /// The first common entry in the caller's preference order.
    Preferred,
    /// No preferred entry matched and the caller explicitly permitted SBC.
    SbcFallback,
}

/// A policy result, not evidence of codec format negotiation or active playback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CodecSelection {
    /// Candidate identity. Codec-specific format checks are still required.
    pub codec: Codec,
    /// Expose this reason to callers instead of silently applying a fallback.
    pub reason: SelectionReason,
}

/// A codec policy failure, distinct from device discovery or transport errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionError {
    /// No preference was supplied. Fallback must not hide this configuration error.
    EmptyPreference,
    /// Neither a preferred codec nor an explicitly permitted SBC fallback matched.
    NoCommonCodec,
}

impl fmt::Display for SelectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::EmptyPreference => "at least one preferred codec is required",
            Self::NoCommonCodec => "no codec satisfies local, remote, and preference constraints",
        })
    }
}

impl core::error::Error for SelectionError {}

/// Select a candidate using preference order and explicit SBC fallback.
///
/// `local_encoders` must describe usable, enabled backends in the caller's build,
/// not [`Codec::ALL`]. This workspace currently contains no such backends.
/// `remote_codecs` must come from a completed capability query for the current
/// session; handle query failures before calling this function. Neither input
/// implies compatible sample rates, channels, bitrates, or a successful stream.
///
/// Duplicate identities are harmless. Input order matters only for `preferred`.
/// The function allocates no memory and performs no I/O.
///
/// # Errors
///
/// Returns [`SelectionError::EmptyPreference`] for an empty preference list, even
/// if fallback is enabled. Otherwise returns [`SelectionError::NoCommonCodec`]
/// if neither a preferred identity nor the allowed SBC fallback is common.
///
/// # Example
///
/// Policy-only example with synthetic inventories, not detected hardware:
///
/// ```
/// use a2dp_core::{Codec, FallbackPolicy, SelectionReason, select_codec};
///
/// let candidate = select_codec(
///     &[Codec::Ldac],
///     &[Codec::Sbc],
///     &[Codec::Sbc, Codec::Ldac],
///     FallbackPolicy::AllowSbc,
/// )?;
/// assert_eq!(candidate.codec, Codec::Sbc);
/// assert_eq!(candidate.reason, SelectionReason::SbcFallback);
/// # Ok::<(), a2dp_core::SelectionError>(())
/// ```
pub fn select_codec(
    preferred: &[Codec],
    local_encoders: &[Codec],
    remote_codecs: &[Codec],
    fallback: FallbackPolicy,
) -> Result<CodecSelection, SelectionError> {
    if preferred.is_empty() {
        return Err(SelectionError::EmptyPreference);
    }

    let common = |codec: &Codec| local_encoders.contains(codec) && remote_codecs.contains(codec);

    if let Some(&codec) = preferred.iter().find(|codec| common(codec)) {
        return Ok(CodecSelection {
            codec,
            reason: SelectionReason::Preferred,
        });
    }

    if fallback == FallbackPolicy::AllowSbc && common(&Codec::Sbc) {
        return Ok(CodecSelection {
            codec: Codec::Sbc,
            reason: SelectionReason::SbcFallback,
        });
    }

    Err(SelectionError::NoCommonCodec)
}
