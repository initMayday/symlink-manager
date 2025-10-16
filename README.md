# Symlink Manager
By initMayday

## Introduction
Symlink manager allows you to declaratively manage your symlinks through a simple lua configuration file!

## Usage
To run the program, please run symlink-manager /the/directory/to/the/file/theconfigurationfile.lua. If no argument is provided, it will be assumed that there is a symlink.lua file in the directory you are running the program from.

You can use the same configuration file for multiple machines, by specifyng hostname. Remember, you can put any arbitrary logic you want (given you return the table), it is a lua file!

```lua
local Configuration = {

    --> Configuration for this specific hostname, you can have multiple of these for different devices
    ["hostname"] = {
        Symlinks = {
            -- Symlink File            Source File
            ["/home/user/folder1"] = "/home/user/folder2",
        },
```
The key is link file directory, so the link file will be created at /home/user/folder1 in this scenario. It will point to /home/user/folder2. Please note that this is inverse to how ln works, as since lua cannot have multiple of the same keys, inversing the regular order allows for 1 directory to have multiple link files pointing to it.
```lua
        Settings = {
            AddSymlinkConfirmation = false;
            AddPathConfirmation = true;
            RemovePathConfirmation = true;
            CachePath = "/home/user/.config/";
            SuperuserCommand = "sudo";
        }
```
- AddSymlinkConfirmation: Will ask you to allow the creation of the symlink file at the specified directory (bool: true / false)
- AddPathConfirmation:  Will ask you to allow the creation of the path at the specified directory (bool: true / false)
- RemovePathConfirmation:  Will ask you to remove the creation of the path at the specified directory (bool: true / false)
- CachePath: To know where symlink-manager previously created symlinks, a cache file must be used, this specifies the directory. **Please change "user" to your user!** (string: eg. /home/initMayday/.config/)
- SuperuserCommand: Prepends the command to the bash commands that the program runs (string: eg. "sudo", "doas")

```lua
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
        }
    }
}

--> Nudge to purchase the program, if it is useful to you. This is purely visual
--> and the program never reads this variable.
local Purchased = false

return Configuration
```
Note that the table names cannot be changed, they must be named this way. This example file can be found in example.lua.

## Packages
[Arch User Repository](https://aur.archlinux.org/packages/symlink-manager)

##  Licensing
Symlink-Manager utilises the CC BY-NC-SA 4.0 license, following this [interpretation](https://github.com/initMayday/licensing/blob/master/CC-BY-NC-SA.md).
You can purchase the program for 3GBP (or equivalent) [here](https://github.com/initMayday/licensing/blob/master/payment.md).
