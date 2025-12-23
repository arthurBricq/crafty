# Migration summary: glium (OpenGL) → wgpu (new frontend)

Goal: Replace the `graphics-glium` crate with `graphics-wgpu` while keeping the existing backend-agnostic render data
(`ToDraw`, `CubeRenderData`, `EntityRenderData`, `RectRenderData`) unchanged.

---

## Mental model shift

- **OpenGL/glium**: state machine (set depth/blend/cull → draw)
- **wgpu**: explicit objects
    - state lives in **RenderPipelines** (depth/blend/cull)
    - resources are bound via **BindGroups** (uniform buffers, textures, samplers)
    - per-frame work is recording commands into a **CommandEncoder** + **RenderPass**

---

## Direct mapping (glium → wgpu)

- `Display` / GL context → `Instance + Surface + Adapter + Device + Queue`
- `Program` (GLSL) → `ShaderModule` (WGSL) + `RenderPipeline`
- `VertexBuffer<T>` → `Buffer` (usage: `VERTEX`)
- uniforms (`uniform!{}`) → uniform `Buffer` + `BindGroup`
- `Texture2DArray` / `Texture2D` → `Texture` + `TextureView` + `Sampler`
- `target.draw(...)` → `render_pass.set_* + render_pass.draw(...)`

---

## What you build in `graphics-wgpu`

### Own these core objects
- **Swapchain**: `surface`, `surface_config` (format/size), acquire frame each render
- **Depth texture**: used for 3D passes
- **Shared uniform**: camera matrices buffer + bind group
- **Static meshes**:
    - cube mesh vertex buffer (36 verts)
    - rect mesh vertex buffer (6 verts)
- **Textures**:
    - block textures: 2D **array** texture view + sampler
    - entity textures: 2D **array** texture view + sampler
    - font atlas: 2D texture view + sampler
    - selection overlay: 2D texture view + sampler
- **Pipelines**:
    - `cube_pipeline`
    - `entity_pipeline`
    - `rect_pipeline`
- **Dynamic instance buffers (reused)**:
    - cube instances
    - entity instances
    - rect instances
    - update each frame via `queue.write_buffer(...)`; grow buffer if capacity exceeded

---

## Passes / shader responsibilities (same as today)

### 1) World cubes (instanced)
- Inputs:
    - base vertex: `position`, `tex_coords`, `face`
    - instance: `world_matrix`, `block_id`, `is_selected`
    - uniform: `view`, `projection`
- Textures:
    - `block_textures: texture_2d_array`
    - `selected_texture: texture_2d`
- Indexing logic (same behavior):
    - `layer = block_id * 3 + face_offset(side/top/bottom)`
- Pipeline state:
    - depth test ON, blend ON, culling OFF (match current behavior)

### 2) Entities (instanced)
- Inputs:
    - base cube mesh
    - instance: `world_matrix`, `body_part_id`, `monster_type`
- Texture array indexing:
    - `layer = face + body_part_id * 6 + monster_type * 24`
- Pipeline state:
    - same as cubes

### 3) UI rectangles (instanced)
- Inputs:
    - base rect mesh in NDC + uv
    - instance: `transformation`, `color`, `is_font`, `font_coords`, `block_id`
- Fragment modes:
    - if `is_font != 0`: sample font atlas using `font_coords + font_offsets * uv`
    - else if `block_id >= 0`: sample block texture array
    - else: solid `color`
- Pipeline state:
    - depth OFF, blend ON

---

## Gotchas (don’t skip)
- **Clip-space Z**: WebGPU depth is **0..1** (not OpenGL -1..1) → update your projection matrix.
- **Layouts matter**: WGSL/uniform alignment rules are strict. Prefer:
    - camera data in uniform buffer
    - per-instance matrices/IDs in instance vertex buffers (common + simpler)
- A `mat4` in an instance buffer typically becomes **4 vec4 attributes**.

---

## Suggested milestones
1. Render 1 textured cube (no instancing)
2. Add camera uniform (view + projection)
3. Add cube instancing (instance buffer)
4. Add block texture array + face indexing
5. Add entity pipeline + entity texture array indexing
6. Add UI pipeline (rect instancing + font atlas)

Done = same `ToDraw` inputs, same 3 passes/features, instance buffers reused per frame, texture arrays preserved.
