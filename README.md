# Desym
Desym allows you to declaratively manage your symlinks through a simple lua configuration file!

## Usage
To run the program, please run desym /the/directory/to/the/file/theconfigurationfile.lua. If no argument is provided, it will be assumed that there is a symlinks.lua file in the directory you are running the program from.

## Configuration
Your lua configuration file must return a table. From here on, that table will be referred to as the root table.
```lua
return {
    --> Contains key value pairs. The argument order
    --> is inverse to what ln takes, to allow for unique keys
    Symlinks = {
        -- Symlink File            Source File
        ["/home/user/folder1"] = "/home/user/folder2",
        --> Folder 1 now mirrors what is in folder 2


        ["/home/user/.config/program.conf"] = "/home/user/Config/program.conf",
        -- ...
    },

    Settings = {
        --> Whether to ask you to allow the creation of a symlink
        AddSymlinkConfirmation = false,
        --> Whether to ask you to allow the creation of a path,
        --> if the path to the symlink file you specified does not exist
        AddPathConfirmation = true,
        --> Whether to ask you to remove an existing path at the symlink file
        --> location, if one exists (and hence conflicts)
        RemovePathConfirmation = true,
        --> Cache file of what symlinks desym has made, so it can remove them
        --> if they are removed from the config
        CachePath = "/home/user/.config/",
        --> The command prepended to the bash commands ran, to elevate
        --> permissions as required
        SuperuserCommand = "sudo",
        --> Never read, just a nudge to purchase the program if it
        --> useful to you
        Purchased = false,
    }
}
```

## Packages
| Repo | Source |
| :--: | :--: |
| Arch User Repository | [Link](https://aur.archlinux.org/packages/desym) |

##  Licensing
The projects's source code is licensed under `AGPL-3.0-or-later`  

The branding (eg. project name, logos etc.) is not covered by the aforementioned license, and remains the sole property of initMayday. Please seek permission from myself before using it, if required, to determine if it is an acceptable use case. Reasonable descriptive use (eg. packaging, articles, etc.) is an example of an acceptable use case. If there are any queries regarding this, please ask.  

You can purchase the program for 5GBP (or equivalent) [here](https://github.com/initMayday/licensing/blob/master/payment.md)
