# Happy Harvest for HackNC 2022

We decided to use what we learned from HackNC 2021 to create the tile farming game in 3D!
It is created using pure Bevy which is game framework for Rust.

Link to the Devpost:

- https://devpost.com/software/big-farm-tycoon

## Demo
 This is the demo of the project:

- https://www.youtube.com/watch?v=zS8GXF_Usrw

[![Demo](https://img.youtube.com/vi/zS8GXF_Usrw/0.jpg)](https://www.youtube.com/watch?v=zS8GXF_Usrw)

## Features
- Moving the camera around the **hex map** using the arrow keys
- Moving and rotating 3D Blender Models around the **hex map**
  - Plower model
  - Planter model
  - Each model can be commanded to move to a tile to perform their action
- Different crop phases
  - Premature
  - Ripe
  - Overripe
- A money system
  - Used for buying seeds
- A yield system 
  - You will get different amounts of money depending at what phase you harvest at
