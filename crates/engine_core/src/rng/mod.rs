//! Dual RNG（乱数）モジュール。
//!
//! - [`VisualRng`] — `Xoshiro256++` ベース。起動時シードはスレッド RNG 由来で**非決定論的**（演出用）。
//! - [`LogicRng`] — `ChaCha8` ベース。**同一シードなら同一列**（ロジック・リプレイ用）。
//! - [`derive_child_seed`] — シーン遷移などで親 [`LogicSeed`] から子シードを決定論的に導出。
//!
//! ECS では [`crate::ecs::World::insert_resource`] にそれぞれ登録して利用する。

mod logic;
mod visual;

pub use logic::{LogicRng, LogicSeed, derive_child_seed};
pub use visual::VisualRng;

#[cfg(test)]
mod world_tests {
    #![allow(clippy::unwrap_used)]

    use rand::RngCore;

    use crate::ecs::World;

    use super::{LogicRng, LogicSeed, VisualRng};

    #[test]
    fn test_world_inserts_dual_rng_resources() {
        let mut world = World::new();
        world.insert_resource(LogicRng::from_seed(LogicSeed::from_u64(1)));
        world.insert_resource(VisualRng::from_seed_bytes([9u8; 32]));

        assert!(world.get_resource::<LogicRng>().is_some());
        assert!(world.get_resource::<VisualRng>().is_some());

        let v_world = {
            let r = world.get_resource_mut::<LogicRng>().unwrap();
            r.next_u64()
        };
        let mut standalone = LogicRng::from_seed(LogicSeed::from_u64(1));
        assert_eq!(v_world, standalone.next_u64());
    }
}
