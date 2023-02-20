# ecs networking system

### Entity
- ComponentA
- ComponentB

-> each component is unique by type<br>
-> generic `get_component<T>() -> Option<T>`

### Component has:
- ServerSide
- ClientSide

### client side movement of other players/enemies or general updating:
- A: interpolation between the last and current position data for moving objects
- B: extrapolation by accounting for velocity/acceleration from server
- C: actually running the server code, only needing to update on unpredictable inputs/randomness

### client side input
- A: send to server, server emits global event, do it client side<br>
  -> only input and rendering needed client side, but very laggy
- B: emulate action client side, "shadowing", waiting for server confirmation<br>
  -> less laggy, harder to sync

### Approach
- server runs at 20 tps
- sending interpolatable data every 5 frames or so
- only sending data of entities within range
- sending spontaneous events which are more important immediately
- considering grouping data to ease network load