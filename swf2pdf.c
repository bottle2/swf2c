#include <stdlib.h>
#include <cairo-pdf.h>

#include "load.h"

int main(int argc, char *argv[])
{
    if (argc != 4)
        return 27;

    int n_frame, frame = atoi(argv[2]), w, h;

    for (int rt = load(argv[1], NULL, &n_frame, &w, &h); rt;)
        return rt;

    if (frame <= 0 || frame > n_frame)
        return 28;

    render(frame, cairo_pdf_surface_create(argv[3], w, h));
    unload();

    return 0;
}
