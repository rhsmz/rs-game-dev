/* FILAMENT_LIB_DIR 未設定時のみビルドに含まれる開発用スタブ（公式 API の C ラッパではない）。
 *
 * 方針:
 * - プリビルト／自前ビルドをリンクするときは filament_bridge.cpp が同じ extern "C" 名で実装する。
 * - スタブは公式ヘッダに追従する必要はなく、filament_sys.rs とブリッジとでシグネチャを三点一致させる。
 * - GPU や Filament ランタイムに依存しないため、`cargo test`（デバッグ）を常に通しやすくする。
 */
#include <stdbool.h>
#include <stdlib.h>

typedef struct Engine Engine;
typedef struct Scene Scene;
typedef struct View View;
typedef struct Renderer Renderer;
typedef struct SwapChain SwapChain;

Engine *Engine_create(void) {
    return (Engine *)calloc(1u, sizeof(void *));
}

void Engine_destroy(Engine *engine) {
    free(engine);
}

Scene *Scene_create(Engine *engine) {
    (void)engine;
    return (Scene *)calloc(1u, sizeof(void *));
}

void Scene_destroy(Engine *engine, Scene *scene) {
    (void)engine;
    free(scene);
}

View *View_create(Engine *engine) {
    (void)engine;
    return (View *)calloc(1u, sizeof(void *));
}

void View_destroy(Engine *engine, View *view) {
    (void)engine;
    free(view);
}

void View_setScene(View *view, Scene *scene) {
    (void)view;
    (void)scene;
}

Renderer *Renderer_create(Engine *engine) {
    (void)engine;
    return (Renderer *)calloc(1u, sizeof(void *));
}

void Renderer_destroy(Engine *engine, Renderer *renderer) {
    (void)engine;
    free(renderer);
}

SwapChain *SwapChain_create(Engine *engine) {
    (void)engine;
    return (SwapChain *)calloc(1u, sizeof(void *));
}

void SwapChain_destroy(Engine *engine, SwapChain *swap_chain) {
    (void)engine;
    free(swap_chain);
}

bool Renderer_beginFrame(SwapChain *swap_chain, Renderer *renderer) {
    (void)swap_chain;
    (void)renderer;
    return true;
}

void Renderer_endFrame(Renderer *renderer) {
    (void)renderer;
}

void SwapChain_resize(SwapChain *swap_chain, unsigned int width, unsigned int height) {
    (void)swap_chain;
    (void)width;
    (void)height;
}

void Renderer_render(Renderer *renderer, View *view) {
    (void)renderer;
    (void)view;
}
