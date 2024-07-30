# Crafty

(The begining of) A MineCraft clone coded with OpenGL, fully in Rust !

### Current list of features
- Rendering textured cubes with OpenGL
- OpenGL instancing allows to render thousands of cubes very fast. It's actually impressive.
- First player camera

### Missing features (short term milestones)

- Remove cubes that are not visible to improve the FPS with a lot of cubes
    - Easy empirical rules 
      - cubes behind the player (but this won't be enough...)
      - cubes too far
      - cubes that are not touching the air (but it still won't be enough)
    - Not so easy ones... (these are just ideas)
      - Instead of drawing cubes, draw faces. Like that, you can always remove 3 faces out of the 6
      - Binary space partioning, in theory it should be able to remove some cubes ? Maybe the effort is too big for the reward.

- Automatic map creation

- Rendering chunk by chunk

- Multiplayer architecture design

- Edition of the map by the player: add cubes, delete cubes, etc...

- Multiplayer implementation through server / client