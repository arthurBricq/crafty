# Roadmap

## Most important items

- Use a tokio-compatible web-server instead of manually implemented http connections between clients and servers.
- Fix the collisions detection algorithm.

## `glium` dependency leak [SOLVED]

glium is leaking into primitives, which should be backend-agnostic. This forces glium into crates that shouldn't depend
on it.

```
     1 │primitives (has glium dependency)
     2 │  ├── graphics (should be backend-agnostic)
     3 │  ├── graphics-glium (OK - can have glium)
     4 │  ├── model (game logic - shouldn't need glium!)
     5 │  └── network (networking - shouldn't need glium!)
```

Root cause: The OpenGL-specific types in `primitives/opengl` use `glium::implement_vertex!` :

1. CubeInstance (opengl/cube_instance.rs) - uses glium::implement_vertex!
2. EntityCube (opengl/entity.rs) - uses glium::implement_vertex!
3. RectInstance (opengl/rectangle.rs) - uses glium::implement_vertex!
4. CubeVertex (opengl/cube.rs) - uses glium::implement_vertex!
5. RectVertex (opengl/rectangle.rs) - uses glium::implement_vertex!

Why they're in primitives:
- Rust's orphan rule prevents implementing glium::Vertex (foreign trait) for types defined in graphics or
graphics-glium (foreign types).
- The types were placed in primitives where glium is available, causing the leak.

Where these types are used

- graphics crate:
    - renderer.rs: ToDraw struct uses CubeInstance, EntityCube, RectInstance
    - Multiple UI modules use RectInstance and GLChar
- model crate:
    - world.rs, cubes_to_draw.rs: use CubeInstance
    - entity/: uses EntityCube
- graphics-glium crate:
    - runtime.rs: uses shader constants and vertex data from primitives::opengl (but this is OK)