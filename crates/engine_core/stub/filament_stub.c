/* Minimal Filament C API stub for local builds when FILAMENT_LIB_DIR is unset.
 * Symbols must match `crates/engine_core/src/ffi/filament_sys.rs` declarations.
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

void Scene_destroy(Scene *scene) {
    free(scene);
}

View *View_create(Engine *engine) {
    (void)engine;
    return (View *)calloc(1u, sizeof(void *));
}

void View_destroy(View *view) {
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

void Renderer_destroy(Renderer *renderer) {
    free(renderer);
}

SwapChain *SwapChain_create(Engine *engine) {
    (void)engine;
    return (SwapChain *)calloc(1u, sizeof(void *));
}

void SwapChain_destroy(SwapChain *swap_chain) {
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
