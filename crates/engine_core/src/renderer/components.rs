//! Renderer 用 ECS コンポーネント（雛形）。

use crate::ecs::component::Component;

/// メッシュ描画情報（雛形）。
///
/// `renderable_id == 0` は **未ロード / プレースホルダ** とみなし、[`crate::renderer::render_system`] では
/// warn ログを出して Filament へは投入しない。非ゼロ ID はブリッジ側の対応表（将来拡張）へ渡る。
#[derive(Debug, Clone, Copy)]
pub struct MeshRenderer {
    /// 描画対象の（暫定）ハンドル ID。
    pub renderable_id: u32,
}

impl Component for MeshRenderer {}
