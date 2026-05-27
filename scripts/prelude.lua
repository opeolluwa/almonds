
local directories = require("./directories")


local function contains(t, value)
    for _, v in ipairs(t) do
        if v == value then return true end
    end
    return false
end


local function valid_directory()
   local target = arg[1]
	if not contains(directories.directories, target) then
		print("Invalid target directory")
		os.exit(1)
	end
end



return {
    valid_directory = valid_directory,
}