# MC Packer

## Todo

- Sub command init
    - Reads minecraft-instance.json if it exists
    - If no minecraft-instance.json prompt for enough information to generate blank manifest.json
    - Creates a manifest.toml file for this tool
        - Has IDs for project and file
        - Has Forge version
        - Needs all information needed to create manifest.json for zip file
- Sub command sync
    - Reads the minecraft-instance.json
    - Modifies manifest.toml as needed
    - Downloads mods that are needed if not already downloaded
    - Adds to override section if jar file is in mod folder but not in project list
- Sub command pack
    - Pack a zip file with appropriate files