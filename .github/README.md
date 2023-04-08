# Aeonetica Engine
2D multiplayer moddable game engine with server side ECS

### [Documentation](docs/README.md)

### crates
- [client](../client): client executable, web client, client loop, graphics, etc
- [server](../server): server executable, web server, server game loop
- [mods/coremod](../mods/coremod): main (example) mod, compiles to `.dll` (windows) or `.so` (linux), has client/server stuff separated via features<br>
-> runtime folders in [client](../client/runtime)/[server](../server/runtime) hold all the extracted library files<br>
-> put mods in [server/mods](../server/mods) folder and include them in [mods.ron](../server/mods/profile.ron) 

### how to run
- compile the mods you changed using [build.bat](../mods/coremod/build.bat) or [build.sh](../mods/coremod/build.sh)
- start server with args specified in [main](../server/src/main.rs)
- start client with args specified in [main](../client/src/main.rs)

# TODO/Fixes:
- [x] ECS
- getting a screen & client side rendering
- [x] clean up client mod loading
- [x] cleanup of error handling
  - [x] client side
  - [x] server side
- [x] utility functions that return the path to a lib (and have platform appropriate ending)
- [ ] checking whether all mods are valid after loading by checking for files and checking which platforms are supported
- [x] better mod build script using probably python, make it platform  generic

---
# ECS (see thoughts below)
- server side ecs
- module can register an associated client side renderer (trait)
- module can register associated client side event (such as keypress) functions (trait)
- can communicate with client via packets (communication event function)<br>
  -> the renderer can take simple actions, such as interpolation of position
to ease the load on the network

### Important note for adding ECS functionality:
Every time you need to make `&mut Engine` available while iterating over ECS components,
do `.cloned().collect::<Vec<_>>()` to whatever you're iterating (best Id or TypeId and not the actual object)
to make the iterator not be invalid when the body of the loop changes it.
This way some keys might return `None` and new elements will not be called,
but the iterator does not crash.

This is only needed when the body of the loop is capable of changing the structure
you are looping over. An example of that would be iterating over entity id's 
and calling a mod function, which could add or remove entites since it has access to `&mut Engine`.

Background: Changing the size of a collection such as a vector while iterating 
causes undefined behaviour, (most likely STATUS_ACCESS VIOLATION)
---
# Some thoughts below:
# ecs networking system

### Entity
- ComponentA
- ComponentB

-> each component is unique by type<br>
-> generic `get_component<T>() -> Option<T>`

### Component has:
- ServerSide
- ClientSide

### Problem 1: client side movement of other players/enemies or general updating:
- A: interpolation between the last and current position data for moving objects
- B: extrapolation by accounting for velocity/acceleration from server
- C: actually running the server code, only needing to update on unpredictable inputs/randomness

### Problem 2: client side input
- A: send to server, server emits global event, do it client side<br>
  -> only input and rendering needed client side, but very laggy
- B: emulate action client side, "shadowing", waiting for server confirmation<br>
  -> less laggy, harder to sync

### Approach
- P1A
- server runs at 20 tps
- sending interpolatable data every 5 frames or so
- only sending data of entities within range
- sending spontaneous events which are more important immediately
- considering grouping data to ease network load 
(use a "ModsData" packet that just groups a few until its full and then a 
second is used, ideally we only ever need one on scheduled updates)