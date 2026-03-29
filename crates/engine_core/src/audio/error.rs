//! 音声ロード・バックエンド周りの型付きエラー。

use thiserror::Error;

/// ファイルロード失敗の分類（I/O / デコード / 出力バックエンド）。
#[derive(Debug, Error)]
pub enum AudioLoadError {
    /// ファイル読み込み（存在、権限、読取り）。
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    /// コンテナ解析・サンプル形式などデコード系。
    #[error("decode: {0}")]
    Decode(String),
    /// `kira` の再生・ミキサ等のバックエンド。
    #[error("backend: {0}")]
    Backend(String),
}

impl AudioLoadError {
    /// `kira` の `FromFileError` をプロジェクトの分類へ写す。
    #[cfg(feature = "audio-kira")]
    #[must_use]
    pub fn from_kira_file_error(e: kira::sound::FromFileError) -> Self {
        use kira::sound::FromFileError as E;
        match e {
            E::IoError(io) => Self::Io(io),
            E::SymphoniaError(err) => Self::Decode(err.to_string()),
            other => Self::Decode(other.to_string()),
        }
    }
}
