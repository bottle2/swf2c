#include <SDL.h>

extern int const there_she_is_framerate;
extern int const there_she_is_n_frame;
extern int const there_she_is_width;
extern int const there_she_is_height;

void there_she_is_init(void);
void there_she_is_free(void);
void there_she_is_render(void *pixels, int pitch, int frame);
