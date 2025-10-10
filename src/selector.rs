use crate::models::MediaSource;

///  1. Resolution First: Higher resolution always wins (e.g., 1080p beats 720p)
//   2. Quality-Optimized Codec Comparison: At the same resolution, it calculates an "effective bitrate" that accounts for codec efficiency:
//     - H.265/HEVC: 1.5x multiplier (50% more efficient than H.264)
//     - AV1: 2.0x multiplier (50% more efficient than H.264, ~30% better than H.265)
//     - H.264: 1.0x baseline
//   3. Example Scenarios:
//     - File A: 1080p H.264 @ 8 Mbps = 8.0 effective bitrate
//     - File B: 1080p H.265 @ 6 Mbps = 9.0 effective bitrate (6 × 1.5) → File B selected
//     - File C: 1080p H.265 @ 4 Mbps = 6.0 effective bitrate → File A selected
pub fn select_best_source(sources: &[MediaSource]) -> Option<usize> {
    if sources.is_empty() {
        return None;
    }

    let mut best_idx = 0;

    for (idx, source) in sources.iter().enumerate().skip(1) {
        if is_better_source(source, &sources[best_idx]) {
            best_idx = idx;
        }
    }

    Some(best_idx)
}

fn is_better_source(candidate: &MediaSource, current_best: &MediaSource) -> bool {
    let candidate_height = get_height(candidate);
    let best_height = get_height(current_best);

    // Higher resolution always wins
    if candidate_height > best_height {
        return true;
    }
    if candidate_height < best_height {
        return false;
    }

    let candidate_effective_bitrate = calculate_effective_bitrate(candidate);
    let best_effective_bitrate = calculate_effective_bitrate(current_best);

    candidate_effective_bitrate > best_effective_bitrate
}

fn calculate_effective_bitrate(source: &MediaSource) -> f64 {
    let bitrate = source.bitrate.unwrap_or(0) as f64;
    let codec = get_codec(source);
    let efficiency_multiplier = get_codec_efficiency_multiplier(&codec);


    bitrate * efficiency_multiplier
}

fn get_codec_efficiency_multiplier(codec: &str) -> f64 {
    let codec_lower = codec.to_lowercase();
    if codec_lower.contains("av1") {
        // AV1 is ~50% more efficient than H.264, or ~30% better than H.265
        // So same bitrate AV1 ≈ 2.0x the quality of H.264
        2.0
    } else if codec_lower.contains("hevc") || codec_lower.contains("h265") {
        // H.265 is ~50% more efficient than H.264
        // So same bitrate H.265 ≈ 1.5x the quality of H.264
        1.5
    } else {
        1.0
    }
}

fn get_height(source: &MediaSource) -> i32 {
    source.height.or_else(|| {
        source.media_streams.as_ref().and_then(|streams| {
            streams
                .iter()
                .find(|s| s.stream_type.as_deref() == Some("Video"))
                .and_then(|s| s.height)
        })
    }).unwrap_or(0)
}

fn get_codec(source: &MediaSource) -> String {
    source.media_streams
        .as_ref()
        .and_then(|streams| {
            streams
                .iter()
                .find(|s| s.stream_type.as_deref() == Some("Video"))
                .and_then(|s| s.codec.clone())
        })
        .unwrap_or_default()
}
