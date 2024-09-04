# Personal Notes

## OpenGL Optimization 

- Without optimization                                : 282240 cubes rendered, fps = 12
- With the first optimization (only insides of chunks): 155232 _____ ________, fps = 21
- With the second optimization (including the borders):  50111 _____ ________, fps = 38
- After removing the floor of the map                 :  33572 _____ ________, fps = 42

So we need to do more work, for sure...

Interesting result. Hiding the cubes behind the camera decreases the performances... Crazy, right ?

Displaying only the chunks around the player helped a lot with performances too ! And Arnaud's work as well. I think that for now (August 26th), we are good.

## Improving the collision algorithm

Here are some insights that i found on the internet

- It is easier to deal with each axis separatly instead of doing three axis at once
- If a collision occurs, you have to fix it

Waiting on Johan.

## Server, Proxy 

### About using callbacks

Using callbacks seems like it is almost impossible.

Why would I want a callback ?
- So that the PROXY can call directly the correct method inside the WORLD RENDERER.

Why is it difficult ?
- Because it would require the PROXY to have mutable access to the WORLD RENDERER.
- Even if I use interior mutability and give the proxy with an immutable access, it is difficult. WHY? 
- Because I am not sure of how I will implement the 'network scanner' that will wait for messages of the server. So maybe I would do a lot of extra work for nothing ?

### Server TODO list

Example provided by https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo

## Entities

What is an entity ? 
- ID
- Name
- Position + Orientation
- 3D rendering description

### Players

When a player moves, the server must forward to the other players its position.


