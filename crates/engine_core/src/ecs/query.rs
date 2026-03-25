//! Component クエリシステム。
//!
//! `World` を対象に、特定の Component を持つ Entity の集まりを検索し、
//! 同時に複数 Component の参照（または可変参照）を取得する仕組みを提供する。

use std::marker::PhantomData;

use super::component::Component;
use super::entity::Entity;
use super::world::World;

mod private {
    pub trait Sealed {}
}

/// 特定の Component が存在することを要求するフィルタ。
#[derive(Debug, Default)]
pub struct With<T: super::component::Component>(std::marker::PhantomData<T>);

/// 特定の Component が存在しないことを要求するフィルタ。
#[derive(Debug, Default)]
pub struct Without<T: super::component::Component>(std::marker::PhantomData<T>);

// --- クエリパラメータトレイト ---

/// クエリパラメータとなる型のトレイト。
pub trait QueryParam: private::Sealed {
    type Item<'a>;

    /// 可変のデータをフェッチする。
    fn fetch_mut(world: &mut World, entity: Entity) -> Option<Self::Item<'_>>;

    /// クエリ対象となる Entity のイテレータを返す。
    /// 最も効率的な `ComponentStorage` を選択してイテレートする戦略を担う。
    fn iter_entities<'a>(world: &'a World) -> Box<dyn Iterator<Item = Entity> + 'a>;
}

/// 読み取り専用のクエリパラメータを表すトレイト。
pub trait ReadOnlyQueryParam: QueryParam {
    /// 読み取り専用のデータをフェッチする。
    fn fetch(world: &World, entity: Entity) -> Option<Self::Item<'_>>;
}

// --- 読み取り専用クエリの実装 ---

/// 単一 Component への読み取り専用参照
impl<T: Component> private::Sealed for &T {}

impl<T: Component> QueryParam for &T {
    type Item<'b> = &'b T;

    fn fetch_mut(world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        world.get_storage::<T>()?.get(entity)
    }

    fn iter_entities<'b>(world: &'b World) -> Box<dyn Iterator<Item = Entity> + 'b> {
        match world.get_storage::<T>() {
            Some(storage) => Box::new(storage.iter().map(|(e, _)| e)),
            None => Box::new(std::iter::empty()),
        }
    }
}

impl<T: Component> ReadOnlyQueryParam for &T {
    fn fetch(world: &World, entity: Entity) -> Option<Self::Item<'_>> {
        world.get_storage::<T>()?.get(entity)
    }
}

/// フィルタ `With<T>` の実装
impl<T: Component> private::Sealed for With<T> {}

impl<T: Component> QueryParam for With<T> {
    type Item<'a> = ();

    fn fetch_mut(world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        if world.get_component::<T>(entity).is_some() { Some(()) } else { None }
    }

    fn iter_entities<'a>(world: &'a World) -> Box<dyn Iterator<Item = Entity> + 'a> {
        match world.get_storage::<T>() {
            Some(storage) => Box::new(storage.iter().map(|(e, _)| e)),
            None => Box::new(std::iter::empty()),
        }
    }
}

impl<T: Component> ReadOnlyQueryParam for With<T> {
    fn fetch(world: &World, entity: Entity) -> Option<Self::Item<'_>> {
        if world.get_component::<T>(entity).is_some() { Some(()) } else { None }
    }
}

/// フィルタ `Without<T>` の実装
impl<T: Component> private::Sealed for Without<T> {}

impl<T: Component> QueryParam for Without<T> {
    type Item<'a> = ();

    fn fetch_mut(world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        if world.get_component::<T>(entity).is_none() { Some(()) } else { None }
    }

    fn iter_entities<'a>(world: &'a World) -> Box<dyn Iterator<Item = Entity> + 'a> {
        Box::new(world.iter_entities())
    }
}

impl<T: Component> ReadOnlyQueryParam for Without<T> {
    fn fetch(world: &World, entity: Entity) -> Option<Self::Item<'_>> {
        if world.get_component::<T>(entity).is_none() { Some(()) } else { None }
    }
}

/// 2つの Component への読み取り専用参照 (2要素タプル)
impl<'a, T1: Component, T2: Component> private::Sealed for (&'a T1, &'a T2) {}

impl<'a, T1: Component, T2: Component> QueryParam for (&'a T1, &'a T2) {
    type Item<'b> = (&'b T1, &'b T2);

    fn fetch_mut(world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        // 両方とも読み取り専用なので安全にフェッチ可能
        Some((world.get_component::<T1>(entity)?, world.get_component::<T2>(entity)?))
    }

    fn iter_entities<'b>(world: &'b World) -> Box<dyn Iterator<Item = Entity> + 'b> {
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

impl<'a, T1: Component, T2: Component> ReadOnlyQueryParam for (&'a T1, &'a T2) {
    fn fetch(world: &World, entity: Entity) -> Option<Self::Item<'_>> {
        Some((world.get_component::<T1>(entity)?, world.get_component::<T2>(entity)?))
    }
}

/// データのフェッチ ＋ With フィルタ (読み取り専用)
impl<T1: Component, T2: Component> private::Sealed for (&T1, With<T2>) {}

impl<T1: Component, T2: Component> QueryParam for (&T1, With<T2>) {
    type Item<'b> = (&'b T1, ());

    fn fetch_mut(world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        world.get_component::<T2>(entity)?;
        Some((world.get_component::<T1>(entity)?, ()))
    }

    fn iter_entities<'b>(world: &'b World) -> Box<dyn Iterator<Item = Entity> + 'b> {
        match world.get_storage::<T1>() {
            Some(s1) => Box::new(
                s1.iter().map(|(e, _)| e).filter(move |e| world.get_component::<T2>(*e).is_some()),
            ),
            None => Box::new(std::iter::empty()),
        }
    }
}

impl<T1: Component, T2: Component> ReadOnlyQueryParam for (&T1, With<T2>) {
    fn fetch(world: &World, entity: Entity) -> Option<Self::Item<'_>> {
        world.get_component::<T2>(entity)?;
        Some((world.get_component::<T1>(entity)?, ()))
    }
}

/// データのフェッチ ＋ Without フィルタ (読み取り専用)
impl<T1: Component, T2: Component> private::Sealed for (&T1, Without<T2>) {}

impl<T1: Component, T2: Component> QueryParam for (&T1, Without<T2>) {
    type Item<'b> = (&'b T1, ());

    fn fetch_mut(world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        if world.get_component::<T2>(entity).is_some() {
            return None;
        }
        Some((world.get_component::<T1>(entity)?, ()))
    }

    fn iter_entities<'b>(world: &'b World) -> Box<dyn Iterator<Item = Entity> + 'b> {
        match world.get_storage::<T1>() {
            Some(s1) => Box::new(
                s1.iter().map(|(e, _)| e).filter(move |e| world.get_component::<T2>(*e).is_none()),
            ),
            None => Box::new(std::iter::empty()),
        }
    }
}

impl<T1: Component, T2: Component> ReadOnlyQueryParam for (&T1, Without<T2>) {
    fn fetch(world: &World, entity: Entity) -> Option<Self::Item<'_>> {
        if world.get_component::<T2>(entity).is_some() {
            return None;
        }
        Some((world.get_component::<T1>(entity)?, ()))
    }
}

/// データの可変フェッチ ＋ With フィルタ
impl<T1: Component, T2: Component> private::Sealed for (&mut T1, With<T2>) {}

impl<T1: Component, T2: Component> QueryParam for (&mut T1, With<T2>) {
    type Item<'b> = (&'b mut T1, ());

    fn fetch_mut(world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        world.get_component::<T2>(entity)?;
        let item1 = world.get_storage_mut::<T1>()?.get_mut(entity)?;
        Some((item1, ()))
    }

    fn iter_entities<'b>(world: &'b World) -> Box<dyn Iterator<Item = Entity> + 'b> {
        match world.get_storage::<T1>() {
            Some(s1) => Box::new(
                s1.iter().map(|(e, _)| e).filter(move |e| world.get_component::<T2>(*e).is_some()),
            ),
            None => Box::new(std::iter::empty()),
        }
    }
}

/// データの可変フェッチ ＋ Without フィルタ
impl<T1: Component, T2: Component> private::Sealed for (&mut T1, Without<T2>) {}

impl<T1: Component, T2: Component> QueryParam for (&mut T1, Without<T2>) {
    type Item<'b> = (&'b mut T1, ());

    fn fetch_mut(world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        if world.get_component::<T2>(entity).is_some() {
            return None;
        }
        let item1 = world.get_storage_mut::<T1>()?.get_mut(entity)?;
        Some((item1, ()))
    }

    fn iter_entities<'b>(world: &'b World) -> Box<dyn Iterator<Item = Entity> + 'b> {
        match world.get_storage::<T1>() {
            Some(s1) => Box::new(
                s1.iter().map(|(e, _)| e).filter(move |e| world.get_component::<T2>(*e).is_none()),
            ),
            None => Box::new(std::iter::empty()),
        }
    }
}

// --- 可変クエリの実装 ---

/// 単一 Component への可変参照
impl<T: Component> private::Sealed for &mut T {}

impl<T: Component> QueryParam for &mut T {
    type Item<'b> = &'b mut T;

    fn fetch_mut(world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        world.get_storage_mut::<T>()?.get_mut(entity)
    }

    fn iter_entities<'b>(world: &'b World) -> Box<dyn Iterator<Item = Entity> + 'b> {
        match world.get_storage::<T>() {
            Some(storage) => Box::new(storage.iter().map(|(e, _)| e)),
            None => Box::new(std::iter::empty()),
        }
    }
}

// --- クエリ実行オブジェクト ---

/// クエリ実行オブジェクト。
pub struct Query<'w, Q: ReadOnlyQueryParam> {
    world: &'w World,
    _marker: PhantomData<Q>,
}

pub struct QueryMut<'w, Q: QueryParam> {
    world: &'w mut World,
    _marker: PhantomData<Q>,
}

impl<'w, Q: ReadOnlyQueryParam> Query<'w, Q> {
    #[must_use]
    pub const fn new(world: &'w World) -> Self {
        Self { world, _marker: PhantomData }
    }

    /// クエリにマッチする Entity と Component のイテレータを返す。
    pub fn iter(&self) -> impl Iterator<Item = Q::Item<'w>> + 'w {
        Q::iter_entities(self.world).filter_map(|entity| Q::fetch(self.world, entity))
    }

    /// 指定した Entity に対してクエリを実行する。
    #[must_use]
    pub fn get(&self, entity: Entity) -> Option<Q::Item<'w>> {
        Q::fetch(self.world, entity)
    }
}

impl<'w, Q: QueryParam> QueryMut<'w, Q> {
    pub const fn new(world: &'w mut World) -> Self {
        Self { world, _marker: PhantomData }
    }

    /// クエリにマッチする Entity と Component の可変イテレータを返す。
    pub fn iter_mut(&mut self) -> impl Iterator<Item = Q::Item<'_>> + '_ {
        // SAFETY:
        // `iter_mut` は `&mut self` を借用するが、内部で `World` への不変参照と可変参照の両方が必要になる。
        let world_ptr = self.world as *mut World;

        // まず、生ポインタから不変参照を作成し、エンティティを収集する。
        let entities: Vec<Entity> = unsafe { Q::iter_entities(&*world_ptr).collect() };

        // 次に、収集したエンティティをイテレートし、クロージャ内で生ポインタから可変参照を作成する。
        entities.into_iter().filter_map(move |entity| {
            // この可変参照は `fetch_mut` の呼び出し中のみ有効。
            unsafe { Q::fetch_mut(&mut *world_ptr, entity) }
        })
    }

    /// 指定した Entity に対して可変クエリを実行する。
    pub fn get_mut(&mut self, entity: Entity) -> Option<Q::Item<'_>> {
        // SAFETY: `iter_mut` と同様の理由で unsafe が必要。
        // `&mut self` から引き上げた world_ptr を使うことで、借用期間をこの関数呼び出しの生存期間に限定する。
        let world_ptr = self.world as *mut World;
        unsafe { Q::fetch_mut(&mut *world_ptr, entity) }
    }
}

/// World のための Query 拡張メソッド
pub trait WorldQueryExt {
    fn query<Q: ReadOnlyQueryParam>(&self) -> Query<'_, Q>;
    fn query_mut<Q: QueryParam>(&mut self) -> QueryMut<'_, Q>;
}

impl WorldQueryExt for World {
    fn query<Q: ReadOnlyQueryParam>(&self) -> Query<'_, Q> {
        Query::new(self)
    }
    fn query_mut<Q: QueryParam>(&mut self) -> QueryMut<'_, Q> {
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

    #[test]
    fn test_with_filter() {
        let mut world = World::new();
        let e1 = world.spawn();
        world.insert_component(e1, Position { x: 1.0, y: 1.0 });
        world.insert_component(e1, Velocity { dx: 0.1, dy: 0.1 });

        let e2 = world.spawn();
        world.insert_component(e2, Position { x: 2.0, y: 2.0 });

        let results: Vec<_> =
            world.query::<(&Position, With<Velocity>)>().iter().map(|(pos, _)| pos.x).collect();
        assert_eq!(results, vec![1.0]);
    }

    #[test]
    fn test_without_filter() {
        let mut world = World::new();
        let e1 = world.spawn();
        world.insert_component(e1, Position { x: 1.0, y: 1.0 });
        world.insert_component(e1, Velocity { dx: 0.1, dy: 0.1 });

        let e2 = world.spawn();
        world.insert_component(e2, Position { x: 2.0, y: 2.0 });

        let results: Vec<_> =
            world.query::<(&Position, Without<Velocity>)>().iter().map(|(pos, _)| pos.x).collect();
        assert_eq!(results, vec![2.0]);
    }

    #[test]
    fn test_standalone_without() {
        let mut world = World::new();
        let e1 = world.spawn();
        world.insert_component(e1, Position { x: 1.0, y: 1.0 });

        let _e2 = world.spawn(); // empty

        let count = world.query::<Without<Position>>().iter().count();
        assert_eq!(count, 1);
    }
}
