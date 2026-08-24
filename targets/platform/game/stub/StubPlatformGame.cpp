#include "StubPlatformGame.h"

#include "platform/game/game.h"

extern "C" {
int rust_stub_platform_game_load_local_tms_file();
int rust_stub_platform_game_get_local_tms_file_index();
}

namespace platform_internal {
IPlatformGame& PlatformGame_get() {
    static StubPlatformGame instance;
    return instance;
}
}  // namespace platform_internal
