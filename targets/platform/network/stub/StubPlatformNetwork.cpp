#include "StubPlatformNetwork.h"

#include <string.h>
#include <wchar.h>

#include <compare>

#include "StubNetworkPlayer.h"
#include "app/common/Network/GameNetworkManager.h"
#include "minecraft/network/Socket.h"
#include "platform/network/NetTypes.h"
#include "platform/network/network.h"
#include "platform/thread/C4JThread.h"

extern "C" {
void rust_stub_network_system_flag_init(size_t flag_index_size);
void rust_stub_network_system_flag_add_player(
    const void* player_ptr,
    bool (*same_system_fn)(const void*, const void*));
void rust_stub_network_system_flag_reset();
void rust_stub_network_system_flag_set(
    const void* player_ptr, int index,
    bool (*same_system_fn)(const void*, const void*));
bool rust_stub_network_system_flag_get(const void* player_ptr, int index);
void rust_stub_network_gather_rtt_stats(
    uint32_t player_count,
    bool (*is_local_fn)(uint32_t),
    int (*get_rtt_fn)(uint32_t),
    char* out_buf, size_t out_capacity);
bool rust_stub_network_set_local_game(bool is_local);
void rust_stub_network_set_private_game(bool is_private);
bool rust_stub_network_is_host();
bool rust_stub_network_is_in_session();
void rust_stub_network_set_game_running(bool running);
bool rust_stub_network_is_leaving_game();
void rust_stub_network_set_leaving_game(bool leaving);
}

namespace platform_internal {
IPlatformNetwork& PlatformNetwork_get() {
    static StubPlatformNetwork instance;
    return instance;
}
}  // namespace platform_internal

static bool s_helper_is_same_system(const void* p1, const void* p2) {
    auto np1 = const_cast<INetworkPlayer*>(reinterpret_cast<const INetworkPlayer*>(p1));
    auto np2 = const_cast<INetworkPlayer*>(reinterpret_cast<const INetworkPlayer*>(p2));
    return np1 && np2 && np1->IsSameSystem(np2);
}

StubNetworkPlayer StubPlatformNetwork::m_players[4];

void StubPlatformNetwork::NotifyPlayerJoined(INetworkPlayer* pQNetPlayer) {
    const char* pszDescription;

    bool createFakeSocket = false;
    bool localPlayer = false;

    INetworkPlayer* networkPlayer =
        (INetworkPlayer*)addNetworkPlayer(pQNetPlayer);

    if (pQNetPlayer->IsLocal()) {
        localPlayer = true;
        if (pQNetPlayer->IsHost()) {
            pszDescription = "local host";
            m_machineQNetPrimaryPlayers.push_back(pQNetPlayer);
        } else {
            pszDescription = "local";
            createFakeSocket = true;
        }
    } else {
        if (pQNetPlayer->IsHost()) {
            pszDescription = "remote host";
        } else {
            pszDescription = "remote";
            if (IsHost()) {
                createFakeSocket = true;
            }
        }

        if (IsHost() && !m_bHostChanged) {
            bool systemHasPrimaryPlayer = false;
            for (auto it = m_machineQNetPrimaryPlayers.begin();
                 it < m_machineQNetPrimaryPlayers.end(); ++it) {
                INetworkPlayer* pQNetPrimaryPlayer = *it;
                if (pQNetPlayer->IsSameSystem(pQNetPrimaryPlayer)) {
                    systemHasPrimaryPlayer = true;
                    break;
                }
            }
            if (!systemHasPrimaryPlayer)
                m_machineQNetPrimaryPlayers.push_back(pQNetPlayer);
        }
    }
    g_NetworkManager.PlayerJoining(networkPlayer);

    if (createFakeSocket == true && !m_bHostChanged) {
        g_NetworkManager.CreateSocket(networkPlayer, localPlayer);
    }

    fprintf(stderr, "Player 0x%p \"%s\" joined; %s; voice %i; camera %i.\n",
            pQNetPlayer, pQNetPlayer->GetOnlineName(), pszDescription,
            (int)pQNetPlayer->HasVoice(), (int)pQNetPlayer->HasCamera());

    if (IsHost()) {
        SystemFlagAddPlayer(networkPlayer);
    }

    for (int idx = 0; idx < XUSER_MAX_COUNT; ++idx) {
        if (playerChangedCallback[idx])
            playerChangedCallback[idx](networkPlayer, false);
    }
}

bool StubPlatformNetwork::Initialise(CGameNetworkManager* pGameNetworkManager,
                                     int flagIndexSize) {
    m_pGameNetworkManager = pGameNetworkManager;
    m_flagIndexSize = flagIndexSize;

    rust_stub_network_system_flag_init(static_cast<size_t>(flagIndexSize));

    for (int i = 0; i < XUSER_MAX_COUNT; i++) {
        playerChangedCallback[i] = nullptr;
    }

    m_bLeavingGame = false;
    m_bLeaveGameOnTick = false;
    m_bHostChanged = false;

    m_bIsOfflineGame = false;
    m_SessionsUpdatedCallback = nullptr;

    return true;
}

void StubPlatformNetwork::Terminate() {}

int StubPlatformNetwork::GetJoiningReadyPercentage() { return 100; }

int StubPlatformNetwork::CorrectErrorIDS(int IDS) { return IDS; }

bool StubPlatformNetwork::isSystemPrimaryPlayer(INetworkPlayer* pQNetPlayer) {
    return true;
}

void StubPlatformNetwork::DoWork() {}

int StubPlatformNetwork::GetPlayerCount() { return 1; }

bool StubPlatformNetwork::ShouldMessageForFullSession() { return false; }

int StubPlatformNetwork::GetOnlinePlayerCount() { return 1; }

int StubPlatformNetwork::GetLocalPlayerMask(int playerIndex) {
    return 1 << playerIndex;
}

bool StubPlatformNetwork::AddLocalPlayerByUserIndex(int userIndex) {
    NotifyPlayerJoined(GetLocalPlayerByUserIndex(userIndex));
    return true;
}

bool StubPlatformNetwork::RemoveLocalPlayerByUserIndex(int userIndex) {
    return true;
}

bool StubPlatformNetwork::IsInStatsEnabledSession() { return true; }

bool StubPlatformNetwork::SessionHasSpace(unsigned int spaceRequired) {
    return true;
}

void StubPlatformNetwork::SendInviteGUI(int quadrant) {}

bool StubPlatformNetwork::IsAddingPlayer() { return false; }

bool StubPlatformNetwork::LeaveGame(bool bMigrateHost) {
    if (m_bLeavingGame) return true;

    m_bLeavingGame = true;
    rust_stub_network_set_leaving_game(true);

    if (IsHost() && g_NetworkManager.ServerStoppedValid()) {
        rust_stub_network_set_game_running(false);
        g_NetworkManager.ServerStoppedWait();
        g_NetworkManager.ServerStoppedDestroy();
    }
    return true;
}

bool StubPlatformNetwork::_LeaveGame(bool bMigrateHost, bool bLeaveRoom) {
    return true;
}

void StubPlatformNetwork::HostGame(
    int localUsersMask, bool bOnlineGame, bool bIsPrivate,
    unsigned char publicSlots, unsigned char privateSlots) {
    SetLocalGame(!bOnlineGame);
    SetPrivateGame(bIsPrivate);
    SystemFlagReset();

    localUsersMask |= GetLocalPlayerMask(g_NetworkManager.GetPrimaryPad());

    m_bLeavingGame = false;
    rust_stub_network_set_leaving_game(false);
    rust_stub_network_set_game_running(true);

    _HostGame(localUsersMask, publicSlots, privateSlots);
}

void StubPlatformNetwork::_HostGame(
    int usersMask, unsigned char publicSlots, unsigned char privateSlots) {}

bool StubPlatformNetwork::_StartGame() { return true; }

int StubPlatformNetwork::JoinGame(FriendSessionInfo* searchResult,
                                  int localUsersMask, int primaryUserIndex) {
    return CGameNetworkManager::JOINGAME_SUCCESS;
}

bool StubPlatformNetwork::SetLocalGame(bool isLocal) {
    m_bIsOfflineGame = isLocal;
    return rust_stub_network_set_local_game(isLocal);
}

void StubPlatformNetwork::SetPrivateGame(bool isPrivate) {
    fprintf(stderr, "Setting as private game: %s\n", isPrivate ? "yes" : "no");
    m_bIsPrivateGame = isPrivate;
    rust_stub_network_set_private_game(isPrivate);
}

void StubPlatformNetwork::RegisterPlayerChangedCallback(
    int iPad,
    std::function<void(INetworkPlayer* pPlayer, bool leaving)> callback) {
    playerChangedCallback[iPad] = std::move(callback);
}

void StubPlatformNetwork::UnRegisterPlayerChangedCallback(int iPad) {
    playerChangedCallback[iPad] = nullptr;
}

void StubPlatformNetwork::HandleSignInChange() { return; }

bool StubPlatformNetwork::_RunNetworkGame() { return true; }

void StubPlatformNetwork::UpdateAndSetGameSessionData(
    INetworkPlayer* pNetworkPlayerLeaving) {}

bool StubPlatformNetwork::RemoveLocalPlayer(INetworkPlayer* pNetworkPlayer) {
    return true;
}

StubPlatformNetwork::PlayerFlags::PlayerFlags(INetworkPlayer* pNetworkPlayer,
                                              unsigned int count) {
    count = (count + 8 - 1) & ~(8 - 1);
    this->m_pNetworkPlayer = pNetworkPlayer;
    this->flags = new unsigned char[count / 8];
    memset(this->flags, 0, count / 8);
    this->count = count;
}
StubPlatformNetwork::PlayerFlags::~PlayerFlags() { delete[] flags; }

void StubPlatformNetwork::SystemFlagAddPlayer(INetworkPlayer* pNetworkPlayer) {
    rust_stub_network_system_flag_add_player(pNetworkPlayer, s_helper_is_same_system);
}

void StubPlatformNetwork::SystemFlagReset() {
    rust_stub_network_system_flag_reset();
}

void StubPlatformNetwork::SystemFlagSet(INetworkPlayer* pNetworkPlayer,
                                        int index) {
    rust_stub_network_system_flag_set(pNetworkPlayer, index, s_helper_is_same_system);
}

bool StubPlatformNetwork::SystemFlagGet(INetworkPlayer* pNetworkPlayer,
                                        int index) {
    return rust_stub_network_system_flag_get(pNetworkPlayer, index);
}

std::string StubPlatformNetwork::GatherStats() { return ""; }

std::string StubPlatformNetwork::GatherRTTStats() {
    char buf[256] = {0};
    auto is_local_fn = [](uint32_t idx) -> bool {
        IPlatformNetwork& net = PlatformNetwork;
        INetworkPlayer* p = net.GetPlayerByIndex(idx);
        return p ? p->IsLocal() : true;
    };
    auto get_rtt_fn = [](uint32_t idx) -> int {
        IPlatformNetwork& net = PlatformNetwork;
        INetworkPlayer* p = net.GetPlayerByIndex(idx);
        return p ? p->GetCurrentRtt() : 0;
    };
    rust_stub_network_gather_rtt_stats(GetPlayerCount(), is_local_fn, get_rtt_fn, buf, sizeof(buf));
    return std::string(buf);
}

void StubPlatformNetwork::TickSearch() {}

void StubPlatformNetwork::SearchForGames() {}

void StubPlatformNetwork::SetSearchResultsReady(int resultCount) {}

std::vector<FriendSessionInfo*>* StubPlatformNetwork::GetSessionList(
    int iPad, int localPlayers, bool partyOnly) {
    return new std::vector<FriendSessionInfo*>();
}

bool StubPlatformNetwork::GetGameSessionInfo(
    int iPad, SessionID sessionId, FriendSessionInfo* foundSessionInfo) {
    return false;
}

void StubPlatformNetwork::SetSessionsUpdatedCallback(
    std::function<void()> callback) {
    m_SessionsUpdatedCallback = std::move(callback);
}

void StubPlatformNetwork::GetFullFriendSessionInfo(
    FriendSessionInfo* foundSession,
    std::function<void(bool success)> callback) {
    callback(true);
}

void StubPlatformNetwork::ForceFriendsSessionRefresh() {
    fprintf(stderr, "Resetting friends session search data\n");
}

INetworkPlayer* StubPlatformNetwork::addNetworkPlayer(
    INetworkPlayer* pQNetPlayer) {
    StubNetworkPlayer* pNetworkPlayer = new StubNetworkPlayer();
    currentNetworkPlayers.push_back(pNetworkPlayer);
    return pNetworkPlayer;
}

void StubPlatformNetwork::removeNetworkPlayer(INetworkPlayer* pQNetPlayer) {
    INetworkPlayer* pNetworkPlayer = getNetworkPlayer(pQNetPlayer);
    for (auto it = currentNetworkPlayers.begin();
         it != currentNetworkPlayers.end(); it++) {
        if (*it == pNetworkPlayer) {
            currentNetworkPlayers.erase(it);
            return;
        }
    }
}

INetworkPlayer* StubPlatformNetwork::getNetworkPlayer(
    INetworkPlayer* pQNetPlayer) {
    return pQNetPlayer;
}

INetworkPlayer* StubPlatformNetwork::GetLocalPlayerByUserIndex(int userIndex) {
    if (userIndex != 0) return nullptr;
    return getNetworkPlayer(&m_players[userIndex]);
}

INetworkPlayer* StubPlatformNetwork::GetPlayerByIndex(int playerIndex) {
    return getNetworkPlayer(&m_players[0]);
}

INetworkPlayer* StubPlatformNetwork::GetPlayerByXuid(PlayerUID xuid) {
    return getNetworkPlayer(&m_players[0]);
}

INetworkPlayer* StubPlatformNetwork::GetPlayerBySmallId(unsigned char smallId) {
    return getNetworkPlayer(&m_players[0]);
}

INetworkPlayer* StubPlatformNetwork::GetHostPlayer() {
    return getNetworkPlayer(&m_players[0]);
}

bool StubPlatformNetwork::IsHost() { return rust_stub_network_is_host(); }

bool StubPlatformNetwork::JoinGameFromInviteInfo(
    int userIndex, int userMask, const INVITE_INFO* pInviteInfo) {
    return 0;
}

void StubPlatformNetwork::SetSessionTexturePackParentId(int id) {
    m_hostGameSessionData.texturePackParentId = id;
}

void StubPlatformNetwork::SetSessionSubTexturePackId(int id) {
    m_hostGameSessionData.subTexturePackId = id;
}

void StubPlatformNetwork::Notify(int ID, uintptr_t Param) {}

bool StubPlatformNetwork::IsInSession() { return rust_stub_network_is_in_session(); }
bool StubPlatformNetwork::IsInGameplay() { return rust_stub_network_is_in_session(); }
bool StubPlatformNetwork::IsReadyToPlayOrIdle() { return true; }
