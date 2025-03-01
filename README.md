# swf2c

<!--See [swf2c(1)](swf2c.1.md) for command invocation,
see [swf2c(7)](swf2c.7.md) for API usage.-->
Usage instructions to come.

Do you want prints? A video? Boring! Check out this LIVE and ONLINE demo instead: https://www.newgrounds.com/projects/games/6419340/preview

The remainder of this document concerns development exclusively.

## News

I post in this thread: https://www.newgrounds.com/bbs/topic/1540199/999

## Supported backends

Windowing libraries:

- [SDL2](https://libsdl.org/) 

Vector graphics libraries:

- [PlutoVG](https://github.com/sammycage/plutovg)
- HTML [CanvasRenderingContext2D](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial)

Vector graphics libraries to come next, because they are available in package managers:

- [Skia](https://chromium.googlesource.com/skia/+/master/experimental/c-api-example/c.md)
- [Cairo](https://www.cairographics.org/samples/)

Vector graphics libraries to come next, because they are so easy to build:

- [NanoVG](https://github.com/memononen/nanovg)

Vector graphics libraries to come next, because they are built-in:

- Direct2D (is it still relevant?)

Other vector graphics libraries I don't care for now:

- [Impeller](https://github.com/flutter/flutter/blob/main/engine/src/flutter/impeller/README.md)
- [Blend2D](https://blend2d.com/)
- [Rive Renderer](https://github.com/rive-app/rive-runtime)

Mysterious vector graphics APIs:

- [OpenVG](https://www.khronos.org/openvg/)

Other windowing libraries I don't care for now:

- [SDL3](https://wiki.libsdl.org/SDL3/FrontPage)
- [SFML](https://www.sfml-dev.org/)
- [Sokol](https://github.com/floooh/sokol)
- [GLFW](https://www.glfw.org/)
- [Freeglut](https://freeglut.sourceforge.net/)
- [Raylib](https://www.raylib.com/)
- [Allegro](https://liballeg.org/)

Maybe a generic API will be provided too.

Libraries that WON'T be used:

- [SDL_gfx](https://sourceforge.net/projects/sdlgfx/)
  - I used it for my first prototype, but it is neither "fast" nor featureful (I have no proof tho)

## Competitors

- SVG (+ CSS or + JavaScript or + SMIL)
- [Rive](https://rive.app/) and its [`.riv`](https://rive.app/docs/runtimes/advanced-topic/format) file format
- [Spine2D](https://esotericsoftware.com/): [runtimes](https://github.com/EsotericSoftware/spine-runtimes/)
- Spriter: [runtimes](https://brashmonkey.com/spriter-runtime-apis/)
- [Dragonbones](https://dragonbones.github.io/en/animation.html): [runtimes](https://dragonbones.github.io/en/download.html#runLibrary)

## Project structure

The entire compiler is one Rust source code [`main.rs`](main.rs) that when compiled and run, generates C code.
Notice my code is terribly hideous, for the following two reasons: 1)
I don't know how to program in Rust, and I won't learn it anytime soon; and 2)
I'm an assiduous adept of eXtreme Go Horse (XGH) development methodology.
Many times I considered employing some [text templating](https://www.arewewebyet.org/topics/templating/)
library to ease my pain, but as my usage of the C preprocessor increased, it became pointless.

File [`main.c`](main.c) is one SDL2 program that makes use of the generated code to render an animation.
It is my demo. It exercises the generated code.

The [`Makefile`](Makefile) probably has bashisms. I don't care for now.

## Performance

Compilation time is a major burden right now.
See https://gcc.gnu.org/pipermail/gcc-help/2025-February/143981.html

Ideas:

- Decrease amount of matrix transformations (but does it matter?)

## Design decisions

### Choice of C

I'm generating C because I know C and I like C. I don't which version of C I'm targeting yet,
but because the generated code is so simple, it should be as low as possible
e.g. C89 and work in broken compilers such as MSVC.

Maybe I could generate assembly instead. The choice of assembly needs to meet the following criteria:

- The assembler must be available in every relevant development environment
- It must generate machine code for every relevant game platform, complying to their ABI, including:
  - WebAssembly/JavaScript
- It must have metaprogramming as powerful as my current use of the C preprocessor
- It must be able to use symbols, types and macro definitions provided by library headers

I could also write an actual compiler, leveraring e.g. LLVM or GCC. But I don't even know how to start, so why bother?

Why not just read the SWF file as-is and embed some existing player into the game?
This is a big deal, because the compiled object is enormous,
while the original SWF is diminute, and the SWF format already takes into account low-power devices.
This challenges the validity of my approach,
because I don't even know if there are any gains or optimization opportunities when hardcoding everything in C.
Currently my only defense is to argue about compressed sizes, such as when tranfering on networks.

### Choice of SWF parser

I'm using [Ruffle](https://ruffle.rs/)'s [SWF parser](https://crates.io/crates/swf).
I chose Rust because it is trendy and it will look good in my portfolio. There is no further rationale.

The following two libraries are written in Haxe:

- https://lib.haxe.org/p/swf/
- https://github.com/HaxeFoundation/format/tree/master/format/swf (apparently used by Swivel, OpenFL and Lime)

The following projects also parse SWF:

- [JPEXS Free Flash Decompiler](https://github.com/jindrapetrik/jpexs-decompiler)
- [Lightspark](https://github.com/lightspark/lightspark)
- [swftools](https://github.com/swftools/swftools/tree/master): it is pretty old, and it is annoying to build
- [Gnash](https://www.gnu.org/software/gnash/): pretty old too
- [ffmpeg](https://git.ffmpeg.org/gitweb/ffmpeg.git/blob/HEAD:/libavformat/swf.h): maybe it parses SWF

## Future directions

- Clipping
- Integration with physics engines such as [Chipmunk2D](https://chipmunk-physics.net/)
- Support for sound
- Some barebones action support (I don't know yet)
- Distribute this compiler
  - Make available in package managers
  - Provide binaries for Windows users
  - Online frontend
  - Integration with IDEs such as Visual Studio
  - As a plugin for Unity, Unreal Engine, Godot etc.
- Weird idea: bypass opaque pointers provided by libraries (I abhor opaque data structures)
- Linear gradient fills
- Radial gradient fills
- Focal gradient fills
- Texture fills
- Stroke styles
- Forced frame interpolation
  - Linear interpolation at first
  - Guess interpolation analyzing how objects transform
- Shape morphing
- Text
- Non-seekable APIs (supposedly with a lower footprint)
- Write tool and API manual
- Create more compilers, such as `svg2c` or `riv2c` or... `riv2swf`
- Be friendly with C++

External efforts:

- Make more multimedia authoring tools export SWF
- Turn SWF into a standardized open format
  - We could bump the version to e.g. 128 and upgrade from there
  - We could add new tags starting with code e.g. 500
  - Have a neutral agency in the likes of IANA to allocate tag codes
  - I saw this idea mentioned in Gnash wiki, something BackLash something

## License

Not decided yet!

## Contact

Get in touch preferably through Newgrounds: https://detergent1.newgrounds.com/

## References

The Gnash wiki is a goldmine, but unfortunately it is offline and can only be browsed through Internet Archive.

- https://web.archive.org/web/20090212212743/http://wiki.gnashdev.org/TimelineControl
- https://web.archive.org/web/20210310120920/https://www.adobe.com/content/dam/acom/en/devnet/pdf/swf-file-format-spec.pdf
- https://open-flash.github.io/mirrors/swf-spec-19.pdf
- https://web.archive.org/web/20080113212114/https://www.drizzle.com/~scottb/gdc/flash-paper.htm
