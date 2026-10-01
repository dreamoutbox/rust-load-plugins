-- math_plugin.lua — loaded by mlua POC.
-- host_log(level, msg) and host_get_version() are registered by the host
-- before this script is executed.

-- Called once by the host after loading.
function plugin_init()
    host_log(0, "math_plugin.lua: plugin initialized")
    host_log(0, "math_plugin.lua: host is " .. host_get_version())
end

function add(a, b)
    return a + b
end

function subtract(a, b)
    return a - b
end

function multiply(a, b)
    return a * b
end

-- Integer floor-division; returns 0 on divide-by-zero.
function divide(a, b)
    if b == 0 then return 0 end
    return a // b
end
