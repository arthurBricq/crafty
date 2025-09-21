# Crafty

A MineCraft clone coded from craft, fully in Rust (without any game engine or any multiplayer framework).

Works best under Linux or Windows. With macos, you can't resize the window...

![](demo/demo1.jpg)

That works in multiplayer, and with monsters.

![](demo/demo3.jpg)

And that allows you to craft and to mine !

![](demo/demo2.jpg)

## In action

Here are some `GIF` to show a little the gameplay... (**In multiplayer !**)

*A player moving through different biomes*

![](demo/gif1.gif)

*A player running away from a monster*

![](demo/gif2.gif)

*A player placing blocks while jumping*

![](demo/gif3.gif)

## Current list of features

- **Graphics rendering**
    - We use bindings to **OpenGL** (*glium.rs*) to render all the **cubes**. We are able to render tens-of-thousands of
      cubes using much different optimization.
    - Easy way to import new textures into the game, allowing to easily reproduce minecraft-like landscape.
    - Easy way to presents `Tiles` on the screen (menus) and a custom way to render text. We have implemented **debug
      menu** (`F3`), a **help menu** (`F12`).
- A custom **Game Engine**, featuring
    - **Detection of collision**. This was not a piece of cake. We used **AABB** collision algorithm to solve this
      problem.
    - **Gravity**, Free-Fall, Jumping
    - **Ray tracing** to detect actions with the world and with other players.
- **Minecraft Gameplay**
    - Edition of the world like in minecraft: you can add cubes, collect cubes.
    - A **crafting framework** (press 'E' and you will see for yourself !). Craft a sword to attack your friends.
    - **Items** displayed on the screen, exactly like in Minecraft.
    - An **attack system** with life points for each players.
    - Automatic **monsters** which walk over the world and will try to kill you ! Click `x` when playing to spawn
      monsters, they will fall from the air above you.
- A smart **world creation** system
    - Persistent world: loading and saving the world to the disk by pressing `F10` (singleplayer)
    - Randomized map creation, using **Perlin noise** to have some randomness and with different **biomes** to look
      super beautiful !
    - Different **biomes**
- A **Multiplayer Game** !
    - A **multithreaded TCP server** is in charge of (1) the game logic (map, monsters, etc...) (2) synchronizing all
      players by receiving `MessageToServer` and dispatching `ServerUpdates`
    - An reusable architecture using `Trait` that abstracts to the client (`WorldRenderer`) whether he is in single
      player or in multiplayer.

## Getting started

This project surely still has some bugs and everything, we did mostly over 1 weekend.

But the `main` branch is supposed to run !

### Single Player

To run in **single player**, use:

```console
cargo run --bin crafty --release -- INIT_MODE
```

(*You need to run in release*)

For `INIT_MODE`, you can use

- `--random`: generates a random world using our world creation
- `--flat`  : generates a flat world
- `--disk`  : tries to load the `map.json` on the disk
- If put nothing, will use `--random`

Press `x` to spawn some monsters above you !

### Multi Player

To run in **multiplayer**, first launch a server like this:

```console
cargo run --bin server --release -- --server "YOUR.IP"  --port "PORT" --init random
```

- You need to write your IP address when running the server...
- You don't need to specify which world initializer to use, by default it will be the random one

Then, every client can connect like this:

```console
cargo run --bin client --release -- --server "YOUR.IP" --port "PORT" -- name "UNIQUE_IDENTIFIER"
```

- Each client needs to specify his name, and it needs to be different than the other players.

# Dependencies

The goal of this for-fun side-project was to write a minecraft clone with **as little dependencies** as possible.

The biggest dependency that we use is for a Rust binding of the OpenGL ports called `glium`. This library is
unfortunately not maintained anymore, but hey, our code works with `0.34`.

# Code architecture

The code is divided in 3 libraries and 3 executables

The libraries are

- `model`: defines the model of the game. Itself contains the following modules
    - `game`: model of the game
    - `entity`: model of the entities (players and monsters)
    - `primitves`
    - `collision`: how collisions are handles
    - `server`: model of the server
    - `world`
- `graphics`: defines an implementation of the graphics, using `glium`.
- `network`: defines the network abstraction used for playing with multiple players.
- `primitives`

One major architecture flaw is that OpenGL sepcifics logic is in both `graphics` and `model` crate. The solution that
I have in mind involves creating a new crate `graphics_glium`, that involves `glium` as a dependency, that defines the
OpenGL specificities. Then, `graphics` would define traits (such as "drawable", or things like that) and
`graphics_glium` would implement them. And then model would only depend on `graphics`, meaning that in theory you
could totally replace glium by another library, and the code would still work.

will depend on `graphics_glium` and `model` will depend on `graphics`.

# Roadmap

## Next steps

This project works quite well, but... There is still a lot to do.

### Missing features

- Infinite map creation (*almost there*)

### Problems

- The collision often crashes, and when it happens the consequences are... terrible.
- The multiplayer architecture and implementations should be improved, to support more players. There is a bunch of
  corner cases with multiplayer, most of them are not tracked.
- The UI library is shit, because we have implemented everything ourselves using a dead-library. As a consequence, it
  does not work well with macos and the performances are quite bad. I am not sure to which extent it will be easy to
  create new bindings for another graphics library.

## Crafting recipes (for now)

Stone word:

```
_c_
_c_
_o_
```

where `c` is a cobblestone block, and `o` is an oak log.
