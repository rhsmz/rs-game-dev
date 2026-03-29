// FILAMENT_LIB_DIR 設定時: 公式 Filament C++ API へ委譲する C シム。
// ヘッダはプリビルト / 自前インストールの include を FILAMENT_INCLUDE_DIR または
// FILAMENT_LIB_DIR からの相対パスで解決する（build.rs 参照）。

#include <cstdint>
#include <mutex>
#include <new>
#include <unordered_map>

#include <filament/Box.h>
#include <filament/Camera.h>
#include <filament/Engine.h>
#include <filament/IndexBuffer.h>
#include <filament/Material.h>
#include <filament/MaterialInstance.h>
#include <filament/RenderableManager.h>
#include <filament/Renderer.h>
#include <filament/Scene.h>
#include <filament/SwapChain.h>
#include <filament/VertexBuffer.h>
#include <filament/View.h>
#include <filament/Viewport.h>

#include <math/vec3.h>

#include <utils/EntityManager.h>

using namespace filament;
using filament::math::float3;

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

/// P0-1: `renderable_id` ごとに 1 エンティティ（三角形 + Engine 既定マテリアル）を Scene に載せる。
/// `Scene_submit_mesh_vertical_slice` はフレーム毎に呼ばれるため、既登録 ID は no-op。
struct VerticalSliceMeshRecord {
    utils::Entity entity{};
    VertexBuffer *vb = nullptr;
    IndexBuffer *ib = nullptr;
};

using VerticalSliceMeshTable = std::unordered_map<uint32_t, VerticalSliceMeshRecord>;

std::mutex g_vertical_slice_mutex;
std::unordered_map<Scene *, VerticalSliceMeshTable> g_vertical_slice_registry;

/// カメラ前方（-Z）に置いた CCW 三角形。`Engine::getDefaultMaterial()`（内部 unlit グレー）で描画する。
static const float3 kVerticalSliceTriangle[3] = {
    {0.0f, 0.55f, -3.5f},
    {-0.55f, -0.45f, -3.5f},
    {0.55f, -0.45f, -3.5f},
};

static constexpr uint16_t kVerticalSliceIndices[3] = {0, 1, 2};

void vertical_slice_destroy_scene_entries(Engine *engine, Scene *scene) {
    if (engine == nullptr || scene == nullptr) {
        return;
    }
    std::lock_guard<std::mutex> lock(g_vertical_slice_mutex);
    auto it = g_vertical_slice_registry.find(scene);
    if (it == g_vertical_slice_registry.end()) {
        return;
    }
    for (auto &kv : it->second) {
        VerticalSliceMeshRecord &rec = kv.second;
        if (!rec.entity.isNull()) {
            engine->destroy(rec.entity);
        }
        if (rec.vb != nullptr) {
            engine->destroy(rec.vb);
        }
        if (rec.ib != nullptr) {
            engine->destroy(rec.ib);
        }
    }
    g_vertical_slice_registry.erase(it);
}

bool vertical_slice_create_mesh_for_id(Engine *engine, Scene *scene, uint32_t renderable_id) {
    std::lock_guard<std::mutex> lock(g_vertical_slice_mutex);
    VerticalSliceMeshTable &table = g_vertical_slice_registry[scene];
    if (table.find(renderable_id) != table.end()) {
        return true;
    }

    VertexBuffer *vb = VertexBuffer::Builder()
                               .vertexCount(3)
                               .bufferCount(1)
                               .attribute(VertexAttribute::POSITION, 0, VertexBuffer::AttributeType::FLOAT3, 0,
                                       12)
                               .build(*engine);
    if (vb == nullptr) {
        return false;
    }
    vb->setBufferAt(*engine, 0,
            VertexBuffer::BufferDescriptor(kVerticalSliceTriangle, sizeof(kVerticalSliceTriangle), nullptr));

    IndexBuffer *ib = IndexBuffer::Builder()
                              .indexCount(3)
                              .bufferType(IndexBuffer::IndexType::USHORT)
                              .build(*engine);
    if (ib == nullptr) {
        engine->destroy(vb);
        return false;
    }
    ib->setBuffer(*engine, IndexBuffer::BufferDescriptor(kVerticalSliceIndices, sizeof(kVerticalSliceIndices), nullptr));

    Material const *defMat = engine->getDefaultMaterial();
    if (defMat == nullptr) {
        engine->destroy(ib);
        engine->destroy(vb);
        return false;
    }
    MaterialInstance const *mi = defMat->getDefaultInstance();

    utils::Entity renderable = utils::EntityManager::get().create();
    if (renderable.isNull()) {
        engine->destroy(ib);
        engine->destroy(vb);
        return false;
    }

    Box bbox{};
    bbox.set(float3{-0.6f, -0.6f, -4.0f}, float3{0.6f, 0.6f, -3.0f});

    RenderableManager::Builder builder(1);
    builder.boundingBox(bbox)
            .material(0, mi)
            .geometry(0, RenderableManager::PrimitiveType::TRIANGLES, vb, ib, 0, 3)
            .culling(false)
            .receiveShadows(false)
            .castShadows(false);
    if (builder.build(*engine, renderable) != RenderableManager::Builder::Success) {
        engine->destroy(renderable);
        engine->destroy(ib);
        engine->destroy(vb);
        return false;
    }

    scene->addEntity(renderable);

    VerticalSliceMeshRecord rec;
    rec.entity = renderable;
    rec.vb = vb;
    rec.ib = ib;
    table.emplace(renderable_id, rec);
    return true;
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
    vertical_slice_destroy_scene_entries(engine, scene);
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

bool Scene_submit_mesh_vertical_slice(Engine *engine, Scene *scene, uint32_t renderable_id) {
    if (engine == nullptr || scene == nullptr || renderable_id == 0u) {
        return false;
    }
    return vertical_slice_create_mesh_for_id(engine, scene, renderable_id);
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
