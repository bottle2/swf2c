#ifndef LOAD_H
#define LOAD_H

#include <cairo.h>

// Zero means success, otherwise not.
// To load another, just call again WITHOUT unload
// If next movie fails, current movie remains.
int load(char *anime, float *framerate, int *n_frame, int *w, int *h);

void render(int frame, cairo_surface_t *s);

// Call exclusively before program ends, to cleanup.
void unload(void);

extern void (*transform)(double,double,double,double,double,double);

#endif
