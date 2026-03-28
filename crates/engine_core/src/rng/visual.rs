//! 演出用の高速 RNG（Xoshiro256++）。シードはスレッド RNG から非決定論的に採取する。

use rand::RngCore;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

/// 演出・パーティクル等向けの非決定論的起動シードを持つ RNG。
#[derive(Clone, Debug)]
pub struct VisualRng(Xoshiro256PlusPlus);

impl VisualRng {
    /// スレッドローカル RNG からシードを採取して生成する。
    ///
    /// `from_rng` が失敗した場合のみフォールバック定数シードを使う（通常は発生しない）。
    #[must_use]
    pub fn new() -> Self {
        let mut thread = rand::thread_rng();
        let inner = Xoshiro256PlusPlus::from_rng(&mut thread)
            .unwrap_or_else(|_| Xoshiro256PlusPlus::seed_from_u64(0xF0E1_D2C3_B4A5_9678));
        Self(inner)
    }

    /// テストや再生性が必要なプレビュー向けに、固定シードから生成する。
    #[must_use]
    pub fn from_seed_bytes(seed: [u8; 32]) -> Self {
        Self(Xoshiro256PlusPlus::from_seed(seed))
    }
}

impl Default for VisualRng {
    fn default() -> Self {
        Self::new()
    }
}

impl RngCore for VisualRng {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.0.try_fill_bytes(dest)
    }
}

#[cfg(test)]
mod tests {
    use rand::RngCore;

    use super::*;

    #[test]
    fn test_visual_rng_from_seed_is_deterministic() {
        let seed = [7u8; 32];
        let mut a = VisualRng::from_seed_bytes(seed);
        let mut b = VisualRng::from_seed_bytes(seed);
        assert_eq!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn test_visual_rng_new_runs() {
        let mut v = VisualRng::new();
        let _ = v.next_u64();
    }
}
