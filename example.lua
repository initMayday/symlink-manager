local Configuration = {

    --> Configuration for this specific hostname, you can have multiple of these for different devices
    ["hostname"] = {
        Symlinks = {
            -- Symlink File            Source File
            ["/home/user/folder1"] = "/home/user/folder2",
        },

        Settings = {
            AddSymlinkConfirmation = false;
            AddPathConfirmation = true;
            RemovePathConfirmation = true;
            CachePath = "/home/user/.config/";
            SuperuserCommand = "sudo";
        }
    },

    --> false means that this is the default configuration if no specific hostname configuration for that hostname is specifed.
    --> it is optional. You can just manually specify all hostnames this file will be used on.
    [false] = {
        Symlinks = {},
        Settings = {
            AddSymlinkConfirmation = false;
            AddPathConfirmation = true;
            RemovePathConfirmation = true;
            CachePath = "$HOME/.config/";
            SuperuserCommand = "doas";
        },
    }
}

--> Nudge to purchase the program, if it is useful to you. This is purely visual
--> and the program never reads this variable.
local Purchased = false

return Configuration
