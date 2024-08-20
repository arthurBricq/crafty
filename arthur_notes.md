# OpenGL Optimization Notes

## Hiding non-visible cubes

- Without optimization                                : 282240 cubes rendered, fps = 12
- With the first optimization (only insides of chunks): 155232 _____ ________, fps = 21
- With the second optimization (including the borders):  50111 _____ ________, fps = 38

So we need to do more work, for sure...