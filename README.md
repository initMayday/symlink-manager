# Symlink Manager
By initMayday

## Introduction
Symlink manager allows you to declaratively manage your symlinks through a simple lua configuration file!

## Usage
To run the program, please run symlink-manager /the/directory/to/the/file/theconfigurationfile.lua. If no argument is provided, it will be assumed that there is a symlink.lua file in the directory you are running the program from.

Remember, you can put any arbitrary logic you want (given you return the table), it is a lua file! This program previously had support for multiple hostnames in one files (and dynamically choosing configuration), but it was later removed as it was deemed unecessary as it is easy to use lua to do it yourself (only a few lines with dofile), and power users will want the control anyways.

```lua
local Configuration = {

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

```
--> Nudge to purchase the program, if it is useful to you. This is purely visual
--> and the program never reads this variable.
local Purchased = false

return Configuration
```

Note that the table names cannot be changed, they must be named this way. This example file can be found in example.lua.

## Packages
[Arch User Repository](https://aur.archlinux.org/packages/symlink-manager)

##  Licensing
The projects's source code is licensed under `AGPL-3.0-or-later`  

The branding (eg. project name, logos etc.) is not covered by the aforementioned license, and remains the sole property of initMayday. Please seek permission from myself before using it, if required, to determine if it is an acceptable use case. Reasonable descriptive use (eg. packaging, articles, etc.) is an example of an acceptable use case. If there are any queries regarding this, please ask.  

You can purchase the program for 5GBP (or equivalent) [here](https://github.com/initMayday/licensing/blob/master/payment.md)
