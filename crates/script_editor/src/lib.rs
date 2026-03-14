//! # rs-game-dev Script Editor
//!
//! スクリプト編集、ライブプレビュー、ビジュアル編集ツール。
//!
//! - **UI**: MVVM + Signal ベースのエディタ UI
//! - **Preview**: ライブプレビュー (ホットリロード)
//! - **Inspector**: `Live2D` / 3D パラメータ調整
//! - **Project**: プロジェクト管理、アセットブラウザ

pub mod inspector;
pub mod preview;
pub mod project;
pub mod ui;
