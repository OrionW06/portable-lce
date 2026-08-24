lib_platform_rust_stubs = static_library(
    'platform_rust_stubs',
    files('rust_stubs/src/lib.rs'),
    rust_abi: 'c',
)

platform_rust_stubs_dep = declare_dependency(
    link_with: lib_platform_rust_stubs,
)

subdir('fs')
subdir('game')
subdir('input')
subdir('leaderboard')
subdir('network')
subdir('profile')
subdir('renderer')
subdir('sound')
subdir('storage')
subdir('thread')
