# Hytale Save Editor

A desktop application for editing Hytale save files. This tool allows users to modify various aspects of their save data through a user-friendly graphical interface.

## Features

- **Dashboard**: Overview of current save data.
- **Player Management**: Edit player stats, inventories, and associated data.
- **World Editing**: Modify world properties and settings.
- **Permissions System**: Manage player permissions and roles.
- **Security**: Edit security configurations and access controls.
- **Memories**: View and modify game memories/states.
- **Mods**: Manage mod-related data within saves.

## Installation

Download the latest version from the [Releases](https://github.com/EdgeTypE/hytale-save-editor/releases) page.

Extract the zip file to a location of your choice.

## Usage

1. Run the `hytale-save-editor.exe` executable.
2. Click "Open Save" to select your Hytale save directory.
3. Use the sidebar to navigate between different editors.
4. Make your changes and save.

## Building from Source

If you want to build the project yourself:

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Clone the repository:
   ```bash
   git clone https://github.com/EdgeTypE/hytale-save-editor.git
   cd hytale-save-editor
   ```
3. Run or build:
   ```bash
   # Run directly
   cargo run --release --bin hytale-save-editor
   
   # Build executable (output in target/release/)
   cargo build --release --bin hytale-save-editor
   ```

## Roadmap

- [ ] World Map visualization
- [ ] Chunk file editing support
- [ ] Chunk regeneration tools

## License

Distributed under the MIT License.
