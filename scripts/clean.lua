local directories = require("./directories.lua")
local prelude = require("./prelude")

local target = arg[1]
local valid_targets = directories.directories


local function clean_desktop()
    os.execute("cd desktop && rm -rf node_modules dist .nuxt && cd src-tauri && cargo clean")
end

local function clean_kernel()
    os.execute("cd kernel && cargo clean")
end

local function clean_server()
    os.execute("cd server && cargo clean")
end

local function clean()
    if not prelude.contains(valid_targets, target) then
        print("Invalid target '" .. (target or "") .. "'. Use one of: desktop, kernel, server, all")
        os.exit(1)
    end

    print("Cleaning " .. target .. " build assets")

    if target == "desktop" then
        clean_desktop()
    elseif target == "kernel" then
        clean_kernel()
    elseif target == "server" then
        clean_server()
    elseif target == "all" then
        clean_desktop()
        clean_kernel()
        clean_server()
    end
end

clean()
