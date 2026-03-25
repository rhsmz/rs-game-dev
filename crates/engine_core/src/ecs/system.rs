//! System トレイトとスケジューラ。
//!
//! System はロジックのみを記述し、状態を保持しない。
//! `Schedule` が System の実行順序を管理する。

use super::world::World;
use std::collections::HashMap;

/// System を実行する段階（フェーズ）を定義する。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemStage {
    PreUpdate,
    Update,
    PostUpdate,
}

impl SystemStage {
    /// デフォルトの実行順序を定義する。
    #[must_use]
    pub const fn execution_order() -> &'static [Self] {
        &[Self::PreUpdate, Self::Update, Self::PostUpdate]
    }
}

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
/// 登録された System をステージごとに管理し、定義された順序で実行する。
pub struct Schedule {
    systems: HashMap<SystemStage, Vec<Box<dyn System>>>,
    stage_order: Vec<SystemStage>,
}

impl Schedule {
    /// 新しい空のスケジュールを作成する。
    #[must_use]
    pub fn new() -> Self {
        Self { systems: HashMap::new(), stage_order: SystemStage::execution_order().to_vec() }
    }

    /// System を指定したステージに追加する。
    pub fn add_system(&mut self, stage: SystemStage, system: impl System + 'static) {
        self.systems.entry(stage).or_default().push(Box::new(system));
    }

    /// 登録されたすべての System をステージ順に実行する。
    pub fn run(&mut self, world: &mut World) {
        // stage_order を clone して、`run_stage` での可変参照の借用と競合しないようにする
        for stage in self.stage_order.clone() {
            self.run_stage(stage, world);
        }
    }

    /// 指定されたステージの System をすべて実行する。
    pub fn run_stage(&mut self, stage: SystemStage, world: &mut World) {
        if let Some(systems) = self.systems.get_mut(&stage) {
            for system in systems {
                system.run(world);
            }
        }
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
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_add_and_run_single_system() {
        let mut schedule = Schedule::new();
        let mut world = World::new();
        world.insert_resource(0_i32);

        schedule.add_system(
            SystemStage::Update,
            into_system(|world| {
                *world.get_resource_mut::<i32>().unwrap() += 1;
            }),
        );

        schedule.run(&mut world);
        assert_eq!(*world.get_resource::<i32>().unwrap(), 1);
    }

    #[test]
    fn test_stage_execution_order() {
        let mut schedule = Schedule::new();
        let mut world = World::new();
        // System が Send + Sync を要求するため、スレッドセーフな Arc<Mutex<>> を使用する
        let execution_log = Arc::new(Mutex::new(Vec::new()));

        let log_clone_1 = execution_log.clone();
        schedule.add_system(
            SystemStage::Update,
            into_system(move |_| {
                log_clone_1.lock().unwrap().push(SystemStage::Update);
            }),
        );

        let log_clone_2 = execution_log.clone();
        schedule.add_system(
            SystemStage::PostUpdate,
            into_system(move |_| {
                log_clone_2.lock().unwrap().push(SystemStage::PostUpdate);
            }),
        );

        let log_clone_3 = execution_log.clone();
        schedule.add_system(
            SystemStage::PreUpdate,
            into_system(move |_| {
                log_clone_3.lock().unwrap().push(SystemStage::PreUpdate);
            }),
        );

        schedule.run(&mut world);

        let expected_order =
            vec![SystemStage::PreUpdate, SystemStage::Update, SystemStage::PostUpdate];
        assert_eq!(*execution_log.lock().unwrap(), expected_order);
    }

    #[test]
    fn test_run_specific_stage() {
        let mut schedule = Schedule::new();
        let mut world = World::new();
        world.insert_resource(String::new());

        schedule.add_system(
            SystemStage::Update,
            into_system(|w| {
                w.get_resource_mut::<String>().unwrap().push('U');
            }),
        );
        schedule.add_system(
            SystemStage::PreUpdate,
            into_system(|w| {
                w.get_resource_mut::<String>().unwrap().push('P');
            }),
        );

        schedule.run_stage(SystemStage::Update, &mut world);

        assert_eq!(*world.get_resource::<String>().unwrap(), "U");
    }
}
