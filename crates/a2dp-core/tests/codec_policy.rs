//! Policy tests use synthetic inventories and make no hardware support claims.

use a2dp_core::{
    Codec, CodecSelection, FallbackPolicy, SelectionError, SelectionReason, select_codec,
};

#[test]
fn preference_order_wins_over_inventory_order() {
    assert_eq!(
        select_codec(
            &[Codec::Ldac, Codec::AptxHd, Codec::Sbc],
            &[Codec::Sbc, Codec::AptxHd, Codec::Ldac],
            &[Codec::AptxHd, Codec::Sbc, Codec::Ldac],
            FallbackPolicy::Disabled,
        ),
        Ok(CodecSelection {
            codec: Codec::Ldac,
            reason: SelectionReason::Preferred,
        })
    );
}

#[test]
fn unavailable_preference_can_move_to_next_explicit_preference() {
    let result = select_codec(
        &[Codec::Ldac, Codec::AptxHd],
        &[Codec::Ldac, Codec::AptxHd],
        &[Codec::Sbc, Codec::AptxHd],
        FallbackPolicy::AllowSbc,
    )
    .unwrap();
    assert_eq!(result.codec, Codec::AptxHd);
    assert_eq!(result.reason, SelectionReason::Preferred);
}

#[test]
fn remote_support_does_not_create_a_local_encoder() {
    assert_eq!(
        select_codec(&Codec::ALL, &[], &Codec::ALL, FallbackPolicy::AllowSbc,),
        Err(SelectionError::NoCommonCodec)
    );
}

#[test]
fn local_encoder_does_not_create_remote_support() {
    assert_eq!(
        select_codec(
            &[Codec::Ldac],
            &[Codec::Ldac, Codec::Sbc],
            &[],
            FallbackPolicy::AllowSbc,
        ),
        Err(SelectionError::NoCommonCodec)
    );
}

#[test]
fn strict_request_does_not_silently_fall_back_to_sbc() {
    assert_eq!(
        select_codec(
            &[Codec::Ldac],
            &[Codec::Ldac, Codec::Sbc],
            &[Codec::Sbc],
            FallbackPolicy::Disabled,
        ),
        Err(SelectionError::NoCommonCodec)
    );
}

#[test]
fn allowed_sbc_fallback_reports_why_it_was_selected() {
    assert_eq!(
        select_codec(
            &[Codec::Ldac],
            &[Codec::Ldac, Codec::Sbc],
            &[Codec::Sbc],
            FallbackPolicy::AllowSbc,
        ),
        Ok(CodecSelection {
            codec: Codec::Sbc,
            reason: SelectionReason::SbcFallback,
        })
    );
}

#[test]
fn fallback_requires_sbc_on_both_sides() {
    for (local, remote) in [
        (&[Codec::Aptx][..], &[Codec::Sbc][..]),
        (&[Codec::Sbc][..], &[Codec::Aptx][..]),
    ] {
        assert_eq!(
            select_codec(&[Codec::Ldac], local, remote, FallbackPolicy::AllowSbc),
            Err(SelectionError::NoCommonCodec)
        );
    }
}

#[test]
fn fallback_does_not_hide_an_empty_preference() {
    for fallback in [FallbackPolicy::Disabled, FallbackPolicy::AllowSbc] {
        assert_eq!(
            select_codec(&[], &Codec::ALL, &Codec::ALL, fallback),
            Err(SelectionError::EmptyPreference)
        );
    }
}

#[test]
fn explicitly_requested_sbc_is_not_reported_as_a_fallback() {
    let selected = select_codec(
        &[Codec::Sbc],
        &[Codec::Sbc],
        &[Codec::Sbc],
        FallbackPolicy::AllowSbc,
    )
    .unwrap();
    assert_eq!(selected.reason, SelectionReason::Preferred);
}

#[test]
fn similarly_named_aptx_variants_are_not_interchangeable() {
    for requested in [Codec::AptxHd, Codec::AptxLowLatency] {
        assert_eq!(
            select_codec(
                &[requested],
                &Codec::ALL,
                &[Codec::Aptx],
                FallbackPolicy::Disabled,
            ),
            Err(SelectionError::NoCommonCodec)
        );
    }
}

#[test]
fn restricted_candidates_are_never_added_to_preferences_implicitly() {
    assert_eq!(
        select_codec(
            &[Codec::Ldac],
            &[Codec::Aac, Codec::AptxLowLatency],
            &[Codec::Aac, Codec::AptxLowLatency],
            FallbackPolicy::AllowSbc,
        ),
        Err(SelectionError::NoCommonCodec)
    );
}

#[test]
fn every_returned_candidate_is_supported_by_both_sides() {
    // Exhaust all 64 x 64 inventories, both fallback policies, and each request.
    // This checks the safety invariant, independently of preference ordering.
    let inventory = |mask: u8| -> Vec<Codec> {
        Codec::ALL
            .iter()
            .enumerate()
            .filter(|(index, _)| mask & (1 << index) != 0)
            .map(|(_, codec)| *codec)
            .collect()
    };
    for local_mask in 0..64 {
        let local = inventory(local_mask);
        for remote_mask in 0..64 {
            let remote = inventory(remote_mask);
            for requested in Codec::ALL {
                for fallback in [FallbackPolicy::Disabled, FallbackPolicy::AllowSbc] {
                    if let Ok(selected) = select_codec(&[requested], &local, &remote, fallback) {
                        assert!(local.contains(&selected.codec));
                        assert!(remote.contains(&selected.codec));
                        if selected.codec != requested {
                            assert_eq!(fallback, FallbackPolicy::AllowSbc);
                            assert_eq!(selected.codec, Codec::Sbc);
                            assert_eq!(selected.reason, SelectionReason::SbcFallback);
                        }
                    }
                }
            }
        }
    }
}
