//! バイナリ向けの `env_logger` 初期化。
//!
//! 既定では**標準エラーとログファイルの両方**へ同じ内容を書き出す（tee）。
//!
//! # 環境変数
//!
//! - **`RUST_LOG`**: [`env_logger`] と同じフィルタ規則。**未設定時は `info` を既定**とする（`env_logger` 単体の既定は `error` のみで、`log::info!` が一切残らない）。
//! - **`RS_GAME_LOG_DIR`**: ログディレクトリ。未設定時はカレントディレクトリ直下の `logs`。
//! - **`RS_GAME_LOG_DISABLE_FILE`**: `1` / `true` / `yes` のときファイルへ書かず標準エラーのみ。

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use env_logger::{Env, Target};
use thiserror::Error;

const ENV_LOG_DIR: &str = "RS_GAME_LOG_DIR";
const ENV_DISABLE_FILE: &str = "RS_GAME_LOG_DISABLE_FILE";

/// `RUST_LOG` が未設定のときに使う既定レベル（起動メッセージがログに残るようにする）。
const DEFAULT_LOG_FILTER: &str = "info";

fn install_panic_hook() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        eprintln!("----- panic -----");
        eprintln!("{info}");
        previous(info);
    }));
}

fn new_logger_builder() -> env_logger::Builder {
    env_logger::Builder::from_env(Env::default().default_filter_or(DEFAULT_LOG_FILTER))
}

/// ログ初期化に失敗したときのエラー（グローバルロガーの二重登録など）。
#[derive(Debug, Error)]
pub enum LoggingInitError {
    #[error("failed to install logger: {0}")]
    Logger(#[from] log::SetLoggerError),
}

struct TeeWriter {
    file: Option<File>,
}

impl Write for TeeWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::stderr().write_all(buf)?;
        if let Some(ref mut f) = self.file {
            f.write_all(buf)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stderr().flush()?;
        if let Some(ref mut f) = self.file {
            f.flush()?;
        }
        Ok(())
    }
}

fn log_file_disabled() -> bool {
    std::env::var(ENV_DISABLE_FILE)
        .is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes"))
}

fn log_directory() -> PathBuf {
    std::env::var(ENV_LOG_DIR).map_or_else(|_| PathBuf::from("logs"), PathBuf::from)
}

fn sanitized_app_name(app_name: &str) -> String {
    app_name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

fn unique_log_path(dir: &Path, app_name: &str) -> PathBuf {
    let safe = sanitized_app_name(app_name);
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let pid = u64::from(std::process::id());
    dir.join(format!("{safe}_{secs}_{pid}.log"))
}

fn open_log_file(dir: &Path, app_name: &str) -> io::Result<(PathBuf, File)> {
    fs::create_dir_all(dir)?;
    let path = unique_log_path(dir, app_name);
    let file = OpenOptions::new().create(true).append(true).open(&path)?;
    Ok((path, file))
}

/// `env_logger` をインストールする。
///
/// `app_name` はログファイル名に使う識別子（例: クレート名 `script_editor`）。
///
/// # 戻り値
///
/// - `Ok(Some(path))` — ファイルへも出力している。
/// - `Ok(None)` — ファイル出力なし（環境変数で無効、またはファイルオープン失敗時の標準エラーのみフォールバック）。
///
/// # Errors
///
/// グローバルロガーが既に登録されている場合など、[`env_logger::Builder::try_init`] が失敗すると返る。
///
/// パニックフックの上書きは、ロガー登録が**成功した場合にのみ**行う（失敗時はグローバル状態を変えない）。
pub fn try_init_app_logging(app_name: &str) -> Result<Option<PathBuf>, LoggingInitError> {
    let mut builder = new_logger_builder();

    let log_path = if log_file_disabled() {
        builder.try_init()?;
        None
    } else {
        let dir = log_directory();
        match open_log_file(&dir, app_name) {
            Ok((path, file)) => {
                let tee = TeeWriter { file: Some(file) };
                builder.target(Target::Pipe(Box::new(tee)));
                builder.try_init()?;
                Some(path)
            }
            Err(e) => {
                eprintln!("rs-game-dev: could not open log file ({e}); logging to stderr only");
                builder.try_init()?;
                None
            }
        }
    };

    install_panic_hook();
    Ok(log_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitized_app_name_replaces_invalid_chars() {
        assert_eq!(sanitized_app_name("script_editor"), "script_editor");
        assert_eq!(sanitized_app_name("a b:c"), "a_b_c");
    }
}
