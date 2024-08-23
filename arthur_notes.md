# OpenGL Optimization Notes

## Hiding non-visible cubes

- Without optimization                                : 282240 cubes rendered, fps = 12
- With the first optimization (only insides of chunks): 155232 _____ ________, fps = 21
- With the second optimization (including the borders):  50111 _____ ________, fps = 38
- After removing the floor of the map                 :  33572 _____ ________, fps = 38

So we need to do more work, for sure...

Interesting result. Hiding the cubes behind the camera decreases the performances... Crazy, right ?

# Improving the collision algorithm

Here are some insights that i found on the internet

- It is easier to deal with each axis separatly instead of doing three axis at once
- If a collision occurs, you have to fix it
- 

