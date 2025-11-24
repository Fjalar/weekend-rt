# Ray Tracing in One Weekend
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) book series, done in Rust

## Features
- Multi-threaded CPU raytracing of spheres
- Global/indirect illumination
- Multiple materials, image textures, and noise textures
- Only dependencies aside from `std` are for randomness (currently `rand` and `rand_chacha`)
- Serialization of PPM files for render output
- Deserialization of PPM files for texture input

## Usage
Run using `cargo run --release`, choose scene by entering a number when prompted, wait for render to finish, then view the resulting `render.ppm` in the root directory of the project

## Example renders
![Render: many spheres](media/many_spheres.png?raw=true)
![Render: three spheres](media/three_spheres.png?raw=true)
![Render: checkers](media/checkers.png?raw=true)
![Render: earth](media/earth.png?raw=true)
![Render: perlin](media/perlin.png?raw=true)

## Future plans
- More materials, including emitters, volumetrics
- Triangle/quad primitives
- Parsing of some simple 3D model format
- Optimizations (e.g. surface area heuristic for BVH)

## Resources
- [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
- [_How to Build a BVH_](https://jacco.ompf2.com/2022/04/13/how-to-build-a-bvh-part-1-basics/)
- [_Physically Based Rendering: From Theory To Implementation_](https://pbr-book.org/)
