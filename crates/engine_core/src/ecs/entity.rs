//! Entity の生成・破棄・世代管理。
//!
//! Generation + Index 方式で Entity ID を安全に再利用する。

/// ゲーム内のオブジェクトを表す一意な識別子。
///
/// Generation により、破棄済み Entity への参照を安全に検出できる。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    /// Entity のインデックス (スロット番号)
    pub(crate) index: u32,
    /// Entity の世代 (再利用検出用)
    pub(crate) generation: u32,
}

impl Entity {
    /// Entity のインデックスを取得する。
    #[must_use]
    pub const fn index(self) -> u32 {
        self.index
    }

    /// Entity の世代を取得する。
    #[must_use]
    pub const fn generation(self) -> u32 {
        self.generation
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity({}v{})", self.index, self.generation)
    }
}

/// Entity の割り当てと再利用を管理するアロケータ。
pub struct EntityAllocator {
    /// 各スロットの現在の世代
    generations: Vec<u32>,
    /// 再利用可能なインデックスのリスト
    free_list: Vec<u32>,
    /// 生存フラグ
    alive: Vec<bool>,
    /// 現在生存している Entity の数
    alive_count: usize,
}

impl EntityAllocator {
    /// 新しい `EntityAllocator` を作成する。
    #[must_use]
    pub const fn new() -> Self {
        Self { generations: Vec::new(), free_list: Vec::new(), alive: Vec::new(), alive_count: 0 }
    }

    /// 新しい Entity を割り当てる。
    ///
    /// # Panics
    /// - Entity 数が多すぎて `u32` の index に収まらない場合。
    pub fn allocate(&mut self) -> Entity {
        self.alive_count += 1;

        if let Some(index) = self.free_list.pop() {
            // 再利用: deallocate 時に世代はすでにインクリメント済み
            self.alive[index as usize] = true;
            Entity { index, generation: self.generations[index as usize] }
        } else {
            // 新規割り当て
            let index = u32::try_from(self.generations.len())
                .unwrap_or_else(|_| panic!("Entity index overflow: too many entities allocated"));
            self.generations.push(0);
            self.alive.push(true);
            Entity { index, generation: 0 }
        }
    }

    /// Entity を解放する。
    ///
    /// 既に解放済みの Entity を渡した場合は `false` を返す。
    pub fn deallocate(&mut self, entity: Entity) -> bool {
        let idx = entity.index as usize;
        if idx < self.generations.len() && self.generations[idx] == entity.generation {
            // 世代をインクリメントして古い参照を無効化
            self.generations[idx] += 1;
            self.free_list.push(entity.index);
            self.alive[idx] = false;
            self.alive_count -= 1;
            true
        } else {
            false
        }
    }

    /// Entity が現在生存しているか確認する。
    #[must_use]
    pub fn is_alive(&self, entity: Entity) -> bool {
        let idx = entity.index as usize;
        idx < self.generations.len()
            && self.alive.get(idx).copied().unwrap_or(false)
            && self.generations[idx] == entity.generation
    }

    /// 現在生存している Entity の数を返す。
    #[must_use]
    pub const fn alive_count(&self) -> usize {
        self.alive_count
    }

    /// 生存しているすべての Entity のイテレータを返す。
    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        let alive = &self.alive;
        let generations = &self.generations;

        (0..generations.len()).filter_map(move |idx| {
            if !alive[idx] {
                return None;
            }

            // allocator が index を `u32` に制限しているため、通常は必ず `Ok` になる。
            let index = u32::try_from(idx).ok()?;
            Some(Entity { index, generation: generations[idx] })
        })
    }
}

impl Default for EntityAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_sequential_entities() {
        let mut alloc = EntityAllocator::new();
        let e0 = alloc.allocate();
        let e1 = alloc.allocate();
        let e2 = alloc.allocate();

        assert_eq!(e0.index(), 0);
        assert_eq!(e1.index(), 1);
        assert_eq!(e2.index(), 2);
        assert_eq!(e0.generation(), 0);
        assert_eq!(alloc.alive_count(), 3);
    }

    #[test]
    fn test_deallocate_and_reuse() {
        let mut alloc = EntityAllocator::new();
        let e0 = alloc.allocate();
        let _e1 = alloc.allocate();

        assert!(alloc.deallocate(e0));
        assert!(!alloc.is_alive(e0));
        assert_eq!(alloc.alive_count(), 1);

        // 再利用: 同じインデックスだが世代が異なる
        let e2 = alloc.allocate();
        assert_eq!(e2.index(), 0);
        assert_eq!(e2.generation(), 1);
        assert_ne!(e0, e2);
        assert!(alloc.is_alive(e2));
    }

    #[test]
    fn test_double_deallocate_returns_false() {
        let mut alloc = EntityAllocator::new();
        let e0 = alloc.allocate();
        assert!(alloc.deallocate(e0));
        assert!(!alloc.deallocate(e0));
    }

    #[test]
    fn test_is_alive_detects_stale_reference() {
        let mut alloc = EntityAllocator::new();
        let e0 = alloc.allocate();
        assert!(alloc.is_alive(e0));

        alloc.deallocate(e0);
        assert!(!alloc.is_alive(e0));

        let e1 = alloc.allocate();
        // 古い参照はまだ無効
        assert!(!alloc.is_alive(e0));
        // 新しい参照は有効
        assert!(alloc.is_alive(e1));
    }

    #[test]
    fn test_entity_display() {
        let e = Entity { index: 42, generation: 3 };
        assert_eq!(format!("{e}"), "Entity(42v3)");
    }
}
