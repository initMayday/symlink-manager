--<> Global Variables
local CacheFileName = "symlinks.cache";
local RelativeScriptPath = debug.getinfo(1, "S").source:sub(2):match("(.*/)") or "./"
package.path = package.path .. ";" ..  RelativeScriptPath .. "?.lua";

--> Decipher argument path
local FileName = "symlink";
if arg[1] ~= nil then
    FileName = arg[1]:match("([^/]+)$"):gsub("%.lua$", "")
    local Directory = arg[1]:match("^(.*)/").. "/"
    package.path = package.path .. ";" .. Directory .. "/?.lua"
end

local Configuration = require(FileName);
if Configuration.Settings.SuperuserCommand ~= "" then Configuration.Settings.SuperuserCommand = Configuration.Settings.SuperuserCommand.. " "; end

local DebugOptions = { NONE=0, REMOVAL=1 };
local DebugState = DebugOptions.REMOVAL;

local Colours = {
    Reset = "\27[0m",
    Red = "\27[31m",
    Green = "\27[32m",
    Yellow = "\27[33m",
    Blue = "\27[34m",
    Magenta = "\27[35m",
    Cyan = "\27[36m",
    White = "\27[37m",
};

--<> Activation Message
if Configuration.Settings.RandomActivationMessage == true and Configuration.Settings.Licensed == false then
    if math.random(0, 6) == 1 then -- 1 in 6 chance of showing
        print(Colours.Cyan .."[ACTIVATION] If you find this product useful, please purchase a license from: https://ko-fi.com/s/f7d3444a62, it really helps!".. Colours.Reset);
    end
end

--<> Functions
--> Different Debug Levels
local function debug_print(Message, RequiredState)
    if RequiredState == DebugState then
        print("[DEBUG] ".. Message);
    end
end

--> Internal errors that cause the program to exit, but not officially
local function fake_error(Message, ExitStatus)
    print(Colours.Red.. "[EXIT] " .. Message ..Colours.Reset);
    os.exit(ExitStatus);
end

--> Get Confirmation for any external action
local function ensure_confirmation()
    local Input = string.lower(io.read());
    if Input == "y" or Input == "yes" then
        return true;
    elseif Input == "n" or Input == "no" then
        return false;
    else
        print(Colours.Red.. "Unknown Input: ".. Input .." Assuming confirmation not granted!".. Colours.Reset);
    end
end

--> Expands variables from directory path, does not check validity
local function real_path(Path)
    --local Handle = io.popen("realpath ".. Path) -- Also evaluates symlinks, which is not behaviour we want
    local Handle = io.popen("echo ".. Path);
    local Read = Handle:read("*a");
    Handle:close();
    return string.sub(Read, 0, -2);
end

--> Removes the target path
local function remove_path(Location)
    if os.execute(Configuration.Settings.SuperuserCommand .."test -e " ..Location) then
        local Confirmation = true;
        if Configuration.Settings.RemovePathConfirmation == true then
            io.write(Colours.Yellow.. "[INPUT REQUIRED] Are you sure you would like to REMOVE the path: ".. Location.. " (y/n) ".. Colours.Reset);
            Confirmation = ensure_confirmation();
        end
        if Confirmation then
            print(Colours.Red.. "Removing path: ".. Location.. Colours.Reset);
            if os.execute(Configuration.Settings.SuperuserCommand.. "rm -r ".. Location) then
                return;
            end
        end
        fake_error("Unable to remove path: ".. Location, -2);
    end
end

--> Creates the target path
local function create_path(Location)
    local Confirmation = true;
    if Configuration.Settings.AddPathConfirmation == true then
        io.write(Colours.Yellow.. "[INPUT REQUIRED] Are you sure you would like to CREATE the path: ".. Location.. " (y/n)".. Colours.Reset);
        Confirmation = ensure_confirmation();
    end
    if Confirmation then
        print(Colours.Green.. "Creating path: ".. Location.. Colours.Reset);
        if os.execute(Configuration.Settings.SuperuserCommand.. "mkdir -p ".. Location) then
            return;
        end
    end
    fake_error("Unable to create path at: ".. Location, -2);
end

local function remove_invalid_links(SymlinkPath, SourcePath)
        --> Check if the symlink is still explicitly listed
        if Configuration.Symlinks[SymlinkPath] ~= nil then
            --> Expand variables, it is still listed
            SymlinkPath = real_path(SymlinkPath);
            SourcePath = real_path(SourcePath);
            --> Check if the path is a symlink
            if os.execute(Configuration.Settings.SuperuserCommand .."test -L " ..SymlinkPath) then
                --> Check if the symlink points to the intended source directory
                local Handle = io.popen(Configuration.Settings.SuperuserCommand .."readlink ".. SymlinkPath);
                if Handle:read() ~= SourcePath then
                    debug_print("Removal Reason 1 (Symlink Does not point to intended directory)", DebugOptions.REMOVAL);
                    remove_path(SymlinkPath);
                end
                Handle:close();
            else
                debug_print("Removal Reason 2 (Path is not a symlink)", DebugOptions.REMOVAL);
                remove_path(SymlinkPath);
            end
        else
            --> Symlink is no longer explicitly listed, we can remove it
            debug_print("Removal Reason 3 (Symlink is no longer required)", DebugOptions.REMOVAL);
            remove_path(SymlinkPath);
        end

end

--<> Manage Cache
Configuration.Settings.CachePath = real_path(Configuration.Settings.CachePath);
--> Ensure trailing slash
if string.sub(Configuration.Settings.CachePath, -1) ~= "/" then
    Configuration.Settings.CachePath = Configuration.Settings.CachePath.. "/";
end

--> Test if the path exists
local CacheDirectoryExists = os.execute(Configuration.Settings.SuperuserCommand .."test -d ".. Configuration.Settings.CachePath);
if CacheDirectoryExists == nil then
    print("[LOG] (PLEASE CHECK THE DIRECTORY IS NOT DEFAULT) Cache Directory doesn't exist, generating a new one at: ".. Configuration.Settings.CachePath ..CacheFileName);

    --> Create the directory if it doesn't exist
    create_path(Configuration.Settings.CachePath);
end

--> Test if the File exists
local CacheFileExists = os.execute(Configuration.Settings.SuperuserCommand .."test -f ".. Configuration.Settings.CachePath ..CacheFileName);

if CacheFileExists == nil then
    print("[LOG] Cache File doesn't exist, generating a new one at: ".. Configuration.Settings.CachePath ..CacheFileName);

    --> Create the file if it doesn't exist
    if not os.execute(Configuration.Settings.SuperuserCommand.. "touch ".. Configuration.Settings.CachePath ..CacheFileName) then
        fake_error("Failed to create Cache File at: ".. Configuration.Settings.CachePath, -1);
    end
end

print("[LOG] Reading Cache File!")
local Handle = io.popen(Configuration.Settings.SuperuserCommand .."cat ".. Configuration.Settings.CachePath ..CacheFileName);
local CacheFileContents = Handle:read(); Handle:close();

if CacheFileContents ~= nil then
    --> Split the string every 2 null bytes (as symlinks come in diretory pairs)
    local Splits = {};
    local PreviousSplit = 0;
    local Switch = false;

    for Index = 1, #CacheFileContents do
        local Character = CacheFileContents:sub(Index, Index);
        if Character == '\0' then
            --> Switch flips from true to false, to only get every other null byte.
            if Switch == true then
                table.insert(Splits, CacheFileContents:sub(PreviousSplit, Index-1));
                PreviousSplit = Index+1;
                Switch = false;
            else
                Switch = true;
            end
        end
    end

    --> Split each pair into its individual directory
    for Index, Value in pairs(Splits) do
        local NullPosition = Value:find("\0");
        local SymlinkPath = Value:sub(1, NullPosition-1);
        local SourcePath = Value:sub(NullPosition+1, -1);
        --print("SymlinkPath: ".. SymlinkPath .." SourcePath: ".. SourcePath);
        remove_invalid_links(SymlinkPath, SourcePath);
    end
end

-- *NEW ADDED PART* --
--> Do the same for above, but for any new pairs that may yet not be cached (as it is their first run)
for SymlinkPath, SourcePath in pairs(Configuration.Symlinks) do
    remove_invalid_links(SymlinkPath, SourcePath);
end

--<> Create Symlinks and update cache
--> SymlinkPath is Link Dir, SourcePath is Source dir
local NewCache = "";
for SymlinkPath, SourcePath in pairs(Configuration.Symlinks) do
    --> Only create path if it doesn't exist
    if os.execute(Configuration.Settings.SuperuserCommand .."test -e ".. SymlinkPath) == nil then
        --> Ensure that source path exists
        if os.execute(Configuration.Settings.SuperuserCommand .."test -e ".. SourcePath) == nil then
            fake_error("Source path does not exist: ".. SourcePath, -1);
        end
        --> Create the symlink
        local Confirmation = true;
        if Configuration.Settings.AddSymlinkConfirmation then
            io.write(Colours.Yellow.. "[INPUT REQUIRED] Are you sure you would like a symlink from Source: ".. SourcePath.. " Symlink File: ".. SymlinkPath .." (y/n)".. Colours.Reset);
            Confirmation = ensure_confirmation();
        end
        if Confirmation then
            os.execute(Configuration.Settings.SuperuserCommand .."ln -s ".. SourcePath .. " ".. SymlinkPath);
            print(Colours.Green .."[LOG] Created Symlink, Source: ".. SourcePath .. ", Symlink File: ".. SymlinkPath ..Colours.Reset);
        end
    end

    --> Update string to write to cache file
    NewCache = NewCache.. SymlinkPath.."\\0"..SourcePath.."\\0";
end

--> Write to cache file
print("[LOG] Writing Cache File!")
os.execute(Configuration.Settings.SuperuserCommand.. "bash -c \'printf \"".. NewCache .."\" > " .. Configuration.Settings.CachePath .. CacheFileName.. "'");

--> Original way, sudo failed to work with redirection of >
--os.execute(Configuration.Settings.SuperuserCommand.. "printf \"".. NewCache .."\" > " .. Configuration.Settings.CachePath .. CacheFileName);
