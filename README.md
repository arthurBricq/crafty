# Crafty

(The begining of) A MineCraft clone coded with OpenGL, fully in Rust !

### Current list of features

- Rendering textured cubes of different kinds with OpenGL
- OpenGL instancing allows to render thousands of cubes very fast.
- First player camera
- The world is divided in chunks of equal sizes

### Missing features (short term milestones)

The first milestones should be implemented roughly in the following order.

- Remove cubes that are not visible to improve the FPS with a lot of cubes
    - Easy empirical rules (easy to implement)
        - cubes behind the player
        - cubes that are not touching the air
    - Not so easy ones... (these are just ideas)
        - use the normal to not draw insides
        - Instead of drawing cubes, draw faces. Like that, you can always remove 3 faces out of the 6
        - Binary space partitioning, in theory it should be able to remove some cubes ? Maybe the effort is too big
          for the reward.

- Map format implementation
    - the map is divided by xy-chunks

- Realistic first camera player
    - Smooth motion
    - Support 'gravity' and 'flying' modes

- Multiplayer architecture design & Multiplayer implementation through server / client
    - The server holds the full map and sends the chunk to the players as they move
    - Action model
    - Drawing players differently

- Edition of the map by the player: add cubes and delete cubes

- Automatic map creation
