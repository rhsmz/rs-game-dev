//! Component ストレージ。
//!
//! `SparseSet` ベースの高速コンポーネント格納。
//! O(1) の挿入・削除・参照を提供する。

use super::entity::Entity;
use std::any::Any;

/// Component マーカートレイト。
///
/// ECS で管理するすべてのデータ型はこのトレイトを実装する。
pub trait Component: Any + Send + Sync + 'static {}

/// 型消去された Component ストレージのインターフェース。
#[allow(dead_code)]
pub(crate) trait AnyComponentStorage: Send + Sync {
    /// Entity に紐づく Component を削除する。
    fn remove(&mut self, entity: Entity) -> bool;
    /// Entity をストレージが保持しているか確認する。
    fn contains(&self, entity: Entity) -> bool;
    /// `Any` への参照を返す（ダウンキャスト用）。
    fn as_any(&self) -> &dyn Any;
    /// `Any` への可変参照を返す（ダウンキャスト用）。
    fn as_any_mut(&mut self) -> &mut dyn Any;
    /// ストレージ内の変更フラグをすべてクリアする。
    fn clear_changed_flags(&mut self);
}

/// `SparseSet` ベースの型付き Component ストレージ。
pub struct ComponentStorage<T: Component> {
    /// Component データの連続配列
    dense: Vec<T>,
    /// dense インデックス → Entity のマッピング
    dense_to_entity: Vec<Entity>,
    /// Entity index → dense インデックスのマッピング (疎配列)
    sparse: Vec<Option<usize>>,
    /// 変更フラグ (dense と同じ順序)
    changed: Vec<bool>,
}

impl<T: Component> ComponentStorage<T> {
    /// 新しい空のストレージを作成する。
    #[must_use]
    pub fn new() -> Self {
        Self {
            dense: Vec::new(),
            dense_to_entity: Vec::new(),
            sparse: Vec::new(),
            changed: Vec::new(),
        }
    }

    /// Entity に Component を挿入する。既存の値は上書きされる。
    pub fn insert(&mut self, entity: Entity, component: T) {
        let idx = entity.index() as usize;

        // sparse 配列を必要に応じて拡張
        if idx >= self.sparse.len() {
            self.sparse.resize(idx + 1, None);
        }

        if let Some(dense_idx) = self.sparse[idx] {
            // 既存値を上書き
            self.dense[dense_idx] = component;
            self.changed[dense_idx] = true;
        } else {
            // 新規挿入
            let dense_idx = self.dense.len();
            self.dense.push(component);
            self.dense_to_entity.push(entity);
            self.sparse[idx] = Some(dense_idx);
            self.changed.push(true);
        }
    }

    /// Entity の Component を取得する。
    #[must_use]
    pub fn get(&self, entity: Entity) -> Option<&T> {
        let idx = entity.index() as usize;
        self.sparse.get(idx).and_then(|opt| opt.map(|dense_idx| &self.dense[dense_idx]))
    }

    /// Entity の Component を可変参照で取得する。
    ///
    /// 可変参照を取得すると、この Component は「変更済み」としてマークされる。
    #[must_use]
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let idx = entity.index() as usize;
        self.sparse.get(idx).copied().flatten().map(|dense_idx| {
            self.changed[dense_idx] = true;
            &mut self.dense[dense_idx]
        })
    }

    /// Entity の Component を削除する。
    pub fn remove_component(&mut self, entity: Entity) -> Option<T> {
        let idx = entity.index() as usize;
        if let Some(dense_idx) = self.sparse.get(idx).copied().flatten() {
            // swap-remove で O(1) 削除
            self.sparse[idx] = None;
            let last_dense = self.dense.len() - 1;

            if dense_idx != last_dense {
                // 末尾要素を削除位置に移動
                let moved_entity = self.dense_to_entity[last_dense];
                self.sparse[moved_entity.index() as usize] = Some(dense_idx);
                self.dense_to_entity[dense_idx] = moved_entity;
                self.changed[dense_idx] = self.changed[last_dense];
            }

            self.dense_to_entity.pop();
            self.changed.pop();
            Some(self.dense.swap_remove(dense_idx))
        } else {
            None
        }
    }

    /// ストレージ内の全 Entity と Component をイテレートする。
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.dense_to_entity.iter().copied().zip(self.dense.iter())
    }

    /// ストレージ内の全 Entity と Component を可変参照でイテレートする。
    ///
    /// このイテレータから取得した Component はすべて「変更済み」としてマークされる。
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        for flag in &mut self.changed {
            *flag = true;
        }
        self.dense_to_entity.iter().copied().zip(self.dense.iter_mut())
    }

    /// 「変更済み」とマークされた Entity と Component のみイテレートする。
    pub fn iter_changed(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.dense.iter().enumerate().filter_map(|(i, comp)| {
            if self.changed[i] { Some((self.dense_to_entity[i], comp)) } else { None }
        })
    }

    /// ストレージ内の変更フラグをすべてクリアする。
    pub fn clear_changed_flags(&mut self) {
        for flag in &mut self.changed {
            *flag = false;
        }
    }

    /// ストレージ内の Component 数を返す。
    #[must_use]
    pub fn len(&self) -> usize {
        self.dense.len()
    }

    /// ストレージが空かどうかを返す。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }
}

impl<T: Component> Default for ComponentStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Component> AnyComponentStorage for ComponentStorage<T> {
    fn remove(&mut self, entity: Entity) -> bool {
        self.remove_component(entity).is_some()
    }

    fn contains(&self, entity: Entity) -> bool {
        let idx = entity.index() as usize;
        self.sparse.get(idx).and_then(|opt| *opt).is_some()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clear_changed_flags(&mut self) {
        ComponentStorage::clear_changed_flags(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }
    impl Component for Position {}

    fn entity(index: u32, generation: u32) -> Entity {
        Entity { index, generation }
    }

    #[test]
    fn test_insert_and_get() {
        let mut storage = ComponentStorage::<Position>::new();
        let e = entity(0, 0);
        storage.insert(e, Position { x: 1.0, y: 2.0 });

        let pos = storage.get(e);
        assert!(pos.is_some());
        let pos = pos.unwrap();
        assert!((pos.x - 1.0).abs() < f32::EPSILON);
        assert!((pos.y - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_overwrite() {
        let mut storage = ComponentStorage::<Position>::new();
        let e = entity(0, 0);
        storage.insert(e, Position { x: 1.0, y: 2.0 });
        storage.insert(e, Position { x: 3.0, y: 4.0 });

        let pos = storage.get(e).unwrap();
        assert!((pos.x - 3.0).abs() < f32::EPSILON);
        assert_eq!(storage.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut storage = ComponentStorage::<Position>::new();
        let e0 = entity(0, 0);
        let e1 = entity(1, 0);
        storage.insert(e0, Position { x: 1.0, y: 1.0 });
        storage.insert(e1, Position { x: 2.0, y: 2.0 });

        let removed = storage.remove_component(e0);
        assert!(removed.is_some());
        assert!(storage.get(e0).is_none());
        assert!(storage.get(e1).is_some());
        assert_eq!(storage.len(), 1);
    }

    #[test]
    fn test_iter() {
        let mut storage = ComponentStorage::<Position>::new();
        storage.insert(entity(0, 0), Position { x: 1.0, y: 1.0 });
        storage.insert(entity(1, 0), Position { x: 2.0, y: 2.0 });

        let items: Vec<_> = storage.iter().collect();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_get_nonexistent_returns_none() {
        let storage = ComponentStorage::<Position>::new();
        assert!(storage.get(entity(99, 0)).is_none());
    }

    #[test]
    fn test_change_detection_on_insert() {
        let mut storage = ComponentStorage::<Position>::new();
        let e = entity(0, 0);
        storage.insert(e, Position { x: 0.0, y: 0.0 });

        let changed: Vec<_> = storage.iter_changed().collect();
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0].0, e);
    }

    #[test]
    fn test_change_detection_on_get_mut() {
        let mut storage = ComponentStorage::<Position>::new();
        let e = entity(0, 0);
        storage.insert(e, Position { x: 0.0, y: 0.0 });
        storage.clear_changed_flags();

        // get_mut を呼ぶと変更フラグが立つ
        storage.get_mut(e).unwrap().x = 1.0;

        let changed: Vec<_> = storage.iter_changed().collect();
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0].1.x, 1.0);
    }

    #[test]
    fn test_clear_changed_flags() {
        let mut storage = ComponentStorage::<Position>::new();
        let e = entity(0, 0);
        storage.insert(e, Position { x: 0.0, y: 0.0 });

        storage.clear_changed_flags();
        assert_eq!(storage.iter_changed().count(), 0);
    }

    #[test]
    fn test_iter_mut_marks_all_as_changed() {
        let mut storage = ComponentStorage::<Position>::new();
        storage.insert(entity(0, 0), Position { x: 1.0, y: 1.0 });
        storage.insert(entity(1, 0), Position { x: 2.0, y: 2.0 });
        storage.clear_changed_flags();

        storage.iter_mut().for_each(|(_, pos)| pos.x += 10.0);

        assert_eq!(storage.iter_changed().count(), 2);
    }

    #[test]
    fn test_remove_preserves_changed_flag_of_swapped_element() {
        let mut storage = ComponentStorage::<Position>::new();
        let e0 = entity(0, 0);
        let e1 = entity(1, 0);
        storage.insert(e0, Position { x: 1.0, y: 1.0 }); // changed: true
        storage.insert(e1, Position { x: 2.0, y: 2.0 }); // changed: true
        storage.clear_changed_flags(); // e0: false, e1: false

        let _ = storage.get_mut(e1); // e0: false, e1: true
        storage.remove_component(e0); // e1 が e0 の位置に移動

        let items: Vec<_> = storage.iter_changed().collect();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].0, e1);
    }
}
