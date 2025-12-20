# OpenGL Architecture Summary

## Overview

This project uses OpenGL through the `glium` library to render a Minecraft-like voxel game. The rendering system is
built around three main shader programs that handle different types of geometry: world cubes, entities, and UI
rectangles.

## Architecture

### High-Level Structure

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
│  (model, graphics, network crates)                          │
│  - Creates abstract render data (CubeRenderData, etc.)      │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                  Graphics Abstraction Layer                 │
│  (graphics crate)                                           │
│  - Collects render data into ToDraw struct                  │
│  - Backend-agnostic types                                   │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                  OpenGL Implementation Layer                │
│  (graphics-glium crate)                                     │
│  - Converts abstract types to OpenGL vertex types           │
│  - Manages shader programs, textures, buffers               │
│  - Handles windowing and event loop                         │
└─────────────────────────────────────────────────────────────┘
```

### Key Design Decisions

1. **Abstraction Layer**: The codebase uses abstract render data types (`CubeRenderData`, `EntityRenderData`,
   `RectRenderData`) that are backend-agnostic. This allows the OpenGL-specific code to be isolated in the
   `graphics-glium` crate.

2. **Instancing**: Heavy use of OpenGL instancing to render many cubes/entities efficiently. A single base mesh is
   defined, and instance data (position, rotation, texture ID) is provided per instance.

3. **Texture Arrays**: Uses `sampler2DArray` to pack multiple textures into a single array, reducing texture binding
   overhead.

## Shader Programs

### 1. Cube Shader Program

**Purpose**: Renders world blocks (cubes) with different textures per face.

**Vertex Shader** (`CUBE_VERTEX_SHADER`):

- **Inputs**:
    - `position` (vec3): Vertex position in cube-local space (-0.5 to 0.5)
    - `world_matrix` (mat4): Per-instance transformation matrix (position + rotation)
    - `face` (int): Which face of the cube (0-5: right, front, left, back, top, bottom)
    - `block_id` (int): Which block type to render
    - `is_selected` (int): Whether this cube is selected
    - `tex_coords` (vec2): Texture coordinates for this vertex
- **Uniforms**:
    - `perspective` (mat4): Perspective projection matrix
    - `view` (mat4): Camera view matrix
- **Outputs**: Transforms vertex to clip space, passes through face ID, block ID, selection state, and texture
  coordinates

**Fragment Shader** (`CUBE_FRAGMENT_SHADER`):

- **Inputs**: Face ID, block ID, selection state, texture coordinates
- **Uniforms**:
    - `textures` (sampler2DArray): Array of block textures (3 textures per block: side, top, bottom)
    - `selected_texture` (sampler2D): Overlay texture for selected blocks
    - `selected_intensity` (float): Blend factor for selection overlay
- **Logic**:
    1. Calculates texture array index: `block_id * 3 + face_offset`
    2. Samples appropriate texture based on face (top/bottom use different indices)
    3. Blends with selection overlay if `is_selected != 0`

**Vertex Data**:

- Base cube mesh: 36 vertices (6 faces × 6 vertices per quad)
- Each vertex includes: position, texture coordinates, face ID
- Instance data: world matrix, block_id, is_selected

**Performance**: Uses instancing to render thousands of cubes efficiently. Each cube instance only requires a 4×4 matrix
and two integers.

---

### 2. Entity Shader Program

**Purpose**: Renders humanoid entities (players, monsters) using cube-based body parts.

**Vertex Shader** (`ENTITY_VERTEX_SHADER`):

- **Inputs**:
    - `position` (vec3): Vertex position (same cube mesh as world cubes)
    - `world_matrix` (mat4): Per-instance transformation (includes rotation for body parts)
    - `face` (int): Face ID (0-5)
    - `body_part_id` (int): Which body part (0=head, 1=legs, 2=body, 3=arms)
    - `monster_type` (int): Which entity type (0=player, 1=monster1, 2=monster2)
    - `tex_coords` (vec2): Texture coordinates
- **Uniforms**: Same as cube shader (perspective, view)
- **Outputs**: Transforms to clip space, passes through face, body_part_id, monster_type, tex_coords

**Fragment Shader** (`ENTITY_FRAGMENT_SHADER`):

- **Inputs**: Face ID, body part ID, monster type, texture coordinates
- **Uniforms**:
    - `entity_textures` (sampler2DArray): Array of entity textures
- **Logic**:
    - Calculates texture array index: `face + body_part_id * 6 + monster_type * 4 * 6`
    - Each entity type has 4 body parts × 6 faces = 24 textures
    - Samples the appropriate texture

**Vertex Data**:

- Reuses the same cube mesh as world cubes
- Instance data: world matrix (with rotation), body_part_id, monster_type

**Design Note**: Entities are built from multiple cube instances, each with different transformations and body part IDs.
The head uses full rotation (yaw + pitch), while other parts use yaw-only rotation.

---

### 3. Rectangle Shader Program (UI/HUD)

**Purpose**: Renders 2D UI elements, text, and HUD overlays.

**Vertex Shader** (`RECT_VERTEX_SHADER`):

- **Inputs**:
    - `position` (vec3): Vertex position in NDC space (-1 to 1)
    - `tex_coords` (vec2): Texture coordinates
    - `block_id` (int): Optional block ID for rendering block icons
    - `transformation` (mat4): Per-instance transformation matrix (position, scale)
    - `color` (vec4): Per-instance color
    - `is_font` (int): Whether this rectangle contains text
    - `font_coords` (vec2): Font atlas coordinates (if is_font)
- **Outputs**: Transforms to clip space, passes through all instance data

**Fragment Shader** (`RECT_FRAGMENT_SHADER`):

- **Inputs**: Color, texture coordinates, font coordinates, is_font flag, block_id
- **Uniforms**:
    - `font_atlas` (sampler2D): Texture atlas containing font characters
    - `font_offsets` (vec2): Dimensions of each character in the atlas
    - `textures` (sampler2DArray): Block textures for rendering block icons
- **Logic**:
    1. If `is_font != 0`: Samples from font atlas using `font_coords + font_offsets * tex_coords`
    2. Else if `block_id >= 0`: Samples from block texture array
    3. Else: Uses solid color

**Vertex Data**:

- Base rectangle mesh: 6 vertices (2 triangles forming a quad)
- Each vertex: position, texture coordinates
- Instance data: transformation matrix, color, font/texture flags

**Design Note**: This shader handles three different rendering modes (solid color, font, block icon) in a single
program, reducing shader switching overhead.

---

## Rendering Pipeline

### Frame Rendering Flow

1. **Update Phase** (`backend.update(dt)`):
    - Game logic updates world state
    - Collects render data into `ToDraw` struct:
        - `cubes_buffer`: Vec<CubeRenderData>
        - `entity_buffer`: Vec<EntityRenderData>
        - `hud_buffer`: Vec<RectRenderData>

2. **Conversion Phase**:
    - Converts abstract render data to OpenGL vertex types:
        - `CubeRenderData` → `CubeInstance`
        - `EntityRenderData` → `EntityCube`
        - `RectRenderData` → `RectInstance`

3. **Rendering Phase** (three passes):
    - **Pass 1: World Cubes**
        - Depth testing enabled
        - Alpha blending enabled
        - Backface culling disabled (for transparency)
        - Uses cube shader program
        - Renders all cubes with instancing

    - **Pass 2: Entities**
        - Same depth/blend settings as cubes
        - Uses entity shader program
        - Renders all entity body parts with instancing

    - **Pass 3: UI/HUD**
        - Depth testing disabled
        - Alpha blending enabled (for transparency)
        - Uses rectangle shader program
        - Renders all UI elements with instancing

### Buffer Management

- **Static Buffers**: Base meshes (cube vertices, rectangle vertices) are created once and reused
- **Dynamic Buffers**: Instance data is recreated each frame:
    - Cubes: `VertexBuffer::immutable` (recreated each frame)
    - Entities: `VertexBuffer::immutable` (recreated each frame)
    - UI: `VertexBuffer::dynamic` (can be updated in-place, but currently recreated)

### Texture Management

- **Block Textures**: Single `Texture2DArray` containing all block textures (3 per block)
- **Entity Textures**: Single `Texture2DArray` containing all entity textures (24 per entity type)
- **Font Atlas**: Single `Texture2D` containing font characters
- **Selection Overlay**: Single `Texture2D` for selected block highlighting

## Pros and Cons

### Strengths

1. **Good Abstraction**: Clean separation between game logic and rendering. The abstraction layer allows for potential
   future backends (e.g., wgpu, Vulkan).

2. **Efficient Instancing**: Heavy use of OpenGL instancing reduces draw calls significantly. Thousands of cubes can be
   rendered in a single draw call.

3. **Texture Arrays**: Using `sampler2DArray` reduces texture binding overhead and simplifies texture management.

4. **Unified UI Shader**: The rectangle shader handles multiple rendering modes (solid, font, block icons) in one
   program, reducing shader switching.

5. **Clear Separation of Concerns**: Each shader program has a well-defined purpose, making the code easier to
   understand.

6. **Good Use of Glium**: The `glium` library provides a safe Rust wrapper around OpenGL, preventing many common errors.

### Weaknesses

1. **Per-Frame Buffer Recreation**: Instance buffers are recreated every frame instead of being updated in-place. This
   causes unnecessary allocations and GPU memory churn.

2. **No Frustum Culling**: All cubes are sent to the GPU regardless of visibility. For large worlds, this wastes
   bandwidth and processing.

3. **No Occlusion Culling**: Hidden cubes are still rendered, wasting fill rate.

4. **Limited Batching**: Each render pass draws everything in one call, but there's no optimization for spatially
   coherent data or LOD (Level of Detail).

5. **Fixed Shader Versions**: Uses older GLSL versions (150, 140, 330) which limits modern GPU features.

6. **No Texture Compression**: Textures are loaded as RGBA8 without compression, increasing memory usage.

7. **Synchronous Texture Loading**: Textures are loaded synchronously at startup, blocking the main thread.

8. **No Mipmapping**: Textures don't use mipmaps, which can cause aliasing at distance.

9. **Limited Error Handling**: Many `unwrap()` calls could fail at runtime without graceful handling.

10. **No Shader Caching**: Shaders are compiled from source every time the program runs.

## Suggestions for Improvement

### Performance Optimizations

1. **Buffer Updates Instead of Recreation**:
   ```rust
   // Instead of:
   let buffer = VertexBuffer::immutable(&display, &data)?;
   
   // Use:
   buffer.write(&data);  // Update existing buffer
   ```

   2. **Frustum Culling**:
       - Calculate view frustum planes
       - Cull cubes outside frustum before creating instance data
       - Can reduce GPU work by 50-80% in typical scenes

   3. **Occlusion Culling**:
       - Use hierarchical Z-buffer or GPU-based occlusion queries
       - Skip rendering of hidden cubes

4. **Spatial Data Structures**:
    - Use octree or chunk-based culling
    - Only process visible chunks

5. **Texture Compression**:
    - Use DXT/BC compression formats
    - Reduces memory usage by 4-8x
    - Modern GPUs handle decompression automatically

6. **Mipmapping**:
    - Generate mipmaps for textures
    - Reduces aliasing and improves cache efficiency

7. **Shader Caching**:
    - Cache compiled shaders to disk
    - Load precompiled shaders at startup

### Code Quality Improvements

1. **Error Handling**:
    - Replace `unwrap()` with proper error handling
    - Use `Result` types throughout
    - Provide meaningful error messages

2. **Async Texture Loading**:
    - Load textures asynchronously
    - Show loading screen during texture loading
    - Stream textures as needed

3. **Modern OpenGL Features**:
    - Upgrade to GLSL 330+ or 430+
    - Use uniform buffer objects (UBOs) for shared data
    - Consider compute shaders for culling/processing

4. **Resource Management**:
    - Implement RAII for OpenGL resources
    - Automatic cleanup on drop
    - Resource pooling for frequently allocated objects

5. **Profiling and Debugging**:
    - Add GPU timing queries
    - Profile draw calls and buffer updates
    - Use RenderDoc or similar tools for debugging

### Architecture Improvements

1. **Render Graph**:
    - Implement a render graph system
    - Better organization of render passes
    - Easier to add new effects (shadows, post-processing)

2. **Material System**:
    - Abstract material properties
    - Support for different material types
    - Easier to add new block/entity types

3. **Shader Management**:
    - Centralized shader loading/compilation
    - Shader variants for different configurations
    - Hot-reloading during development

4. **Multi-threading**:
    - Prepare render data on worker threads
    - Upload to GPU on main thread
    - Can improve frame times significantly

5. **Level of Detail (LOD)**:
    - Different mesh complexity at distance
    - Impostors for far-away chunks
    - Reduces geometry complexity

## Conclusion

This is a solid OpenGL implementation for a learning project. The architecture is clean, the use of instancing is
appropriate, and the abstraction layer is well-designed. The main areas for improvement are performance optimizations (
culling, buffer updates) and modern OpenGL features. For a project created over a year ago to learn OpenGL, this
demonstrates good understanding of the fundamentals and provides a solid foundation for further development.
