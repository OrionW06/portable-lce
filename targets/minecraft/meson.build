fs = import('fs')
minecraft_sources = files(fs.read('sources.txt').strip().split('\n'))

lib_rust_minecraft = static_library(
    'rust_minecraft',
    files('rust_minecraft/src/lib.rs'),
    rust_abi: 'c',
    link_with: lib_platform_rust_stubs,
)

rust_minecraft_dep = declare_dependency(link_with: lib_rust_minecraft)

lib_minecraft = static_library(
    'minecraft',
    minecraft_sources,
    dependencies: [
        dependency('zlib'),
        dependency('glm'),
        dependency('glew'),
        nbt_dep,
        java_dep,
        util_dep,
        assets_localisation_dep,
        platform_sound_dep,
        platform_renderer_dep,
        platform_input_dep,
        platform_profile_dep,
        platform_storage_dep,
        platform_fs_dep,
        platform_thread_dep,
        rust_minecraft_dep,
    ],
    include_directories: include_directories('..'),
    cpp_args: global_cpp_args + global_cpp_defs,
)

minecraft_dep = declare_dependency(link_with: [lib_minecraft, lib_rust_minecraft])
