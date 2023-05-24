<div align="center">

![logo](../assets/font_and_glyphs/logo_banner_upscaled_x10.png)

<h1>The Aeonetica Multiplayer Game Engine</h1>

#### [📑Documentation](https://github.com/DragonFIghter603/aeonetica/wiki) [⚖️License](../LICENSE) [🌿Release Branch](https://github.com/DragonFIghter603/aeonetica/tree/release) [🚀Releases](https://github.com/DragonFIghter603/aeonetica/releases)

</div>

[![License](https://img.shields.io/github/license/DragonFIghter603/aeonetica?style=flat-square)](https://github.com/DragonFIghter603/aeonetica/blob/main/LICENSE)
[![Issues](https://img.shields.io/github/issues/DragonFIghter603/aeonetica?style=flat-square)](https://github.com/DragonFIghter603/aeonetica/issues)
![GitHub pull requests](https://img.shields.io/github/issues-pr/DragonFIghter603/aeonetica?style=flat-square)
![Platform](https://img.shields.io/badge/platform-linux%20|%20windows-blueviolet?style=flat-square)
[![Stars](https://img.shields.io/github/stars/DragonFIghter603/aeonetica?style=flat-square)](https://github.com/DragonFIghter603/aeonetica/stargazers)
[![Forks](https://img.shields.io/github/forks/DragonFIghter603/aeonetica?style=flat-square)](https://github.com/DragonFIghter603/aeonetica/network/members)
![GitHub repo size](https://img.shields.io/github/repo-size/DragonFIghter603/aeonetica?style=flat-square)
![Lines of code](https://raster.shields.io/tokei/lines/github/DragonFIghter603/aeonetica?style=flat-square)

---

2D multiplayer moddable game engine with server side ECS

## Quickstart
1. Clone the repo and (optionally) go to [release branch](https://github.com/DragonFIghter603/aeonetica/tree/release) for stability
    ```shell
    git clone https://github.com/DragonFIghter603/aeonetica.git --recursive
    git checkout -b release
    ```
2. Compile the client, server and mods
    ```shell
    py build.py
    ```
3. Run first the client and then the server in two separate command prompts next to each other
    ```shell
    cd server
    cargo run -- <server_ip:server_port>
    # example: cargo run -- 127.0.0.1:6090

    # and
   
    cd client
    cargo run -- <client_ip:client_port> <server_ip:server_port> 
    # example: cargo run -- 127.0.0.1:9000 127.0.0.1:6090
    ```
    Alternatively, run the binaries of client/server from `<crate>/target/release/<executable>.exe`. <br>
    Build with `--release` flag `py build.py --release` and `cargo run --rlease` for better performance. <br>
    For multiple clients, use a different `clipet_port` for each: `9000`, `9001`, ...

---

<img src="img/progress_screenshot.png" alt="" style="width: 510px; image-rendering: pixelated">

---

