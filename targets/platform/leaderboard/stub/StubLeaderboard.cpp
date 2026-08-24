#include "StubLeaderboard.h"

#include "platform/leaderboard/leaderboard.h"

extern "C" {
bool rust_stub_leaderboard_open_session();
bool rust_stub_leaderboard_write_stats();
bool rust_stub_leaderboard_read_stats();
bool rust_stub_leaderboard_is_idle();
}

namespace platform_internal {
IPlatformLeaderboard& PlatformLeaderboard_get() {
    static StubLeaderboard instance;
    return instance;
}
}  // namespace platform_internal
