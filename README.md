# Crafty

(The beginning of) A MineCraft clone coded with OpenGL, fully in Rust !

Note that the entire **game engine** is written by ourselves, using **only a driver to OpenGL** !

![](demo/first_chunks.png)

### Current list of features

- Rendering textured cubes of different kinds with OpenGL
  - OpenGL instancing allows to render thousands of cubes very fast.
- World is divided in chunks of equal sizes
- First Player Camera
- Detection of collision
- Gravity, Free-Fall, Jumping
- Easy way to import textures into the game
- Easy way to presents `Tiles` on the screen (menus)

### Missing features (short term milestones)

The first milestones should be implemented roughly in the following order.

- Remove cubes that are not visible to improve the FPS with a lot of cubes
    - Easy empirical rules (easy to implement)
        - cubes behind the player
        - cubes that are not touching the air

- Client must be able to edit the map
    - Show the currently selected block (done)
    - Deletion of cubes
    - Addition of cubes

- Realistic first camera player
    - Support 'gravity' and 'flying' modes

- Display menus and text
    - This is a huge milestone... At least we must be able to display some things on the screen, like the items.

- Multiplayer architecture design & Multiplayer implementation through server / client
    - The server holds the full map and sends the chunk to the players as they move
    - Action model to edit the map
    - Drawing players differently

- Edition of the map by the player: add cubes and delete cubes

- Automatic map creation

- Infinite map creation
