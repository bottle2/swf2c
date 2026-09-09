xx #ifndef LOAD_H
xx #define LOAD_H

xx #include <cairo.h>

xx int load(char *anime, float *framerate,
        int *n_frame, int *w, int *h);
xx void render(int frame, cairo_surface_t *s);
xx void unload(void);
xx extern void (*transform)(double,double,double,
        double,double,double);
xx #endif
