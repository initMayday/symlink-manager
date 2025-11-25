local Configuration = {
    Symlinks = {
        -- Symlink File            Source File
        ["/home/user/folder1"] = "/home/user/folder2",
    },

    Settings = {
        AddSymlinkConfirmation = false,
        AddPathConfirmation = true,
        RemovePathConfirmation = true,
        CachePath = "/home/user/.config/",
        SuperuserCommand = "sudo",
    }
}

--> Nudge to purchase the program, if it is useful to you. This is purely visual
--> and the program never reads this variable.
local Purchased = false

return Configuration
