
local function contains(t, value)
    for _, v in ipairs(t) do
        if v == value then return true end
    end
    return false
end

contains(directories, "desktop")