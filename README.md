# Crafty

(The beginning of) A MineCraft clone coded with OpenGL, fully in Rust !

Note that the entire **game engine** is written by ourselves, using **only a driver to OpenGL** !

![](demo/first_text.png)

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
- Possibility to add text to the screen
- Persistent world: loading and Saving the world to the disk

### Missing features (short term milestones)

- Better implementation of the collision detection 

- Tiles library [Arnaud]: 
    - [x] Display text
    - Display menu
    - 'menus' can be added:
        - the 'HELP' menu: shows all the commands
        - the 'DEBUG' mode: shows the FPS on the screen, the number of cubes being rendered, the position, the orientation (like in minecraft) !
        - the 'ITEM' menus: shows all the items of the player

- OpenGl improvements: remove cubes that are not visible to improve the FPS with a lot of cubes
      - cubes behind the player
      - cubes that are not touching the air

- Client must be able to edit the map
    - [x] Show the currently selected block
    - [x] Deletion of cubes
    - Addition of cubes

- Player logic
    - When deleting a cube, the cube goes into the items of the player

- Create the `chunk API` [Arthur]
    - *chunk API* = when the player moves, he asks *the server* to send the new chunks further away from him.

- Automatic & Infinite map creation [Johan]

- Multiplayer architecture design & Multiplayer implementation through server / client
    - The server holds the full map and sends the chunk to the players as they move
    - Action model to edit the map
    - Drawing players differently

- Entities: display other players

- Entities: add monsters
