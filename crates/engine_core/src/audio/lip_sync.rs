//! RMS 解析（リップシンク値計算）雛形。

/// 音声サンプルから RMS（実効値）を計算し、リップシンク用の 0.0〜1.0 値へ変換する。
///
/// # Panics
/// - `frame_size == 0` の場合は 0.0 を返すため、パニックしない。
#[must_use]
pub fn calculate_lip_sync_value(audio_samples: &[f32], frame_size: usize) -> f32 {
    if frame_size == 0 {
        return 0.0;
    }

    let sum_sq: f32 = audio_samples.iter().take(frame_size).map(|s| s * s).sum();

    #[allow(clippy::cast_precision_loss)]
    let frame_size_f32 = frame_size as f32;

    let rms = sum_sq / frame_size_f32;
    let rms = rms.sqrt();

    (rms * 10.0).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_lip_sync_value_zero_frame_size_returns_zero() {
        let samples = [0.5_f32, -0.5_f32];
        assert_eq!(calculate_lip_sync_value(&samples, 0), 0.0);
    }

    #[test]
    fn test_calculate_lip_sync_value_all_zero_samples_returns_zero() {
        let samples = [0.0_f32; 256];
        assert_eq!(calculate_lip_sync_value(&samples, 128), 0.0);
    }

    #[test]
    fn test_calculate_lip_sync_value_unit_amplitude_clamps_to_one() {
        // RMS = 1.0 → (1.0 * 10.0).clamp(0.0, 1.0) = 1.0
        let samples = [1.0_f32; 64];
        assert_eq!(calculate_lip_sync_value(&samples, 64), 1.0);
    }
}
