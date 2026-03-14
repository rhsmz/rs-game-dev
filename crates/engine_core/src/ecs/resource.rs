//! グローバルリソース管理。
//!
//! ECS World に登録するシングルトンリソース (AudioManager, InputState 等) を
//! 型安全に管理する。

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// 型安全なグローバルリソースストア。
pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Resources {
    /// 新しい空のリソースストアを作成する。
    #[must_use]
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    /// リソースを登録する。同じ型のリソースが既に存在する場合は上書きされる。
    pub fn insert<T: Send + Sync + 'static>(&mut self, resource: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(resource));
    }

    /// リソースへの参照を取得する。
    #[must_use]
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>()).and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// リソースへの可変参照を取得する。
    #[must_use]
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.data.get_mut(&TypeId::of::<T>()).and_then(|boxed| boxed.downcast_mut::<T>())
    }

    /// リソースを削除して返す。
    pub fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        self.data
            .remove(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast::<T>().ok())
            .map(|boxed| *boxed)
    }

    /// リソースが登録されているか確認する。
    #[must_use]
    pub fn contains<T: Send + Sync + 'static>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<T>())
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Counter(u32);
    struct Name(String);

    #[test]
    fn test_insert_and_get() {
        let mut res = Resources::new();
        res.insert(Counter(42));
        res.insert(Name("test".to_string()));

        assert_eq!(res.get::<Counter>().unwrap().0, 42);
        assert_eq!(res.get::<Name>().unwrap().0, "test");
    }

    #[test]
    fn test_get_mut() {
        let mut res = Resources::new();
        res.insert(Counter(0));

        res.get_mut::<Counter>().unwrap().0 += 10;
        assert_eq!(res.get::<Counter>().unwrap().0, 10);
    }

    #[test]
    fn test_remove() {
        let mut res = Resources::new();
        res.insert(Counter(99));

        let removed = res.remove::<Counter>();
        assert_eq!(removed.unwrap().0, 99);
        assert!(!res.contains::<Counter>());
    }

    #[test]
    fn test_get_nonexistent_returns_none() {
        let res = Resources::new();
        assert!(res.get::<Counter>().is_none());
    }
}
