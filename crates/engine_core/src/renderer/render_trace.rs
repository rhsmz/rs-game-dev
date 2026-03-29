//! `render_system` の描画パス順序を検証するための軽量トレース。
//!
//! 環境変数 `ENGINE_CORE_RENDER_TRACE` が設定されているときのみ記録する（`0` / `false` は無効）。
//! `filament_smoke` など統合テストから利用する。

use std::sync::Mutex;

static PASSES: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

fn trace_enabled() -> bool {
    match std::env::var("ENGINE_CORE_RENDER_TRACE") {
        Ok(s) if s == "0" || s.eq_ignore_ascii_case("false") => false,
        Ok(_) => true,
        Err(_) => false,
    }
}

/// トレースバッファを空にする（テストの前処理）。
pub fn reset_render_pass_trace() {
    if let Ok(mut g) = PASSES.lock() {
        g.clear();
    }
}

pub(super) fn record_render_pass(name: &'static str) {
    if !trace_enabled() {
        return;
    }
    if let Ok(mut g) = PASSES.lock() {
        g.push(name);
    }
}

/// 記録済みパス名を取り出してバッファを空にする。
#[must_use]
pub fn take_render_pass_trace() -> Vec<String> {
    PASSES.lock().map(|mut g| g.drain(..).map(str::to_string).collect()).unwrap_or_default()
}
