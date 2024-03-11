add_rules("plugin.compile_commands.autoupdate", {outputdir = "."})
target("main")
    set_kind("binary")
    set_languages("cxx11")
    add_files("src/main.cc") 

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
    set_languages("c99")
    set_kind("binary")
    set_toolchains("clang")
    add_files("src/return_struct.c")

target("sptr")
    -- https://xmake.io/#/manual/project_target?id=targetset_languages
    set_languages("cxx14")
    set_toolchains("clang")
    set_kind("binary")
    add_files("src/sptr.cc")

target("move")
    -- https://xmake.io/#/manual/project_target?id=targetset_languages
    set_languages("cxx14")
    set_toolchains("clang")
    set_kind("binary")
    add_files("src/move.cc")
    add_includedirs("include")

target("rvalue")
    -- https://xmake.io/#/manual/project_target?id=targetset_languages
    set_languages("cxx11")
    set_toolchains("clang")
    set_kind("binary")
    add_files("src/rvalue.cc")
target("variant")
    -- https://xmake.io/#/manual/project_target?id=targetset_languages
    set_languages("cxx17")
    set_toolchains("clang")
    set_kind("binary")
    add_files("src/variant.cc")
target("inheritance")
    -- https://xmake.io/#/manual/project_target?id=targetset_languages
    set_languages("c++23")
    set_toolchains("clang")
    set_kind("binary")
    add_files("src/inheritance.cc")
target("template")
    set_languages("c++20")
    set_toolchains("clang")
    set_kind("binary")
    add_files("src/template.cc")
target("queue")
    set_languages("c++20")
    set_toolchains("clang")
    set_kind("binary")
    add_files("src/queue.cc")