# Crafty

(The begining of) A MineCraft clone coded with OpenGL, fully in Rust !

Note that the entire **game engine** is written by ourselves, using **only a driver to OpenGL** !

![](demo/first_chunks.png)

### Current list of features

- Rendering textured cubes of different kinds with OpenGL
  - OpenGL instancing allows to render thousands of cubes very fast.
- First Player Camera
- Detection of collision
- World is divided in chunks of equal sizes

### Missing features (short term milestones)

The first milestones should be implemented roughly in the following order.

- Remove cubes that are not visible to improve the FPS with a lot of cubes
    - Easy empirical rules (easy to implement)
        - cubes behind the player
        - cubes that are not touching the air

- Map format implementation
    - the map is divided by xy-chunks

- Realistic first camera player
    - Smooth motion
    - Supports gravity
    - Be blocked when a cube is in front of you
    - Support 'gravity' and 'flying' modes

- Multiplayer architecture design & Multiplayer implementation through server / client
    - The server holds the full map and sends the chunk to the players as they move
    - Action model
    - Drawing players differently

- Edition of the map by the player: add cubes and delete cubes

- Automatic map creation

- Infinite map creation
