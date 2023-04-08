# Hello world: Running an example mod

This page explains how to set up a mod and includes a small example.
Step 4 and later can be changed, the process here is only for demonstrational purposes.

_TODO: Project template generator (maybe using cargo generate, maybe using custom python script)_

## 1. Clone this repo or download the latest client/server builds.
`git clone https://github.com/DragonFIghter603/aeonetica.git`
## 2. Create a new Rust Lib
`cargo init --lib`
## 3. Append this to your cargo.toml
```toml
[lib]
crate-type = ["dylib"]

[features]
client = []
server = []

[dependencies]
aeonetica_engine = { package="engine", path="../../engine" }
aeonetica_client = { package="client", path="../../client" }
aeonetica_server = { package="server", path="../../server" }
```
- If you did not create your mod inside the [mods](../../mods) directory, adjust the paths to the crates accordingly
- If you did not clone the repository, use the following dependencies instead:
```toml
[dependencies]
aeonetica_engine = { package="engine", git="https://github.com/DragonFighter603/aeonetica.git" }
aeonetica_client = { package="client", git="https://github.com/DragonFighter603/aeonetica.git" }
aeonetica_server = { package="server", git="https://github.com/DragonFighter603/aeonetica.git" }
```
## 4. Create Rust Module Files:
- `lib.rs` (already exists)
- `server.rs` (server side code)
- `client.rs` (client side code)

## 5. Template Implementation:
Replace `<ModName>` with the name of your mod:

`lib.rs`
```rs
use aeonetica_engine::register;

pub mod client;
pub mod server;

register!(client::<ModName>Client{}, server::<ModName>Server{});
```
`client.rs`
```rs
use aeonetica_client::ClientMod;

pub struct <ModName>Client {}

impl ClientMod for <ModName>Client {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client!")
    }
}
```
`server.rs`
```rs
use aeonetica_server::ServerMod;

pub struct <ModName>Server {}

impl ServerMod for <ModName>Server {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from server!");
    }
}
```
## 6. Compile your Mod
The compilation results in a zip file in the [mods](../../mods) directory of the server. This directory already exists in the `server` crate if you cloned the repo.
By default, build.py is located in [mods](../../mods) next to your mod. <br>
`py build.py --working-dir <mod_dir_path> --deploy ../server/mods`<br>
Depending on where your mod is the `--deploy` flag might need adjustment.
## 7. Edit Server Mod File
- Create a file `mods/mods.ron` next to your server executable. This should already exist in the `server` crate if you cloned the repo.
- Add/edit the contents of the file to include your mod. `<path>` is the relative path of the mod zip file without the zip extension. `<mod_name>` is the actual name of your mod.
```rust
(
    profile: "<profile_name>",
    version: "0.1.0",
    modstack: {
        "<path>:<mod_name>": []
    }
)
```
## 8. Run Server and Client
- run server (in server crate): `cargo run -- <server_ip:server_port>`
- run client (in client crate): `cargo run -- <server_ip:server_port>`
  Running the executables instead:
- run server: `server.exe <server_ip:server_port>`
- run client: `client.exe <client_ip:client_port> <server_ip:server_port>`
  Default values are:
  client address: `127.0.0.1:9000`
  server address: `0.0.0.0:6090`
