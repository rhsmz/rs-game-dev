//! Component クエリシステム。
//!
//! `World` を対象に、特定の Component を持つ Entity の集まりを検索し、
//! 同時に複数 Component の参照（または可変参照）を取得する仕組みを提供する。

use std::marker::PhantomData;

use super::component::Component;
use super::entity::Entity;
use super::world::World;

// --- クエリパラメータトレイト ---

/// クエリパラメータとなる型のトレイト。
/// GAT (Generic Associated Types) を用いて、fetch の結果のライフタイムを 'a に紐づける。
pub trait QueryParam<'a> {
    type Item;

    /// 読み取り専用のデータをフェッチする。
    fn fetch(world: &'a World, entity: Entity) -> Option<Self::Item>;

    /// 可変のデータをフェッチする。
    fn fetch_mut(world: &'a mut World, entity: Entity) -> Option<Self::Item>
    where
        Self: Sized,
    {
        // デフォルトでは読み取り専用として動作
        Self::fetch(world, entity)
    }

    /// クエリ対象となる Entity のイテレータを返す。
    /// 最も効率的な ComponentStorage を選択してイテレートする戦略を担う。
    fn iter_entities(world: &'a World) -> Box<dyn Iterator<Item = Entity> + 'a>;
}

// --- 読み取り専用クエリの実装 ---

/// 単一 Component への読み取り専用参照
impl<'a, T: Component> QueryParam<'a> for &'a T {
    type Item = &'a T;

    fn fetch(world: &'a World, entity: Entity) -> Option<Self::Item> {
        world.get_storage::<T>()?.get(entity)
    }

    fn iter_entities(world: &'a World) -> Box<dyn Iterator<Item = Entity> + 'a> {
        match world.get_storage::<T>() {
            Some(storage) => Box::new(storage.iter().map(|(e, _)| e)),
            None => Box::new(std::iter::empty()),
        }
    }
}

/// 2つの Component への読み取り専用参照
impl<'a, T1: Component, T2: Component> QueryParam<'a> for (&'a T1, &'a T2) {
    type Item = (&'a T1, &'a T2);

    fn fetch(world: &'a World, entity: Entity) -> Option<Self::Item> {
        Some((world.get_component::<T1>(entity)?, world.get_component::<T2>(entity)?))
    }

    fn iter_entities(world: &'a World) -> Box<dyn Iterator<Item = Entity> + 'a> {
        let s1 = world.get_storage::<T1>();
        let s2 = world.get_storage::<T2>();

        match (s1, s2) {
            (Some(s1), Some(s2)) => {
                if s1.len() <= s2.len() {
                    Box::new(s1.iter().map(|(e, _)| e).filter(move |e| s2.get(*e).is_some()))
                } else {
                    Box::new(s2.iter().map(|(e, _)| e).filter(move |e| s1.get(*e).is_some()))
                }
            }
            _ => Box::new(std::iter::empty()),
        }
    }
}

// --- 可変クエリの実装 ---

/// 単一 Component への可変参照
impl<'a, T: Component> QueryParam<'a> for &'a mut T {
    type Item = &'a mut T;

    fn fetch(_world: &'a World, _entity: Entity) -> Option<Self::Item> {
        // 可変参照を &World からは取得できない
        panic!("Cannot fetch mutable data from an immutable world reference.");
    }

    fn fetch_mut(world: &'a mut World, entity: Entity) -> Option<Self::Item> {
        world.get_storage_mut::<T>()?.get_mut(entity)
    }

    fn iter_entities(world: &'a World) -> Box<dyn Iterator<Item = Entity> + 'a> {
        match world.get_storage::<T>() {
            Some(storage) => Box::new(storage.iter().map(|(e, _)| e)),
            None => Box::new(std::iter::empty()),
        }
    }
}

// --- クエリ実行オブジェクト ---

/// クエリ実行オブジェクト。
pub struct Query<'w, Q: QueryParam<'w>> {
    world: &'w World,
    _marker: PhantomData<Q>,
}

pub struct QueryMut<'w, Q: QueryParam<'w>> {
    world: &'w mut World,
    _marker: PhantomData<Q>,
}

impl<'w, Q: QueryParam<'w>> Query<'w, Q> {
    pub fn new(world: &'w World) -> Self {
        Self { world, _marker: PhantomData }
    }

    /// クエリにマッチする Entity と Component のイテレータを返す。
    pub fn iter(&self) -> impl Iterator<Item = Q::Item> + 'w {
        Q::iter_entities(self.world).filter_map(|entity| Q::fetch(self.world, entity))
    }

    /// 指定した Entity に対してクエリを実行する。
    pub fn get(&self, entity: Entity) -> Option<Q::Item> {
        Q::fetch(self.world, entity)
    }
}

impl<'w, Q: QueryParam<'w>> QueryMut<'w, Q> {
    pub fn new(world: &'w mut World) -> Self {
        Self { world, _marker: PhantomData }
    }

    /// クエリにマッチする Entity と Component の可変イテレータを返す。
    pub fn iter_mut(&mut self) -> impl Iterator<Item = Q::Item> + 'w {
        // SAFETY:
        // `iter_mut` は `&mut self` を借用するが、内部で `World` への不変参照と可変参照の両方が必要になる。
        // 1. `Q::iter_entities` でエンティティリストを収集 (不変参照)
        // 2. `Q::fetch_mut` で各エンティティのコンポーネントをフェッチ (可変参照)
        // このパターンは Rust の借用チェッカーでは安全だと証明できないため、`unsafe` と生ポインタを使用する。
        let world_ptr = self.world as *mut World;

        // まず、生ポインタから不変参照を作成し、エンティティを収集する。
        // この参照のライフタイムはこの `let` 文に限定される。
        let entities: Vec<Entity> = unsafe { Q::iter_entities(&*world_ptr).collect() };

        // 次に、収集したエンティティをイテレートし、クロージャ内で生ポインタから可変参照を作成する。
        // クロージャは `self` ではなく `world_ptr` のみをキャプチャするため、借用チェッカーの問題を回避できる。
        entities.into_iter().filter_map(move |entity| {
            // この可変参照は `fetch_mut` の呼び出し中のみ有効。
            // エンティティは一意であるため、エイリアスとなる可変参照は作成されない。
            unsafe { Q::fetch_mut(&mut *world_ptr, entity) }
        })
    }

    /// 指定した Entity に対して可変クエリを実行する。
    pub fn get_mut(&mut self, entity: Entity) -> Option<Q::Item> {
        // SAFETY: `iter_mut` と同様の理由で unsafe が必要。
        // `&mut self` のライフタイムは `'w` より短いが、`QueryMut` が存在する限り
        // `world` は有効であり、返される参照もその範囲でのみ有効となるため安全。
        let world_ptr = self.world as *mut World;
        unsafe { Q::fetch_mut(&mut *world_ptr, entity) }
    }
}

/// World のための Query 拡張メソッド
pub trait WorldQueryExt {
    fn query<'w, Q: QueryParam<'w>>(&'w self) -> Query<'w, Q>;
    fn query_mut<'w, Q: QueryParam<'w>>(&'w mut self) -> QueryMut<'w, Q>;
}

impl WorldQueryExt for World {
    fn query<'w, Q: QueryParam<'w>>(&'w self) -> Query<'w, Q> {
        Query::new(self)
    }
    fn query_mut<'w, Q: QueryParam<'w>>(&'w mut self) -> QueryMut<'w, Q> {
        QueryMut::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Position {
        x: f32,
        y: f32,
    }
    impl Component for Position {}

    #[derive(Debug, PartialEq, Clone)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }
    impl Component for Velocity {}

    #[test]
    fn test_single_iter() {
        let mut world = World::new();
        let e1 = world.spawn();
        world.insert_component(e1, Position { x: 1.0, y: 1.0 });
        let e2 = world.spawn();
        world.insert_component(e2, Position { x: 2.0, y: 2.0 });
        let e3 = world.spawn();
        world.insert_component(e3, Velocity { dx: 0.0, dy: 0.0 });

        let mut count = 0;
        for pos in world.query::<&Position>().iter() {
            assert!(pos.x > 0.0);
            count += 1;
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_tuple_iter() {
        let mut world = World::new();
        let e1 = world.spawn();
        world.insert_component(e1, Position { x: 1.0, y: 1.0 });
        world.insert_component(e1, Velocity { dx: 0.1, dy: 0.1 });
        let e2 = world.spawn();
        world.insert_component(e2, Position { x: 2.0, y: 2.0 });

        let mut count = 0;
        for (pos, vel) in world.query::<(&Position, &Velocity)>().iter() {
            assert_eq!(pos.x, 1.0);
            assert_eq!(vel.dx, 0.1);
            count += 1;
        }
        assert_eq!(count, 1);
    }

    #[test]
    fn test_mutable_iter() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert_component(e, Position { x: 1.0, y: 1.0 });

        for pos in world.query_mut::<&mut Position>().iter_mut() {
            pos.x = 100.0;
        }

        let pos = world.get_component::<Position>(e).unwrap();
        assert_eq!(pos.x, 100.0);
    }

    #[test]
    fn test_mutable_get() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert_component(e, Position { x: 0.0, y: 0.0 });

        if let Some(pos) = world.query_mut::<&mut Position>().get_mut(e) {
            pos.y = 5.0;
        }

        assert_eq!(world.get_component::<Position>(e).unwrap().y, 5.0);
    }
}
