xx #ifndef THERE_SHE_IS_H
xx #define THERE_SHE_IS_H

xx #define there_she_is_framerate 12
xx enum {
xx     there_she_is_n_frame = 2548,
xx     there_she_is_width   = 550,
xx     there_she_is_height  = 281
xx };

xx #ifdef FEAT_CAIRO
xx #include <cairo.h>

xx void there_she_is_render_cairo(cairo *cr, int frame);
xx #endif

xx #ifdef FEAT_PLUTOVG
xx void there_she_is_init_plutovg(void);
xx void there_she_is_free_plutovg(void);
xx void there_she_is_render_sdl_plutovg(
xx         void *pixels, int pitch, int frame);
xx #endif

xx #ifdef FEAT_HTML5
xx void there_she_is_render_html5(
xx         __externref_t CanvasRenderingContext2D,
xx         int frame);
xx #endif

xx #ifndef FEAT_NO_DATA
xx void there_she_is_data(
xx         float *framerate, int *n_frame,
xx         int *width, int *height);
xx #endif

xx #endif
