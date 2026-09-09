xx #include <stdlib.h>
xx #include <cairo-pdf.h>

xx #include "load.h"

xx int main(int argc, char *argv[])
xx {
xx     if (argc != 4)
xx         return 27;
xx 
xx     int n_frame,
       frame = atoi(argv[2]), w, h;
NN
xx     for (int rt =
                load(argv[1], NULL,
        &n_frame, &w, &h); rt;)
xx         return rt;
xx 
xx     if (frame <= 0 || frame > n_frame)
xx         return 28;
xx 
xx     render(frame,
    cairo_pdf_surface_create(argv[3], w, h));
xx     unload();
xx 
xx     return 0;
xx }
