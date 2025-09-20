#ifndef THERE_SHE_IS_H
#define THERE_SHE_IS_H

enum {
    there_she_is_framerate = 12,
    there_she_is_n_frame   = 2548,
    there_she_is_width     = 550,
    there_she_is_height    = 281
};

#ifdef FEAT_PLUTOVG
void there_she_is_init_plutovg(void);
void there_she_is_free_plutovg(void);
void there_she_is_render_sdl_plutovg(void *pixels, int pitch, int frame);
#endif

#ifdef FEAT_HTML5
void there_she_is_render_html5(__externref_t CanvasRenderingContext2D, int frame);
void there_she_is_render_sdl_html5(__externref_t CanvasRenderingContext2D, int frame, void *pixels, int pitch);
#endif

// TODO Discuss rendering cutscenes, pre-rendering sprites, ImageBitmap etc.
// TODO See Web Worker + OffscreenCanvasRenderingContext2D.
// TODO Document pixel format expected.
// TODO Explain how to copy HTML canvas data to e.g. texture.
// TODO Figure out parameter order
// TODO Figure out some naming convention <swf>_{init,free,render}[_<framework>]_<engine>[_<variant>]
// TODO Discuss thread safety and reentrancy

#endif
