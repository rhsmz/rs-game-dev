//! ECS で共通的に利用される基本 Component。

use super::component::Component;

/// 3D 空間における位置・回転・スケールを表す。
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4], // Quaternion (x, y, z, w)
    pub scale: [f32; 3],
}

impl Component for Transform {}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0], // Identity quaternion
            scale: [1.0, 1.0, 1.0],
        }
    }
}

/// Entity の名前。デバッグやエディタでの識別に用いる。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(pub String);

impl Component for Name {}

/// Entity がアクティブかどうかを示す。
///
///非アクティブな Entity は通常、描画や更新処理からスキップされる。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Active(pub bool);

impl Component for Active {}

impl Default for Active {
    fn default() -> Self {
        Self(true)
    }
}
