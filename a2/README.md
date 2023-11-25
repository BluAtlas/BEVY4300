# A2

## Program Specification

In this assignment you will be writing the game that was presented in class. This game must have the following features:

### Player:

- [x] The player is represented by a shape which is defined in the config file The player must spawn in the center of the screen at the beginning of the game, and after it dies (collides with an enemy)
- [x] The player moves by a speed read from the config file in these directions: Up: W key, Left: A key, Down: S key, Right: D key
- [x] The player is confined to move only within the bounds of the window
- [x] The player will shoot a bullet toward the mouse pointer when the left mouse button is clicked. The speed, size, and lifespan of the bullets are read from the config file.

### Enemy (s):

- [x] Enemies will spawn in a random location on the screen every X frames, where X is defined in the configuration file.
- [x] Enemies must not overlap the sides of the screen at the time of spawn.
- [x] Enemies shapes have random number of vertices, between a given minimum and maximum number, which is specified in the config file.
- [x] Enemy shape radius will be specified in the configuration file.
- [x] Enemies will be given a random color upon spawning.
- [x] Enemies will be given a random speed upon spawning, between a minimum and maximum value specified in the config file.
- [x] When an enemy reaches the edge of the window, it should bounce off in the opposite direction at the same speed.
- [x] When (large) enemies collide with a bullet or player, they are destroyed, and N small enemies spawn in its place, where N is the number of vertices of the original enemy. Each small enemy must have the same number of vertices and color of the original enemy. These small entities travel outward at angles at a fixed intervals equal to (360 / vertices). For example, if the original enemy had 6 sides, the 6 smaller enemies will travel outward in intervals of (360/6) = 60 degrees. The smaller enemies must have a radius equal to half of the original entity.

### Drawing:

- [x] In the render system, all entities should be given a slow rotation, which makes the game look a little nicer.
- [ ] Any special effects which do not alter game play can be added for up to 5% bonus marks on the assignment. Note that assignments cannot go above 100% total marks, but the 5% bonus can overwrite any marks lost in other areas of the assignment.
- [x] Any Entity with a lifespan is currently alive, it should have its Color alpha channel set to a ratio depending on how long it has left to live. For example, if an Entity has a 100 frame life span, and it has been alive for 50 frames, its alpha value should be set to 0.5 * 255. The alpha should go from 255 when it is first spawned, to 0 on the last frame it is alive.

### Score:
- [x] Each time an enemy spawns, it is given a score component of N*100, where N is the number of vertices it has. Small enemies get double this value.
- [x] If a player bullet kills an enemy, the game score is increased by the score component of the enemy killed.
- [ ] The score should be displayed with the font specified by the config file in the top-left corner of the screen.

### Special Ability:

You are free to come up with your own 'special move' which is fired by the player when the right mouse button is clicked.  
This special ability must:

- [ ] Multiple entities (bullets etc) spawned by special weapon
- [ ] Entities have some unique graphic associate with them
- [ ] A unique game mechanic is introduced via a new component
- [ ] A 'cooldown timer' must be implemented for the special weapon The properties of the special move are not in the config file.

### Misc:
- [ ] The 'P' key should pause the game
- [x] The 'ESC' key should close the game



### Configuration File:

The configuration file will have one line each specifying the window size, font format, player, bullet specification, and enemy specifications.
Lines will be given in that order, with the following syntax:

`Window W H FL FS`
- This line declares that the SFML Window must be constructed with width W
and height H, each of which will be integers. FL is the frame limit that the window should be set to, and FS will be an integer which specifies whether to display the application in full-screen mode (1) or not (0).

`Font F S R G B`
- This lines defines the font which is to be used to draw text
for this program. The format of the line is as follows:
    - `F` - Font File - String
    - `S` - Font Size - Int
    - `R G B` - RGB Color - int, int, int


`Player SR CR S FR FG FB OR OG OB OT V`
- `SR` - Shape Radius - int
- `CR` - Collision Radius - int
- `S` - Speed - float
- `FR, FG, FB` -  Fill Color - int, int, int 
- `OR, OG, OB` - Outline Color - int, int, int
- `OT` - Outline Thickness - int
- `V` - Shape Vertices - int

`Enemy SR CR SMIN SMAX OR OG OB OT VMIN VMAX L SI`
- `SR` - Shape Radius - int
- `CR` - Collision Radius - int
- `SMIN, SMAX` - Min/Max Speed - float, float
- `OR, OG, OB` - Outline Color - int, int, int
- `OT` - Outline Thickness - int
- `VMIN, VMAX` - Min/Max - Vertices int, int 
- `L` - Small Lifespan - int
- `SP` - Spawn Interval - int

`Bullet SR CRS FR FG FB OR OG OB OT VL`
- `SR` - Shape Radius - int
- `CR` - Collision Radius - int
- `S` - Speed - float
- `FR, FG, FB` - Fill Color - int, int, int
- `OR, OG, OB` - Outline Color - int, int, int
- `OT` - Outline Thickness - int
- `V` - Shape Vertices - int
- `L` - Lifespan - int