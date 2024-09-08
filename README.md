# Crafty

A multiplayer MineCraft clone coded from craft, fully in Rust (without any game engine or any multiplayer framework)

![](demo/ms1.png)

![](demo/first_text.png)

## Current list of features

- **Graphics rendering**
    - We use bindings to **OpenGL** (*glium.rs*) to render all the **cubes**. We are able to render tens-of-thousands of cubes using much different optimization.
    - Easy way to import new textures into the game, allowing to easily reproduce minecraft-like landscape.
    - Easy way to presents `Tiles` on the screen (menus) and a custom way to render text. We have implemented **debug menu** (`F3`), a **help menu** (`F12`).
- A custom **Game Engine**, featuring
    - **Detection of collision**. This was not a piece of cake. We used **AABB** collision algorithm to solve this problem.
    - **Gravity**, Free-Fall, Jumping
    - **Ray tracing** to detect actions with the world and with other players.
- **Minecraft Gameplay**
    - Edition of the world like in minecraft: you can add cubes, collect cubes.
    - A **crafting framework** (press 'E' and you will see for yourself !). Craft a sword to attack your friends.
    - An **attack system** with lifes.
    - **Items** displayed on the screen, exactly like in Minecraft.
    - Automatic **monsters** which walk over the world and will try to kill you...
- A smart **world creation** system
    - Persistent world: loading and saving the world to the disk by pressing `F10` (singleplayer)
    - Randomized map creation, using **Perlin noise** to have some randomness and with different **biomes** to look super beautiful !

And finally, probably the most advanced feature of this repo: 

- A **Multiplayer Game** ! 
    - A **multithreaded TCP server** is in charge of (1) the game logic (map, monsters, etc...) (2) synchronizing all players by receiving `MessageToServer` and dispatching `ServerUpdates`
    - An reusable architecture using `Trait` that abstracts to the client (`WorldRenderer`) whether he is in single player or in multiplayer.

## Getting started

This project surely still has some bugs and everything, we did mostly over 1 weekend. 

But the `main` branch is supposed to run !

To run in **single player**, use

```console
cargo run --bin crafty
```

To run in **multiplayer**, first launch a server like this:

```console
cargo run --bin server -- --server "YOUR.IP" 
```

Then, every client can connect like this: 

```console
cargo run --bin client -- --server "YOUR.IP" -- name "UNIQUE_IDENTIFIER"
```

## Missing features (short term milestones)

- Infinite map creation (*almost there*)
- Entities: finalize monsters logic

And of course ...

- Stabilization of all the bugs that we find...
