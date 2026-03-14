//! ECS (Entity Component System) 基盤モジュール。
//!
//! すべてのゲームオブジェクトを Entity として管理し、
//! Component でデータ、System でロジックを分離する。

pub mod component;
pub mod entity;
pub mod event;
pub mod resource;
pub mod system;
pub mod world;

// 主要型の re-export
pub use component::{Component, ComponentStorage};
pub use entity::Entity;
pub use event::EventQueue;
pub use resource::Resources;
pub use system::{Schedule, System, into_system};
pub use world::World;
