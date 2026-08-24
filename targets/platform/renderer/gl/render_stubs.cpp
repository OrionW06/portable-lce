#include "GLRenderer.h"

extern "C" {
int rust_gl_renderer_cbuff_size();
int rust_gl_renderer_save_texture_data();
bool rust_gl_renderer_suspended();
}

// Command Buffers
void GLRenderer::CBuffLockStaticCreations() {}
int GLRenderer::CBuffSize(int) { return rust_gl_renderer_cbuff_size(); }
void GLRenderer::CBuffTick() {}
void GLRenderer::CBuffDeferredModeStart() {}
void GLRenderer::CBuffDeferredModeEnd() {}

// Render States
void GLRenderer::StateSetLightEnable(int, bool) {}
void GLRenderer::StateSetEnableViewportClipPlanes(bool) {}
void GLRenderer::StateSetForceLOD(int) {}
void GLRenderer::StateSetTexGenCol(int, float, float, float, float, bool) {}

// Textures
void GLRenderer::TextureDynamicUpdateStart() {}
void GLRenderer::TextureDynamicUpdateEnd() {}
void GLRenderer::TextureGetStats() {}
void* GLRenderer::TextureGetTexture(int) { return nullptr; }

int GLRenderer::SaveTextureData(const char*, D3DXIMAGE_INFO*, int*) {
    return rust_gl_renderer_save_texture_data();
}
int GLRenderer::SaveTextureDataToMemory(void*, int, int*, int, int, int*) {
    return rust_gl_renderer_save_texture_data();
}

// Screen/Image Capturing
void GLRenderer::DoScreenGrabOnNextPresent() {}
void GLRenderer::CaptureThumbnail(ImageFileBuffer*) {}
void GLRenderer::CaptureScreen(ImageFileBuffer*, XSOCIAL_PREVIEWIMAGE*) {}

// Conditional Rendering & Events
void GLRenderer::BeginConditionalSurvey(int) {}
void GLRenderer::EndConditionalSurvey() {}
void GLRenderer::BeginConditionalRendering(int) {}
void GLRenderer::EndConditionalRendering() {}
void GLRenderer::BeginEvent(const char*) {}
void GLRenderer::EndEvent() {}
void GLRenderer::Tick() {}

// Lifecycle
void GLRenderer::Suspend() {}
bool GLRenderer::Suspended() { return rust_gl_renderer_suspended(); }
void GLRenderer::Resume() {}
