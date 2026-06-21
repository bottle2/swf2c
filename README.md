# swf2c

Generate C code that plays Flash animations

Do you want prints? A video? Boring! Check out this LIVE and ONLINE demo of actual _running code_ instead: https://www.newgrounds.com/projects/games/6419340/preview (pro tip: it is outdated)

## Usage

    $ swf2c -c|-h

Example usage:

    $ swf2c -c saw.swf > saw.c
    $ swf2c -h saw.swf > saw.h

<!--See [swf2c(1)](swf2c.1.md) for command invocation,
see [swf2c(7)](swf2c.7.md) for API usage.-->

## License

swf2c is free software released under GNU General Public License version 3 or any later version, see [COPYING](COPYING).

## Contact

Get in touch preferably through Newgrounds: https://detergent1.newgrounds.com/

<!--
## News

I post in this thread: https://www.newgrounds.com/bbs/topic/1540199/999-->

## Backends

Windowing libraries:

- [SDL](https://libsdl.org/)

Vector graphics libraries:

- [Cairo](https://www.cairographics.org/samples/)

I worked with these vector graphics libraries, but pressured to finish my thesis, I temporarily dropped support:

- HTML [CanvasRenderingContext2D](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial)
- [PlutoVG](https://github.com/sammycage/plutovg)

Other vector graphics libraries:

- [Skia](https://chromium.googlesource.com/skia/+/master/experimental/c-api-example/c.md): generally available in package managers
- [NanoVG](https://github.com/memononen/nanovg): easy to build
- [Impeller](https://github.com/flutter/flutter/blob/main/engine/src/flutter/impeller/README.md): I'll try package it myself then interface it
- [Blend2D](https://blend2d.com/): it is C++, but has C API, we can give it a shot
- Direct2D: is it still relevant?
- [Rive Renderer](https://github.com/rive-app/rive-runtime): their Web demos lag a lot
- [OpenVG](https://www.khronos.org/openvg/): what even is this?
- [SDL_gfx](https://sourceforge.net/projects/sdlgfx/): I used it for my first prototype, but it is neither "fast" nor featureful

Windowing libraries I don't care for now:

- [SFML](https://www.sfml-dev.org/)
- [Sokol](https://github.com/floooh/sokol)
- [GLFW](https://www.glfw.org/)
- [Freeglut](https://freeglut.sourceforge.net/)
- [Raylib](https://www.raylib.com/)
- [Allegro](https://liballeg.org/)

I also want to integrate physics engines:

- [Box2d](https://box2d.org/)
- [Chipmunk](https://codeberg.org/slembcke/Chipmunk2D)

And support for sound, but I haven't thought much about it yet.

Maybe a generic API will be provided too.

## Competitors

- Lottie: also take a look at [this](https://learn.microsoft.com/en-us/windows/communitytoolkit/animations/lottie-scenarios/json_codegen)
- SVG (+ CSS or + JavaScript or + SMIL)
- [Rive](https://rive.app/) and its [`.riv`](https://rive.app/docs/runtimes/advanced-topic/format) file format
- [Scaleform GFx](https://www.mobygames.com/group/8075/middleware-scaleform-gfx-sdk/): discontinued
- [GAF](https://gafmedia.com/)
- [Live2D](https://www.live2d.com)
- [GameMaker](https://manual.gamemaker.io/lts/en/Settings/Texture_Information/Non-Bitmap_Sprites.htm#h)
- Whatever this is: https://github.com/colin-i/actionswf

Rigged bones:

- [Spine2D](https://esotericsoftware.com/): [runtimes](https://github.com/EsotericSoftware/spine-runtimes/)
- Spriter: [runtimes](https://brashmonkey.com/spriter-runtime-apis/)
- [Dragonbones](https://dragonbones.github.io/en/animation.html): [runtimes](https://dragonbones.github.io/en/download.html#runLibrary)

## Versioning

Consider this tool as "live at head", things will change drastically as I see fit.

## Development

The entire tool is one atrocious hideous Rust source code [`main.rs`](main.rs) written following the [XGH](https://gohorse.com.br/extreme-go-horse-xgh.html) development methodology.

[JPEXS FFDec](https://github.com/jindrapetrik/jpexs-decompiler) has been tremendously useful.

<!--File [`main.c`](main.c) is one SDL2 program that makes use of the generated code to render an animation.
It is my demo. It exercises the generated code.

The [`Makefile`](Makefile) probably has bashisms. I don't care for now.-->

## Performance

Compilation time of the generated C code is a major burden right now.
See https://gcc.gnu.org/pipermail/gcc-help/2025-February/143981.html

<!--Ideas:

- Decrease amount of matrix transformations (but does it matter?)-->

## Thesis

My bachelor thesis is defended and published: https://repositorio.ufsm.br/handle/1/37590

The files needed to build the PDF are... not committed yet. It is a mess, I need to organize it.

<!--## Design decisions

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
- [ffmpeg](https://git.ffmpeg.org/gitweb/ffmpeg.git/blob/HEAD:/libavformat/swf.h): maybe it parses SWF-->

## Future directions

- Clipping
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
<!--- Integration with physics engines such as [Chipmunk2D](https://chipmunk-physics.net/)-->
<!--- Support for sound-->
<!--- Some barebones action support (I don't know yet)-->
<!--- Distribute this compiler
  - Make available in package managers
  - Provide binaries for Windows users
  - Online frontend
  - Integration with IDEs such as Visual Studio
  - As a plugin for Unity, Unreal Engine, Godot etc.
- Weird idea: bypass opaque pointers provided by libraries (I abhor opaque data structures)-->
<!--- Create more compilers, such as `svg2c` or `riv2c` or... `riv2swf`-->


<!--External efforts:

- Make more multimedia authoring tools export SWF
- Turn SWF into a standardized open format
  - We could bump the version to e.g. 128 and upgrade from there
  - We could add new tags starting with code e.g. 500
  - Have a neutral agency in the likes of IANA to allocate tag codes
  - I saw this idea mentioned in Gnash wiki, something BackLash something-->

<!--## Thesis

TODO: discuss mess and lore of videos

Open JPEXS Free Flash Decompiler as admin.

com.newgrounds.swivel/Swivel

Experimental ActionScript editor doesn't work.

Middle panel, ActionScript source, go to lines 328/329 and click any

Rightmost panel changes specific to `updateOutputSize`

Edit P-code

Remove line range 76-97 (inclusive)

Save on panel.

Save on top bar.

If it borks, just reinstall.

---

$ make proxy

to obtain raw videos, replace ffmpeg.exe from Swivel/ffmpeg/win64/ with proxy.exe, but rename to ffmpeg.exe. do a backup etc.-->

## References

The Gnash wiki is a goldmine, but unfortunately it is offline and can only be browsed through Internet Archive.

- https://web.archive.org/web/20090212212743/http://wiki.gnashdev.org/TimelineControl
- https://web.archive.org/web/20210310120920/https://www.adobe.com/content/dam/acom/en/devnet/pdf/swf-file-format-spec.pdf
- https://open-flash.github.io/mirrors/swf-spec-19.pdf
- https://web.archive.org/web/20080113212114/https://www.drizzle.com/~scottb/gdc/flash-paper.htm
- https://sembiance.com/fileFormatSamples/archive/swf/
- https://open-flash.github.io/
- https://www.loc.gov/preservation/digital/formats/fdd/fdd000629.shtml
