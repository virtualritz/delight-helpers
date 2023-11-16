# `delight`

<!-- cargo-rdme start -->

Utility functions for working with library version of the
[*3Delight*](https://www.3delight.com) renderer, `lib3delight`.

## Compile- vs. Runtime

The crate builds as-is, with default features.

However, at runtime this crate requires a library/renderer that
implements the resp. C-API to link against. Currently the only
renderer that does is [*3Delight*](https://www.3delight.com/).

## Cargo Features

* `download_lib3delight` -- Fetches the dynamic library version of
  3Delight for Linux, macOS or Windows. This can be used as a fallback, to
  build against, if you do not have the renderer installed on your system.
  But it is an old version of 3Delight and foremost a CI feature.

  It is instead suggested that you download a 3Delight package for your
  platform & install it. This will set the `DELIGHT` environment variable
  that the build script is looking for to find a locally installed library
  to link against.

  This will also install *3Delight Display* which you can render to,
  progressively -- useful for debugging.

  The free version renders with up to 12 cores.

* `link_lib3delight` -- Statically link against `lib3dlight`` during build.

  This requires a 3Delight installation unless `download_lib3delight` is
  set. See also next section.

## Linking Style

The 3Delight dynamic library (`lib3delight`) can either be linked to,
during build, or loaded at runtime.

* By default `lib3deligh` is loaded at runtime. This has several
  advantages:

  1. If you ship your application or library you can ship it without the
     library. It can still run and will print an informative error if the
     library cannot be loaded.

  2. A user can install an updated version of the renderer and stuff will
     ‘just work’.

* Dynamically link against `lib3delight`.

  * The feature is called `link_lib3delight`. You should disable default
    features (they are not needed/used) in this case:

    ```toml
    [dependencies]
    delight-sys = {
        version = "0.8",
        default-features = false,
        features = ["link_lib3delight"]
    }
    ```

  * `lib3delight` becomes a dependency. If it cannot be found by the
    system's dynamic linker at runtime, your lib/app will not load/start.

<!-- cargo-rdme end -->
