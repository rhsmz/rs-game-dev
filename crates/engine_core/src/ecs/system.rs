//! System トレイトとスケジューラ。
//!
//! System はロジックのみを記述し、状態を保持しない。
//! `Schedule` が System の実行順序を管理する。

use super::world::World;

/// ECS System トレイト。
///
/// ゲームロジックの最小単位。`World` を受け取り、
/// Component の読み書きを行う。
pub trait System: Send + Sync {
    /// System を 1 フレーム分実行する。
    fn run(&mut self, world: &mut World);
}

/// 関数ポインタから System を作成するためのラッパー。
pub struct FnSystem<F: FnMut(&mut World) + Send + Sync> {
    func: F,
}

impl<F: FnMut(&mut World) + Send + Sync> System for FnSystem<F> {
    fn run(&mut self, world: &mut World) {
        (self.func)(world);
    }
}

/// 関数を System に変換する。
pub fn into_system<F: FnMut(&mut World) + Send + Sync + 'static>(f: F) -> impl System {
    FnSystem { func: f }
}

/// System の実行スケジュール。
///
/// 登録された System を順次実行する。
pub struct Schedule {
    systems: Vec<Box<dyn System>>,
}

impl Schedule {
    /// 新しい空のスケジュールを作成する。
    #[must_use]
    pub fn new() -> Self {
        Self { systems: Vec::new() }
    }

    /// System を追加する。追加順に実行される。
    pub fn add_system(&mut self, system: impl System + 'static) {
        self.systems.push(Box::new(system));
    }

    /// 登録されたすべての System を順次実行する。
    pub fn run(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.run(world);
        }
    }

    /// 登録されている System の数を返す。
    #[must_use]
    pub fn len(&self) -> usize {
        self.systems.len()
    }

    /// スケジュールが空かどうかを返す。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.systems.is_empty()
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestResource {
        counter: i32,
    }

    #[test]
    fn test_into_system() {
        let mut world = World::new();
        world.insert_resource(TestResource { counter: 0 });

        let mut sys = into_system(|w: &mut World| {
            let res = w
                .get_resource_mut::<TestResource>()
                .expect("TestResource should exist");
            res.counter += 1;
        });

        sys.run(&mut world);
        sys.run(&mut world);

        let res = world
            .get_resource::<TestResource>()
            .expect("TestResource should exist");
        assert_eq!(res.counter, 2);
    }

    #[test]
    fn test_into_system_with_captured_state() {
        let mut world = World::new();
        world.insert_resource(TestResource { counter: 0 });

        let mut local_counter = 10;
        let mut sys = into_system(move |w: &mut World| {
            local_counter += 5;
            let res = w
                .get_resource_mut::<TestResource>()
                .expect("TestResource should exist");
            res.counter = local_counter;
        });

        sys.run(&mut world);
        assert_eq!(
            world
                .get_resource::<TestResource>()
                .expect("TestResource should exist")
                .counter,
            15
        );

        sys.run(&mut world);
        assert_eq!(
            world
                .get_resource::<TestResource>()
                .expect("TestResource should exist")
                .counter,
            20
        );
    }
}
