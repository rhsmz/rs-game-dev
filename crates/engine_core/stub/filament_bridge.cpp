// FILAMENT_LIB_DIR 設定時: 公式 Filament C++ API へ委譲する C シム。
// ヘッダはプリビルト / 自前インストールの include を FILAMENT_INCLUDE_DIR または
// FILAMENT_LIB_DIR からの相対パスで解決する（build.rs 参照）。

#include <new>

#include <filament/Camera.h>
#include <filament/Engine.h>
#include <filament/Renderer.h>
#include <filament/Scene.h>
#include <filament/SwapChain.h>
#include <filament/View.h>
#include <filament/Viewport.h>
#include <utils/EntityManager.h>

using namespace filament;

namespace {

constexpr uint32_t kHeadlessSwapWidth = 4;
constexpr uint32_t kHeadlessSwapHeight = 4;

struct FilamentViewCameraBinding {
    utils::Entity entity{};
};

constexpr double kPi = 3.14159265358979323846;

double radiansToDegrees(double rad) {
    return rad * 180.0 / kPi;
}

/// UI View をスワップチェーン上に合成する際の View レベル設定。
/// 深度テスト／深度書き込みは Filament では主に `MaterialInstance` 側のため、ここでは合成・負荷の抑止のみ行う。
void apply_ui_view_surface_state(View *view) {
    if (view == nullptr) {
        return;
    }
    view->setBlendMode(BlendMode::TRANSLUCENT);
    view->setPostProcessingEnabled(false);
    view->setShadowingEnabled(false);
    view->setScreenSpaceRefractionEnabled(false);
}

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

void *ViewCamera_create_game(Engine *engine, View *view, unsigned int width, unsigned int height,
                             float fov_y_radians, float near_plane, float far_plane) {
    if (engine == nullptr || view == nullptr || width == 0 || height == 0) {
        return nullptr;
    }
    auto *binding = new (std::nothrow) FilamentViewCameraBinding();
    if (binding == nullptr) {
        return nullptr;
    }
    binding->entity = utils::EntityManager::get().create();
    if (binding->entity.isNull()) {
        delete binding;
        return nullptr;
    }
    Camera *cam = engine->createCamera(binding->entity);
    if (cam == nullptr) {
        utils::EntityManager::get().destroy(binding->entity);
        delete binding;
        return nullptr;
    }
    double const aspect = static_cast<double>(width) / static_cast<double>(height);
    double const fovDeg = radiansToDegrees(static_cast<double>(fov_y_radians));
    cam->setProjection(fovDeg, aspect, near_plane, far_plane, Camera::Fov::VERTICAL);
    view->setCamera(cam);
    view->setViewport(Viewport(0, 0, width, height));
    return binding;
}

void *ViewCamera_create_ui(Engine *engine, View *view, unsigned int width, unsigned int height,
                           float near_plane, float far_plane) {
    if (engine == nullptr || view == nullptr || width == 0 || height == 0) {
        return nullptr;
    }
    auto *binding = new (std::nothrow) FilamentViewCameraBinding();
    if (binding == nullptr) {
        return nullptr;
    }
    binding->entity = utils::EntityManager::get().create();
    if (binding->entity.isNull()) {
        delete binding;
        return nullptr;
    }
    Camera *cam = engine->createCamera(binding->entity);
    if (cam == nullptr) {
        utils::EntityManager::get().destroy(binding->entity);
        delete binding;
        return nullptr;
    }
    cam->setProjection(Camera::Projection::ORTHO, 0.0, static_cast<double>(width), 0.0,
                       static_cast<double>(height), near_plane, far_plane);
    view->setCamera(cam);
    view->setViewport(Viewport(0, 0, width, height));
    apply_ui_view_surface_state(view);
    return binding;
}

void ViewCamera_update_game(Engine *engine, View *view, void *binding_void, unsigned int width,
                            unsigned int height, float fov_y_radians, float near_plane,
                            float far_plane) {
    if (engine == nullptr || view == nullptr || binding_void == nullptr || width == 0 ||
        height == 0) {
        return;
    }
    auto *binding = static_cast<FilamentViewCameraBinding *>(binding_void);
    Camera *cam = engine->getCameraComponent(binding->entity);
    if (cam == nullptr) {
        return;
    }
    double const aspect = static_cast<double>(width) / static_cast<double>(height);
    double const fovDeg = radiansToDegrees(static_cast<double>(fov_y_radians));
    cam->setProjection(fovDeg, aspect, near_plane, far_plane, Camera::Fov::VERTICAL);
    view->setViewport(Viewport(0, 0, width, height));
}

void ViewCamera_update_ui(Engine *engine, View *view, void *binding_void, unsigned int width,
                          unsigned int height, float near_plane, float far_plane) {
    if (engine == nullptr || view == nullptr || binding_void == nullptr || width == 0 ||
        height == 0) {
        return;
    }
    auto *binding = static_cast<FilamentViewCameraBinding *>(binding_void);
    Camera *cam = engine->getCameraComponent(binding->entity);
    if (cam == nullptr) {
        return;
    }
    cam->setProjection(Camera::Projection::ORTHO, 0.0, static_cast<double>(width), 0.0,
                       static_cast<double>(height), near_plane, far_plane);
    view->setViewport(Viewport(0, 0, width, height));
    apply_ui_view_surface_state(view);
}

void ViewCamera_destroy(Engine *engine, View *view, void *binding_void) {
    if (binding_void == nullptr) {
        return;
    }
    auto *binding = static_cast<FilamentViewCameraBinding *>(binding_void);
    if (view != nullptr) {
        view->setCamera(nullptr);
    }
    if (engine != nullptr && !binding->entity.isNull()) {
        engine->destroyCameraComponent(binding->entity);
        utils::EntityManager::get().destroy(binding->entity);
    }
    delete binding;
}

} // extern "C"
