//! `sound.csv` のデシリアライズ（雛形）。

use serde::Deserialize;

/// BGM/SE の種別。
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SoundType {
    Bgm,
    Se,
}

/// `sound.csv` 1 行分のメタデータ。
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SoundMeta {
    pub sound_id: String,
    #[serde(rename = "type")]
    pub sound_type: SoundType,
    pub file_path: String,
    pub volume: f32,
    pub loop_start: f32,
    pub loop_end: f32,
}

/// CSV文字列から `SoundMeta` を読み込む。
///
/// # Errors
/// - CSV の構文/型変換に失敗した場合
pub fn deserialize_sound_meta_from_str(csv_text: &str) -> anyhow::Result<Vec<SoundMeta>> {
    deserialize_sound_meta_from_csv_reader(csv_text.as_bytes())
}

/// CSV Reader から `SoundMeta` を読み込む。
///
/// # Errors
/// - CSV の構文/型変換に失敗した場合
pub fn deserialize_sound_meta_from_csv_reader<R: std::io::Read>(
    reader: R,
) -> anyhow::Result<Vec<SoundMeta>> {
    let mut csv_reader = csv::ReaderBuilder::new().has_headers(true).from_reader(reader);

    let mut out = Vec::new();
    for result in csv_reader.deserialize::<SoundMeta>() {
        out.push(result?);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_sound_meta_missing_loop_end_returns_err() {
        let csv_text = "\
sound_id,type,file_path,volume,loop_start,loop_end
bgm_001,BGM,assets/bgm/1.ogg,0.8,1.0
";

        let result = deserialize_sound_meta_from_str(csv_text);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_sound_meta_valid_csv_parses_ok() -> anyhow::Result<()> {
        let csv_text = "\
sound_id,type,file_path,volume,loop_start,loop_end
bgm_001,BGM,assets/bgm/1.ogg,0.8,1.0,2.5
se_001,SE,assets/se/1.wav,0.9,0.0,0.0
";

        let result = deserialize_sound_meta_from_str(csv_text)?;
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].sound_id, "bgm_001");
        assert_eq!(result[0].sound_type, SoundType::Bgm);
        assert_eq!(result[0].file_path, "assets/bgm/1.ogg");
        assert_eq!(result[0].volume, 0.8);
        assert_eq!(result[0].loop_start, 1.0);
        assert_eq!(result[0].loop_end, 2.5);

        Ok(())
    }
}
