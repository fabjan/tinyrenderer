## Rust tinyrenderer

This was me learning some Rust while having fun following the first few lessons
on the awesome [tinyrenderer wiki] by [ssloy].

### Building

(tested with Rust 1.40.0)

```shell
$ cargo build --release
```

### Running

You will need a model OBJ file and a diffuse texture map TGA file. You can for
example use the obj and diffuse textures from [boggie] in [ssloy/tinyrenderer].

```shell
$ target/release/tinyrenderer boggie_body.obj boggie_body_diffuse.tga > boggie.pbm
```

[tinyrenderer wiki]: https://github.com/ssloy/tinyrenderer/wiki
[ssloy]: https://github.com/ssloy
[boggie]: https://github.com/ssloy/tinyrenderer/tree/master/obj/boggie
[ssloy/tinyrenderer]: https://github.com/ssloy/tinyrenderer
