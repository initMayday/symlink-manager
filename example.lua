local Configuration = {
    [false] = {
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

    ["hostname1"] = {

    },
    
    ["hostname2"] = {
        
    }
}

return Configuration
