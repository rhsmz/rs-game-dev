//! ECS World — Entity と Component の統合管理。
//!
//! ゲーム内のすべてのオブジェクトをここで管理する。

use std::any::TypeId;
use std::collections::HashMap;

use super::component::{AnyComponentStorage, Component, ComponentStorage};
use super::entity::{Entity, EntityAllocator};
use super::event::{EventQueue, EventQueues, EventReader, EventWriter};
use super::resource::Resources;

/// ECS World。Entity・Component・Resource の統合コンテナ。
pub struct World {
    entities: EntityAllocator,
    components: HashMap<TypeId, Box<dyn AnyComponentStorage>>,
    resources: Resources,
    events: EventQueues,
}

impl World {
    /// 新しい空の World を作成する。
    #[must_use]
    pub fn new() -> Self {
        Self {
            entities: EntityAllocator::new(),
            components: HashMap::new(),
            resources: Resources::new(),
            events: EventQueues::new(),
        }
    }

    // ── Entity 操作 ──

    /// 新しい Entity を生成する。
    pub fn spawn(&mut self) -> Entity {
        self.entities.allocate()
    }

    /// 生存しているすべての Entity のイテレータを返す。
    pub fn iter_entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter()
    }

    /// Entity を破棄し、紐づく全 Component を削除する。
    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.entities.deallocate(entity) {
            return false;
        }
        // 全ストレージから Component を削除
        for storage in self.components.values_mut() {
            storage.remove(entity);
        }
        true
    }

    /// Entity が生存しているか確認する。
    #[must_use]
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities.is_alive(entity)
    }

    /// 生存している Entity の数を返す。
    #[must_use]
    pub const fn entity_count(&self) -> usize {
        self.entities.alive_count()
    }

    // ── Component 操作 ──

    /// Entity に Component を追加する。
    pub fn insert_component<T: Component>(&mut self, entity: Entity, component: T) {
        let storage = self.get_or_create_storage::<T>();
        storage.insert(entity, component);
    }

    /// Entity の Component を取得する。
    #[must_use]
    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.get_storage::<T>().and_then(|storage| storage.get(entity))
    }

    /// Entity の Component を可変参照で取得する。
    #[must_use]
    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        self.get_storage_mut::<T>().and_then(|storage| storage.get_mut(entity))
    }

    /// Entity の Component を削除する。
    pub fn remove_component<T: Component>(&mut self, entity: Entity) -> Option<T> {
        self.get_storage_mut::<T>().and_then(|storage| storage.remove_component(entity))
    }

    /// 指定型の `ComponentStorage` を取得する。
    #[must_use]
    pub fn get_storage<T: Component>(&self) -> Option<&ComponentStorage<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.as_any().downcast_ref::<ComponentStorage<T>>())
    }

    /// 指定型の `ComponentStorage` を可変参照で取得する。
    #[must_use]
    pub fn get_storage_mut<T: Component>(&mut self) -> Option<&mut ComponentStorage<T>> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.as_any_mut().downcast_mut::<ComponentStorage<T>>())
    }

    /// 指定型の `ComponentStorage` を取得するか、存在しなければ作成する。
    fn get_or_create_storage<T: Component>(&mut self) -> &mut ComponentStorage<T> {
        self.components
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(ComponentStorage::<T>::new()));

        let storage = self
            .components
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.as_any_mut().downcast_mut::<ComponentStorage<T>>());

        storage.map_or_else(
            || panic!("storage type mismatch: this should never happen"),
            |storage| storage,
        )
    }

    // ── Resource 操作 ──

    /// リソースを登録する。
    pub fn insert_resource<T: Send + Sync + 'static>(&mut self, resource: T) {
        self.resources.insert(resource);
    }

    /// リソースへの参照を取得する。
    #[must_use]
    pub fn get_resource<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    /// リソースへの可変参照を取得する。
    #[must_use]
    pub fn get_resource_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /// リソースを削除して返す。
    pub fn remove_resource<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        self.resources.remove::<T>()
    }

    // ── Event 操作 ──

    /// 指定した型のイベントキューを登録する。
    pub fn register_event<T: Send + Sync + 'static>(&mut self) {
        self.events.register::<T>();
    }

    /// イベントを送信する。
    pub fn send_event<T: Send + Sync + 'static>(&mut self, event: T) {
        self.events.send(event);
    }

    /// イベントキューへの可変参照を取得する。
    #[must_use]
    pub fn get_event_queue_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut EventQueue<T>> {
        self.events.get_mut::<T>()
    }

    /// すべてのイベントキューをクリアする。
    pub fn clear_all_events(&mut self) {
        self.events.clear_all();
    }

    /// 指定した型のイベントライタを取得する。
    ///
    /// 返却された `EventWriter` は、キューにイベントを追加するために使用する。
    pub fn get_event_writer<T: Send + Sync + 'static>(&mut self) -> Option<EventWriter<'_, T>> {
        let queue = self.events.get_mut::<T>()?;
        Some(EventWriter::new(queue))
    }

    /// 指定した型のイベントリーダを取得する。
    ///
    /// 返却された `EventReader` は、キューに蓄積されたイベントを読み取るために使用する。
    pub fn get_event_reader<T: Send + Sync + 'static>(&mut self) -> Option<EventReader<'_, T>> {
        let queue = self.events.get_mut::<T>()?;
        Some(EventReader::new(queue))
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    struct Position {
        x: f32,
        y: f32,
    }
    impl Component for Position {}

    struct Velocity {
        dx: f32,
        dy: f32,
    }
    impl Component for Velocity {}

    struct GameTime(f64);
    struct PlayerDeath;

    #[test]
    fn test_spawn_and_despawn() {
        let mut world = World::new();
        let e0 = world.spawn();
        let e1 = world.spawn();

        assert_eq!(world.entity_count(), 2);
        assert!(world.is_alive(e0));

        world.despawn(e0);
        assert_eq!(world.entity_count(), 1);
        assert!(!world.is_alive(e0));
        assert!(world.is_alive(e1));
    }

    #[test]
    fn test_insert_and_get_component() {
        let mut world = World::new();
        let e = world.spawn();

        world.insert_component(e, Position { x: 10.0, y: 20.0 });
        world.insert_component(e, Velocity { dx: 1.0, dy: -1.0 });

        let pos = world.get_component::<Position>(e).unwrap();
        assert!((pos.x - 10.0).abs() < f32::EPSILON);
        assert!((pos.y - 20.0).abs() < f32::EPSILON);

        let vel = world.get_component::<Velocity>(e).unwrap();
        assert!((vel.dx - 1.0).abs() < f32::EPSILON);
        assert!((vel.dy - -1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_despawn_removes_all_components() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert_component(e, Position { x: 1.0, y: 2.0 });
        world.insert_component(e, Velocity { dx: 0.0, dy: 0.0 });

        world.despawn(e);
        // Component にアクセスしようとしても None
        assert!(world.get_component::<Position>(e).is_none());
        assert!(world.get_component::<Velocity>(e).is_none());
    }

    #[test]
    fn test_resources() {
        let mut world = World::new();
        world.insert_resource(GameTime(0.0));

        assert!((world.get_resource::<GameTime>().unwrap().0 - 0.0).abs() < f64::EPSILON);

        world.get_resource_mut::<GameTime>().unwrap().0 += 0.016;
        assert!((world.get_resource::<GameTime>().unwrap().0 - 0.016).abs() < f64::EPSILON);
    }

    #[test]
    fn test_component_storage_iterate() {
        let mut world = World::new();
        let e0 = world.spawn();
        let e1 = world.spawn();
        let e2 = world.spawn();

        world.insert_component(e0, Position { x: 0.0, y: 0.0 });
        world.insert_component(e1, Position { x: 1.0, y: 1.0 });
        world.insert_component(e2, Position { x: 2.0, y: 2.0 });

        let storage = world.get_storage::<Position>();
        assert!(storage.is_some());
        assert_eq!(storage.unwrap().len(), 3);
    }

    #[test]
    fn test_event_system() {
        let mut world = World::new();
        world.register_event::<PlayerDeath>();

        {
            let mut writer = world.get_event_writer::<PlayerDeath>().unwrap();
            writer.send(PlayerDeath);
        }

        {
            let mut reader = world.get_event_reader::<PlayerDeath>().unwrap();
            assert_eq!(reader.drain().count(), 1);
        }

        // クリア後にキューが空になるか
        {
            let mut writer = world.get_event_writer::<PlayerDeath>().unwrap();
            writer.send(PlayerDeath);
            writer.send(PlayerDeath);
        }

        world.clear_all_events();

        let reader_after_clear = world.get_event_reader::<PlayerDeath>().unwrap();
        assert!(reader_after_clear.is_empty());
    }
}
