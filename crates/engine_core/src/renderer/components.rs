//! Renderer 用 ECS コンポーネント（雛形）。

use crate::ecs::component::Component;

/// メッシュ描画情報（雛形）。
#[derive(Debug, Clone, Copy)]
pub struct MeshRenderer {
    /// 描画対象の（暫定）ハンドル ID。
    pub renderable_id: u32,
}

impl Component for MeshRenderer {}
