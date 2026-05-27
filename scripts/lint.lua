local function run(dir, cmd)
  local full = string.format("cd %s && %s", dir, cmd)
  local ok = os.execute(full)
  if not ok then
    io.stderr:write("Command failed: " .. full .. "\n")
    os.exit(1)
  end
end

local function lint(target)
  if target == "all" then
    run("desktop", "npm run format")
    run("desktop", "npm run lint:fix")
    run("kernel", "sh ../scripts/format")
    run("server", "sh ../scripts/format")
    run("desktop/src-tauri", "sh ../../scripts/format")
  elseif target == "desktop" then
    run("desktop", "npm run format")
    run("desktop", "npm run lint:fix")
  elseif target == "kernel" then
    run("kernel", "sh ../scripts/format")
  elseif target == "server" then
    run("server", "sh ../scripts/format")
  elseif target == "desktop-tauri" then
    run("desktop/src-tauri", "sh ../../scripts/format")
  else
    io.stderr:write("Unknown target: " .. target .. "\n")
    io.stderr:write("Targets: all, desktop, kernel, server, desktop-tauri\n")
    os.exit(1)
  end
end

local target = arg[1]
if not target then
  io.stderr:write("Usage: lua lint.lua <target>\n")
  io.stderr:write("Targets: all, desktop, kernel, server, desktop-tauri\n")
  os.exit(1)
end

lint(target)
