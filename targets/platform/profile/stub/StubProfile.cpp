#include "StubProfile.h"

#include <cstdio>
#include <cstring>
#include <functional>

#include "../ProfileConstants.h"
#include "platform/input/input.h"

extern "C" {
void rust_stub_profile_initialise(int game_defined_data_size_x4);
int rust_stub_profile_get_locked_profile();
void rust_stub_profile_set_locked_profile(int prof);
bool rust_stub_profile_is_signed_in(int quadrant);
bool rust_stub_profile_is_signed_in_live(int prof);
bool rust_stub_profile_is_guest(int quadrant);
bool rust_stub_profile_query_signin_status();
void rust_stub_profile_get_xuid(int pad, PlayerUID* out_xuid);
bool rust_stub_profile_are_xuids_equal(PlayerUID xuid1, PlayerUID xuid2);
bool rust_stub_profile_xuid_is_guest(PlayerUID xuid);
bool rust_stub_profile_allowed_to_play_multiplayer(int prof);
bool rust_stub_profile_get_chat_and_content_restrictions(
    int pad, bool* pb_chat_restricted, bool* pb_content_restricted, int* pi_age);
char* rust_stub_profile_get_gamertag(int pad);
void rust_stub_profile_get_display_name(int pad, char* out_buf, size_t capacity);
IPlatformProfile::PROFILESETTINGS* rust_stub_profile_get_dashboard_profile_settings(
    int pad);
void* rust_stub_profile_get_game_defined_profile_data(int quadrant);
void rust_stub_profile_allowed_player_created_content(
    int pad, bool this_quadrant_only, bool* all_allowed, bool* friends_allowed);
bool rust_stub_profile_can_view_player_created_content(
    int pad, bool this_quadrant_only, PlayerUID* p_xuids, unsigned int xuid_count);
}

namespace platform_internal {
IPlatformProfile& PlatformProfile_get() {
    static StubProfile instance;
    return instance;
}
}  // namespace platform_internal

void StubProfile::Initialise(std::uint32_t, std::uint32_t, unsigned short,
                             unsigned int, unsigned int, std::uint32_t*,
                             int iGameDefinedDataSizeX4, unsigned int*) {
    rust_stub_profile_initialise(iGameDefinedDataSizeX4);
}

int StubProfile::GetLockedProfile() {
    return rust_stub_profile_get_locked_profile();
}

void StubProfile::SetLockedProfile(int iProf) {
    rust_stub_profile_set_locked_profile(iProf);
}

bool StubProfile::IsSignedIn(int iQuadrant) {
    return rust_stub_profile_is_signed_in(iQuadrant);
}

bool StubProfile::IsSignedInLive(int iProf) {
    return rust_stub_profile_is_signed_in_live(iProf);
}

bool StubProfile::IsGuest(int iQuadrant) {
    return rust_stub_profile_is_guest(iQuadrant);
}

bool StubProfile::QuerySigninStatus() {
    return rust_stub_profile_query_signin_status();
}

void StubProfile::GetXUID(int iPad, PlayerUID* pXuid, bool) {
    rust_stub_profile_get_xuid(iPad, pXuid);
}

bool StubProfile::AreXUIDSEqual(PlayerUID xuid1, PlayerUID xuid2) {
    return rust_stub_profile_are_xuids_equal(xuid1, xuid2);
}

bool StubProfile::XUIDIsGuest(PlayerUID xuid) {
    return rust_stub_profile_xuid_is_guest(xuid);
}

bool StubProfile::AllowedToPlayMultiplayer(int iProf) {
    return rust_stub_profile_allowed_to_play_multiplayer(iProf);
}

bool StubProfile::GetChatAndContentRestrictions(int iPad, bool* pbChatRestricted,
                                                bool* pbContentRestricted,
                                                int* piAge) {
    return rust_stub_profile_get_chat_and_content_restrictions(
        iPad, pbChatRestricted, pbContentRestricted, piAge);
}

char* StubProfile::GetGamertag(int iPad) {
    return rust_stub_profile_get_gamertag(iPad);
}

std::string StubProfile::GetDisplayName(int iPad) {
    char buf[64] = {0};
    rust_stub_profile_get_display_name(iPad, buf, sizeof(buf));
    return std::string(buf);
}

int StubProfile::SetDefaultOptionsCallback(
    std::function<int(PROFILESETTINGS*, int)> callback) {
    return 0;
}

IPlatformProfile::PROFILESETTINGS* StubProfile::GetDashboardProfileSettings(
    int iPad) {
    return rust_stub_profile_get_dashboard_profile_settings(iPad);
}

void* StubProfile::GetGameDefinedProfileData(int iQuadrant) {
    return rust_stub_profile_get_game_defined_profile_data(iQuadrant);
}

void StubProfile::AllowedPlayerCreatedContent(int iPad, bool thisQuadrantOnly,
                                              bool* allAllowed,
                                              bool* friendsAllowed) {
    rust_stub_profile_allowed_player_created_content(
        iPad, thisQuadrantOnly, allAllowed, friendsAllowed);
}

bool StubProfile::CanViewPlayerCreatedContent(int iPad, bool thisQuadrantOnly,
                                              PlayerUID* pXuids,
                                              unsigned int xuidCount) {
    return rust_stub_profile_can_view_player_created_content(
        iPad, thisQuadrantOnly, pXuids, xuidCount);
}

int StubProfile::GetPrimaryPad() { return PlatformInput.GetPrimaryPad(); }
void StubProfile::SetPrimaryPad(int iPad) { PlatformInput.SetPrimaryPad(iPad); }
