-- math_plugin.lua — loaded by mlua POC.
-- The host calls these functions by name after loading this file.

function add(a, b)
    return a + b
end

function subtract(a, b)
    return a - b
end

function multiply(a, b)
    return a * b
end

-- Returns 0 on divide-by-zero to mirror the other plugin behaviours.
-- Use integer floor-division (//) so the result is always an integer;
-- float division (/) produces 3.333… which math.tointeger then rejects as nil.
function divide(a, b)
    if b == 0 then return 0 end
    return a // b
end
