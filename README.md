# Ray Tracer

An experimental Ray Tracer built following *The Ray Tracer Challenge*[^1] in Rust with a few optimizations leveraging multi-threading and caching matrix operations.

## Building

```
cargo build --release
```

## Example Renders

There are a handful of example renders which are built into the ray tracer. List them using `--help`. Some renders can take a long so it's also recommend to specify as many threads as you can space with `--threads`. 

### Glass Pawn [^2]

| `./target/release/ray_tracer --example="pawn"` |
|:-:|
|  ![pawn](./renders/pawn.png) |

### Utah Tea Set [^3][^4]

| `./target/release/ray_tracer --example="tea set"` |
|:-:|
|  ![pawn](./renders/teaset.png) |

### Book Cover [^1]

| `./target/release/ray_tracer --example="cover"` |
|:-:|
|  ![pawn](./renders/cover.png) |

## References

[^1]: [The Ray Tracer Challenge](http://raytracerchallenge.com/)
[^2]: [Beautiful pawn chess](https://www.turbosquid.com/3d-models/beautiful-pawn-chess-3d-model-1550111)
[^3]: [Tea cup](https://www.turbosquid.com/3d-models/cup-saucer-3d-model-1434751)
[^4]: [Utah Tea Pot](https://graphics.cs.utah.edu/courses/cs6620/fall2013/?prj=5)
