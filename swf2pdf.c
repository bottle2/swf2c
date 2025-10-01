#include <assert.h>
#include <stdio.h>

#if defined(FEAT_WINDOWS)
# include <libloaderapi.h>
# include <WinDef.h>
# define DL_LOAD(        ID) LoadLibrary(TEXT(ID))
# define DL_EXTRACT(LIB, ID) GetProcAddress((LIB), (ID))
# define DL_CLOSE(  LIB    ) FreeLibrary(LIB)
# define DL_CHECK(RES, CODE) if (!RES) return (CODE); else (void)0
  typedef HMODULE library;
#else
# error Dynamic function loading interface not implemented
#endif

#include <cairo.h>
#include <cairo-pdf.h>

int main(int argc, char *argv[])
{
    if (argc != 4)
        return 1;

    library lib = DL_LOAD(argv[1]);
    DL_CHECK(lib, 2);

    typedef void render(cairo_t *, int);
    typedef void data(float *, int *, int *, int *);

    data *d = (data *)DL_EXTRACT(lib, "there_she_is_data");
    render *r = (render *)DL_EXTRACT(lib, "there_she_is_render_cairo");

    DL_CHECK(d, 3);
    DL_CHECK(r, 4);

    int n_frame;
    int width;
    int height;

    d(NULL, &n_frame, &width, &height);

    int frame = atoi(argv[2]);

    if (frame <= 0 || frame > n_frame)
        return 5;

    cairo_surface_t *pdf = cairo_pdf_surface_create(argv[3], width, height);

    if (cairo_surface_status(pdf) != CAIRO_STATUS_SUCCESS)
        return 6;

    cairo_t *cr = cairo_create(pdf);

    r(cr, frame - 1);

    DL_CLOSE(lib);

    cairo_destroy(cr);
    assert(1 == cairo_surface_get_reference_count(pdf));
    cairo_surface_destroy(pdf);

    return 0;
}
