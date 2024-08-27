# Crafty

A MineCraft clone coded with OpenGL, fully in Rust !

Note that the entire **game engine** (3D rendering and physics) is written by ourselves, using **only a driver to OpenGL**.

![](demo/ms1.png)

![](demo/first_text.png)


### Current list of features

- Rendering textured cubes of different kinds with OpenGL
  - OpenGL instancing allows to render thousands of cubes very fast.
- First Player Camera
- Detection of collision
- Gravity, Free-Fall, Jumping
- Edition of the world like in minecraft
- Easy way to import textures into the game
- Easy way to presents `Tiles` on the screen (menus, texts)
- Persistent world: loading and Saving the world to the disk
- Randomized map creation

### Missing features (short term milestones)

- Better implementation of the collision detection 

- Automatic & Infinite map creation [Johan]

- Multiplayer architecture design & Multiplayer implementation through server / client over the network

- Entities: display other players

- Entities: add monsters
