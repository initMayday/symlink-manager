local Configuration = {
    Files = {
        -- Symlink File            Source File
        ["/home/pika/coolfile.txt"] = "lmaobozoratioscrrt",
    },

    Symlinks = {

    },

    Settings = {
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
