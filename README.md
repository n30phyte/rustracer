# An implementation of "Ray Tracing in One Weekend" in Rust

A very simple ray tracer implemented in Rust. Initial commit is about 24 hours worth of work, completing the steps outlined in the [book](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

Code quality is decent. Some inspiration to get the code more rust-like (instead of looking like a C++ port) was taken from https://github.com/jorendorff/rust-raytrace. Multithreading works thanks to [rayon](https://docs.rs/rayon/1.3.1/rayon/).

## Result:

This was before the rendering bugs got fixed
![Render](./final.png)

## Personal goals for the project

- [x] Learn rust
- [x] Basic ray tracing
- [x] Basic multithreading
- [ ] Optimization
  - [x] Recursion -> Iteration conversion
  - [ ] Profiling