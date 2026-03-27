//! 型付きイベントキュー。
//!
//! ECS ↔ UI 間のメッセージパッシングに使用する。
//! フレーム単位でイベントを蓄積し、消費する。

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// 型消去されたイベントキューが実装すべきトレイト。
#[allow(dead_code)]
trait AnyEventQueue: Send + Sync {
    /// キューをクリアする。
    fn clear(&mut self);
    /// `Any` への参照を返す。
    fn as_any(&self) -> &dyn Any;
    /// `Any` への可変参照を返す。
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 型付きイベントキュー。
///
/// 1 フレーム内にイベントを蓄積し、消費側が `drain` でまとめて取り出す。
pub struct EventQueue<T: Send + Sync + 'static> {
    events: Vec<T>,
}

/// `World::get_event_writer<T>()` が返すハンドル。
pub struct EventWriter<'a, T: Send + Sync + 'static> {
    queue: &'a mut EventQueue<T>,
}

impl<T: Send + Sync + 'static> EventWriter<'_, T> {
    #[allow(clippy::elidable_lifetime_names)]
    pub(crate) const fn new<'a>(queue: &'a mut EventQueue<T>) -> EventWriter<'a, T> {
        EventWriter { queue }
    }

    /// イベントを送信（キューに追加）。
    pub fn send(&mut self, event: T) {
        self.queue.send(event);
    }

    /// 現在キューに溜まっているイベント数。
    #[must_use]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// キューが空かどうか。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/// `World::get_event_reader<T>()` が返すハンドル。
pub struct EventReader<'a, T: Send + Sync + 'static> {
    queue: &'a mut EventQueue<T>,
}

impl<T: Send + Sync + 'static> EventReader<'_, T> {
    #[allow(clippy::elidable_lifetime_names)]
    pub(crate) const fn new<'a>(queue: &'a mut EventQueue<T>) -> EventReader<'a, T> {
        EventReader { queue }
    }

    /// 蓄積されたイベントをすべて取り出す。キューは空になる。
    pub fn drain(&mut self) -> std::vec::Drain<'_, T> {
        self.queue.drain()
    }

    /// 蓄積されたイベントを参照する（消費しない）。
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.queue.iter()
    }

    /// 現在キューに溜まっているイベント数。
    #[must_use]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// キューが空かどうか。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl<'a, T: Send + Sync + 'static> IntoIterator for &'a EventReader<'a, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Send + Sync + 'static> EventQueue<T> {
    /// 新しい空のイベントキューを作成する。
    #[must_use]
    pub const fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// イベントを送信 (キューに追加) する。
    pub fn send(&mut self, event: T) {
        self.events.push(event);
    }

    /// 蓄積されたイベントをすべて取り出す。キューは空になる。
    pub fn drain(&mut self) -> std::vec::Drain<'_, T> {
        self.events.drain(..)
    }

    /// 蓄積されたイベントを参照する (消費しない)。
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.events.iter()
    }

    /// キュー内のイベント数を返す。
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// キューが空かどうかを返す。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl<T: Send + Sync + 'static> AnyEventQueue for EventQueue<T> {
    fn clear(&mut self) {
        self.events.clear();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<'a, T: Send + Sync + 'static> IntoIterator for &'a EventQueue<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Send + Sync + 'static> Default for EventQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 複数の `EventQueue` を管理するコンテナ。
pub struct EventQueues {
    queues: HashMap<TypeId, Box<dyn AnyEventQueue>>,
}

impl EventQueues {
    /// 新しい `EventQueues` を作成する。
    #[must_use]
    pub fn new() -> Self {
        Self { queues: HashMap::new() }
    }

    /// 指定した型のイベントキューを登録する。
    pub fn register<T: Send + Sync + 'static>(&mut self) {
        self.queues.entry(TypeId::of::<T>()).or_insert_with(|| Box::new(EventQueue::<T>::new()));
    }

    /// イベントを送信する。
    ///
    /// 対応するキューが存在しない場合は何も起こらない。
    pub fn send<T: Send + Sync + 'static>(&mut self, event: T) {
        if let Some(queue) = self.get_mut::<T>() {
            queue.send(event);
        }
    }

    /// 指定した型のイベントキューへの可変参照を取得する。
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut EventQueue<T>> {
        self.queues
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.as_any_mut().downcast_mut::<EventQueue<T>>())
    }

    /// すべてのイベントキューをクリアする。
    pub fn clear_all(&mut self) {
        for queue in self.queues.values_mut() {
            queue.clear();
        }
    }
}

impl Default for EventQueues {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn test_send_and_drain() {
        let mut queue = EventQueue::new();
        queue.send(1);
        queue.send(2);
        queue.send(3);

        let events: Vec<_> = queue.drain().collect();
        assert_eq!(events, vec![1, 2, 3]);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_iter_does_not_consume() {
        let mut queue = EventQueue::new();
        queue.send("hello");
        queue.send("world");

        assert_eq!(queue.iter().count(), 2);
        assert_eq!(queue.len(), 2); // まだ消費されていない
    }

    #[test]
    fn test_clear() {
        let mut queue = EventQueue::new();
        queue.send(42);
        queue.clear();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_event_queues_send_and_get() {
        let mut queues = EventQueues::new();
        queues.register::<i32>();
        queues.register::<&'static str>();

        queues.send(100);
        queues.send("event");

        let q1 = queues.get_mut::<i32>().unwrap();
        assert_eq!(q1.drain().collect::<Vec<_>>(), vec![100]);

        let q2 = queues.get_mut::<&'static str>().unwrap();
        assert_eq!(q2.drain().collect::<Vec<_>>(), vec!["event"]);
    }

    #[test]
    fn test_event_queues_clear_all() {
        let mut queues = EventQueues::new();
        queues.register::<i32>();
        queues.register::<String>();

        queues.send(123);
        queues.send("hello".to_string());

        queues.clear_all();

        let q1 = queues.get_mut::<i32>().unwrap();
        assert!(q1.is_empty());
        let q2 = queues.get_mut::<String>().unwrap();
        assert!(q2.is_empty());
    }
}
