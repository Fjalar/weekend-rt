# Ray Tracing in One Weekend
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html), done in Rust

## Features
- Raytracing of spheres
- Multiple materials, image textures, and noise textures
- Only dependencies for randomness (currently `rand` and `rand_chacha`)
- Serialization of PPM files for render output
- Deserialization of PPM files for texture input

## Usage
Run using `cargo run --release`, choose scene by entering a number when prompted, wait for render to finish, then view the resulting `render.ppm` in the root directory of the project

## Example renders
![Render: three spheres](media/three_spheres.png?raw=true)
![Render: many spheres](media/many_spheres.png?raw=true)
![Render: checkers](media/checkers.png?raw=true)
![Render: earth](media/earth.png?raw=true)
![Render: perlin](media/perlin.png?raw=true)

## Future plans
- More materials, including volumetrics
- Triangle/quad primitives
- Parsing of some simple 3D model format
- Optimizations (e.g. surface area heuristic for BVH)
