//! ロジック用決定論的 RNG（ChaCha8）。

use rand::RngCore;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

/// ロジック RNG のシード（セーブデータやシーン間バケツリレー用）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogicSeed(pub [u8; 32]);

impl LogicSeed {
    /// 先頭 8 バイトに `v` を little-endian で格納し、残りを 0 で埋めたシード。
    #[must_use]
    pub fn from_u64(v: u64) -> Self {
        let mut s = [0u8; 32];
        s[..8].copy_from_slice(&v.to_le_bytes());
        Self(s)
    }

    /// シードの生バイト列。
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// 親シードと文脈ラベルから子シードを決定論的に導出する。
///
/// 同じ `(parent, salt)` からは常に同じ [`LogicSeed`] が得られる。
/// シーン遷移時に `salt` にシーン ID やカウンタを渡す想定。
#[must_use]
pub fn derive_child_seed(parent: &LogicSeed, salt: u64) -> LogicSeed {
    let mut rng = ChaCha8Rng::from_seed(parent.0);
    rng.set_stream(salt);
    let mut out = [0u8; 32];
    rng.fill_bytes(&mut out);
    LogicSeed(out)
}

/// ロジック・リプレイ用の `ChaCha8` RNG。
#[derive(Clone, Debug)]
pub struct LogicRng {
    inner: ChaCha8Rng,
}

impl LogicRng {
    /// 指定シードから生成する。
    #[must_use]
    pub fn from_seed(seed: LogicSeed) -> Self {
        Self { inner: ChaCha8Rng::from_seed(seed.0) }
    }

    /// 現在のキー（32 バイト）。ストリーム位置とは独立。
    #[must_use]
    pub fn key_seed(&self) -> LogicSeed {
        LogicSeed(self.inner.get_seed())
    }
}

impl RngCore for LogicRng {
    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.inner.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.inner.try_fill_bytes(dest)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn test_logic_rng_same_seed_same_sequence() {
        let seed = LogicSeed::from_u64(0x1234_5678_ABCD_EF01);
        let mut a = LogicRng::from_seed(seed);
        let mut b = LogicRng::from_seed(seed);
        for _ in 0..256 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn test_derive_child_seed_deterministic() {
        let parent = LogicSeed::from_u64(42);
        let c0 = derive_child_seed(&parent, 1);
        let c1 = derive_child_seed(&parent, 1);
        let c2 = derive_child_seed(&parent, 2);
        assert_eq!(c0, c1);
        assert_ne!(c0, c2);
    }

    // 親→子→孫の 3 段鎖。`salt` はシーン ID 等の文脈ラベル想定（A4-2）。
    #[test]
    fn test_derive_child_seed_parent_child_grandchild_chain() {
        let root = LogicSeed::from_u64(0xDEAD_BEEF_0000_0001);
        let scene_a: u64 = 100; // 遷移先シーン A
        let scene_b: u64 = 200; // A から B へ

        let child = derive_child_seed(&root, scene_a);
        let grandchild = derive_child_seed(&child, scene_b);

        // 同一入力なら常に同一出力（リプレイ再現性）
        assert_eq!(grandchild, derive_child_seed(&derive_child_seed(&root, scene_a), scene_b));
        // 中間 salt を変えると孫は変わる
        assert_ne!(grandchild, derive_child_seed(&derive_child_seed(&root, scene_a ^ 1), scene_b));
    }

    #[test]
    fn test_logic_rng_uniform_buckets_loose() {
        const BINS: usize = 8;
        const SAMPLES: usize = 12_000;
        let mut rng = LogicRng::from_seed(LogicSeed::from_u64(0x00C0_FFEE));
        let mut counts = [0usize; BINS];
        for _ in 0..SAMPLES {
            let v = (rng.next_u32() as usize) % BINS;
            counts[v] += 1;
        }
        let expected = SAMPLES / BINS;
        let margin = expected / 4;
        for c in counts {
            assert!(
                c > expected.saturating_sub(margin) && c < expected + margin,
                "bin count {c} expected near {expected} (±{margin})"
            );
        }
    }
}
