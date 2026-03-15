//! 型付きイベントキュー。
//!
//! ECS ↔ UI 間のメッセージパッシングに使用する。
//! フレーム単位でイベントを蓄積し、消費する。

/// 型付きイベントキュー。
///
/// 1 フレーム内にイベントを蓄積し、消費側が `drain` でまとめて取り出す。
pub struct EventQueue<T> {
    events: Vec<T>,
}

impl<T> EventQueue<T> {
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

    /// キューをクリアする。
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl<'a, T> IntoIterator for &'a EventQueue<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Default for EventQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
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

        let items: Vec<_> = queue.iter().collect();
        assert_eq!(items.len(), 2);
        assert_eq!(queue.len(), 2); // まだ消費されていない
    }

    #[test]
    fn test_clear() {
        let mut queue = EventQueue::new();
        queue.send(42);
        queue.clear();
        assert!(queue.is_empty());
    }
}
