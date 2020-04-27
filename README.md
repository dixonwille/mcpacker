# MC Packer

[Beta Release Items](https://github.com/dixonwille/mcpacker/projects/1)

Manage and maintain a Minecraft Modpack. This allows a user to keep certain files checked into a git repository without all the bloat and changes in the `minecraftinstance.json`.

## Usage

`mcpacker init` - Looks for `minecraftinstance.json` and populates the `.manifest.yaml` from that. If it cannot find `minecraftinstance.json`, it will prompt the user for enough information to build a basic manifest.

> **Note**: it is probably best to start new mod pack in the twitch launcher. Working without one, would require running `mcpacker pack` first then import the zip to the launcher. Then move the `.manifest.yaml` file to the folder the launcher uses.

`mcpacker sync` - Compares `.manifest.yaml` with `minecraftinstance.json`. It will add and remove mods as needed from `.manifest.yaml` as well as the `mods/` folder. You can use this in a [git hook](https://git-scm.com/docs/githooks) to sync on certain actions (after pulling changes, or before commiting changes). This command should be ran before packing your mod pack to make sure everything is included.

`mcpacker pack` - Reads the `.manifest.yaml` and creates a zip file that can be imported into the twitch launcher. It adds all files found in the `includes` section as `overrides`.

`mcpacker includes add [PATH...]` - Adds multiple paths to the includes section of the manifest. It is best to use this command rather than manually update the file as it does some house keeping to keep the list as small as it needs to be.

`mcpacker includes remove [PATH...]` - Removes multiple paths from the includes section of the manifest.

## Workflows

### Start a new Mod Pack

1. Open up the Twitch launcher and create a new Custom Pack
1. Open up the folder with your new Custom Pack (same place the `minecarftinstance.json`)
1. Run `mcpacker init`
1. Modify the pack as you wish

### Use MCPacker with existing pack

1. Open up the folder with your Mod Pack
1. Run `mcpacker init`

### Modifying Mod Pack

1. Modify and test as usual through the the Twitch Launcer
1. Run `mcpacker sync` so it picks up the new changes
1. Run `mcpacker includes add/remove` as needed to make sure configuration changes are packed

### Using Git

1. Start one of the Workflows above
1. Commit `.manifest.yaml` and any config files that need to be added (`includes` section of manifest). **Note**: Will be easier once MCPacker can create the `.gitignore`

> `mods` folder and `minecraftinstance.json` are not needed to be checked in