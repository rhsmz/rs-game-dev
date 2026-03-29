//! `render_system` のメッシュ縦スライス投入を検証するための軽量カウンタ。
//!
//! 環境変数 `ENGINE_CORE_MESH_SUBMIT_TRACE` が設定されているときのみ記録する（`0` / `false` は無効）。

use std::sync::Mutex;

#[derive(Debug, Default, Clone, Copy)]
struct MeshSubmitStats {
    skipped_zero: u32,
    attach_ok: u32,
    attach_fail: u32,
}

static STATS: Mutex<MeshSubmitStats> =
    Mutex::new(MeshSubmitStats { skipped_zero: 0, attach_ok: 0, attach_fail: 0 });

fn trace_enabled() -> bool {
    match std::env::var("ENGINE_CORE_MESH_SUBMIT_TRACE") {
        Ok(s) if s == "0" || s.eq_ignore_ascii_case("false") => false,
        Ok(_) => true,
        Err(_) => false,
    }
}

/// トレースカウンタをリセットする（テストの前処理）。
pub fn reset_mesh_submit_trace() {
    if let Ok(mut g) = STATS.lock() {
        *g = MeshSubmitStats::default();
    }
}

pub(super) fn record_skipped_unloaded_mesh() {
    if !trace_enabled() {
        return;
    }
    if let Ok(mut g) = STATS.lock() {
        g.skipped_zero = g.skipped_zero.saturating_add(1);
    }
}

pub(super) fn record_attach_result(ok: bool) {
    if !trace_enabled() {
        return;
    }
    if let Ok(mut g) = STATS.lock() {
        if ok {
            g.attach_ok = g.attach_ok.saturating_add(1);
        } else {
            g.attach_fail = g.attach_fail.saturating_add(1);
        }
    }
}

/// `(skipped_unloaded, attach_ok, attach_fail)` を取り出し、バッファをリセットする。
#[must_use]
pub fn take_mesh_submit_trace() -> (u32, u32, u32) {
    if !trace_enabled() {
        return (0, 0, 0);
    }
    STATS
        .lock()
        .map(|mut g| {
            let out = (g.skipped_zero, g.attach_ok, g.attach_fail);
            *g = MeshSubmitStats::default();
            out
        })
        .unwrap_or((0, 0, 0))
}
