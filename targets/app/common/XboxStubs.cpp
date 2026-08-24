#include "platform/XboxStubs.h"

#include "platform/PlatformTypes.h"

extern "C" {
bool rust_is_equal_xuid(PlayerUID a, PlayerUID b);
uint32_t rust_xuser_get_signin_info(uint32_t dwUserIndex, uint32_t dwFlags,
                                    PXUSER_SIGNIN_INFO pSigninInfo);
const char* rust_cxui_string_table_lookup_id(const char* szId);
const char* rust_cxui_string_table_lookup_index(uint32_t nIndex);
void rust_cxui_string_table_clear();
int32_t rust_cxui_string_table_load(const char* szId);
uint32_t rust_xget_language();
uint32_t rust_xget_locale();
uint32_t rust_xenable_guest_signin(bool fEnable);
}

bool IsEqualXUID(PlayerUID a, PlayerUID b) { return rust_is_equal_xuid(a, b); }

uint32_t XUserGetSigninInfo(uint32_t dwUserIndex, uint32_t dwFlags,
                            PXUSER_SIGNIN_INFO pSigninInfo) {
    return rust_xuser_get_signin_info(dwUserIndex, dwFlags, pSigninInfo);
}

const char* CXuiStringTable::Lookup(const char* szId) {
    return rust_cxui_string_table_lookup_id(szId);
}

const char* CXuiStringTable::Lookup(uint32_t nIndex) {
    return rust_cxui_string_table_lookup_index(nIndex);
}

void CXuiStringTable::Clear() { rust_cxui_string_table_clear(); }

int32_t CXuiStringTable::Load(const char* szId) {
    return rust_cxui_string_table_load(szId);
}

uint32_t XGetLanguage() { return rust_xget_language(); }
uint32_t XGetLocale() { return rust_xget_locale(); }
uint32_t XEnableGuestSignin(bool fEnable) {
    return rust_xenable_guest_signin(fEnable);
}
