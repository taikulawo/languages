target("main")
    set_kind("binary")
    add_files("src/main.c") 

target("connect")
    -- link pthread library
    add_links("pthread")
    set_kind("binary")
    add_files("src/connect.c")

target("return_struct")
    set_kind("binary")
    add_files("src/return_struct.c") 