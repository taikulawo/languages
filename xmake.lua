target("main")
    set_kind("binary")
    add_files("src/main.c") 

target("connect")
    -- link pthread library
    set_languages("c99")
    add_links("pthread")
    set_kind("binary")
    add_files("src/connect.c")
    -- use clang can remove libgcc_s.so.1 dependency
    -- https://unix.stackexchange.com/questions/1812/what-does-libgcc-s-so-contain
    set_toolchains("clang")
target("return_struct")
    -- https://xmake.io/#/manual/project_target?id=targetset_languages
    set_languages("cxx17")
    set_toolchains("clang")
    set_kind("binary")
    add_files("src/return_struct.c")