//! ECS (Entity Component System) 基盤モジュール。
//!
//! すべてのゲームオブジェクトを Entity として管理し、
//! Component でデータ、System でロジックを分離する。

pub mod component;
pub mod components;
pub mod entity;
pub mod event;
pub mod query;
pub mod resource;
pub mod system;
pub mod world;

// 主要型の re-export
pub use component::{Component, ComponentStorage};
pub use components::{Active, Name, Transform};
pub use entity::Entity;
pub use event::{EventQueue, EventQueues, EventReader, EventWriter};
pub use query::{QueryParam, WorldQueryExt};
pub use resource::Resources;
pub use system::{Schedule, System, SystemStage, into_system};
pub use world::World;
