## Description
Conway's Game of Life, implemented in the Unix terminal

## Controls
* `q` to quit
* Vim keys `h`, `j`, `k`, `l` to move camera
* Space to toggle pause mode
* `f` to toggle frame advance mode
* `a` reserved to advance frame in frame advance mode, although any key will work.

## Modes
* Run mode: the game will advance automatically
* Pause mode: you can still move, the camera, but the game will not advance
* Frame advance mode: Pressing any key will advance the game

## Arguments
* `--fps`: set the frames per second
* `--frame-time` or `--ft`: set the frame time in milliseconds
* `--char` or `-c`: set which character is used to render living cells
