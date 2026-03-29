// FILAMENT_LIB_DIR 設定時: 公式 Filament C++ API へ委譲する C シム。
// ヘッダはプリビルト / 自前インストールの include を FILAMENT_INCLUDE_DIR または
// FILAMENT_LIB_DIR からの相対パスで解決する（build.rs 参照）。

#include <filament/Engine.h>
#include <filament/Renderer.h>
#include <filament/Scene.h>
#include <filament/SwapChain.h>
#include <filament/View.h>

using namespace filament;

namespace {

constexpr uint32_t kHeadlessSwapWidth = 4;
constexpr uint32_t kHeadlessSwapHeight = 4;

} // namespace

extern "C" {

Engine *Engine_create(void) {
    return Engine::create();
}

void Engine_destroy(Engine *engine) {
    if (engine != nullptr) {
        Engine::destroy(engine);
    }
}

Scene *Scene_create(Engine *engine) {
    if (engine == nullptr) {
        return nullptr;
    }
    return engine->createScene();
}

void Scene_destroy(Engine *engine, Scene *scene) {
    if (engine != nullptr && scene != nullptr) {
        (void)engine->destroy(scene);
    }
}

View *View_create(Engine *engine) {
    if (engine == nullptr) {
        return nullptr;
    }
    return engine->createView();
}

void View_destroy(Engine *engine, View *view) {
    if (engine != nullptr && view != nullptr) {
        (void)engine->destroy(view);
    }
}

void View_setScene(View *view, Scene *scene) {
    if (view != nullptr) {
        view->setScene(scene);
    }
}

Renderer *Renderer_create(Engine *engine) {
    if (engine == nullptr) {
        return nullptr;
    }
    return engine->createRenderer();
}

void Renderer_destroy(Engine *engine, Renderer *renderer) {
    if (engine != nullptr && renderer != nullptr) {
        (void)engine->destroy(renderer);
    }
}

SwapChain *SwapChain_create(Engine *engine) {
    if (engine == nullptr) {
        return nullptr;
    }
    // スモークテスト用ヘッドレス。実ウィンドウは engine->createSwapChain(nativeWindow) に差し替え予定。
    return engine->createSwapChain(kHeadlessSwapWidth, kHeadlessSwapHeight);
}

void SwapChain_destroy(Engine *engine, SwapChain *swap_chain) {
    if (engine != nullptr && swap_chain != nullptr) {
        (void)engine->destroy(swap_chain);
    }
}

bool Renderer_beginFrame(SwapChain *swap_chain, Renderer *renderer) {
    if (swap_chain == nullptr || renderer == nullptr) {
        return false;
    }
    return renderer->beginFrame(swap_chain);
}

void Renderer_endFrame(Renderer *renderer) {
    if (renderer != nullptr) {
        renderer->endFrame();
    }
}

void SwapChain_resize(SwapChain *swap_chain, unsigned int width, unsigned int height) {
    (void)swap_chain;
    (void)width;
    (void)height;
}

void Renderer_render(Renderer *renderer, View *view) {
    if (renderer != nullptr && view != nullptr) {
        renderer->render(view);
    }
}

} // extern "C"
