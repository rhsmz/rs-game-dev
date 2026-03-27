//! 入力抽象化モジュール。
//!
//! キーボード、マウス、ゲームパッドの統一入力管理と
//! アクションマッピングを提供する。

mod action;
mod event;
mod state;
mod winit_bridge;

pub use action::{ActionMap, PhysicalInput};
pub use event::{ActionName, GamepadButton, GamepadId, InputEvent, KeyCode, MouseButton};
pub use state::{GamepadState, InputState};
pub use winit_bridge::convert_winit_event;

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use super::*;
    use crate::ecs::World;

    #[test]
    fn test_input_event_can_be_sent_to_ecs_event_queue() {
        let mut world = World::new();
        world.register_event::<InputEvent>();

        {
            let mut writer = world
                .get_event_writer::<InputEvent>()
                .expect("InputEvent queue should be registered");
            writer.send(InputEvent::ActionTriggered { name: "confirm" });
        }

        let mut reader =
            world.get_event_reader::<InputEvent>().expect("InputEvent queue should be registered");
        let events: Vec<_> = reader.drain().collect();
        assert_eq!(events, vec![InputEvent::ActionTriggered { name: "confirm" }]);
    }
}
