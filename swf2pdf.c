#include <assert.h>
#include <stdio.h>
#include <string.h>

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

    // TODO remove hardcoded function names

    char name[1000];
    int len;
    {
        char *slash = strrchr(argv[1], '/');
        if (!slash) slash = argv[1];
        else slash++;
        char *dot = strchr(slash, '.');
        if (!dot) dot = strchr(slash, '\0') - 1;
        else dot--;
        len = dot - slash + 1;
        if (len > 900)
            return 3;
        memcpy(name, slash, len);
    }

    #define CRINGE_CPY(W) memcpy(name + len, (W), sizeof (W))
    CRINGE_CPY("_data");
    data *d = (data *)DL_EXTRACT(lib, name);
    CRINGE_CPY("_render_cairo");
    render *r = (render *)DL_EXTRACT(lib, name);

    DL_CHECK(d, 4);
    DL_CHECK(r, 5);

    int n_frame;
    int width;
    int height;

    d(NULL, &n_frame, &width, &height);

    int frame = atoi(argv[2]);

    if (frame <= 0 || frame > n_frame)
        return 6;

    cairo_surface_t *pdf = cairo_pdf_surface_create(argv[3], width, height);

    if (cairo_surface_status(pdf) != CAIRO_STATUS_SUCCESS)
        return 7;

    cairo_t *cr = cairo_create(pdf);

    r(cr, frame - 1);

    if (cairo_surface_status(pdf) != CAIRO_STATUS_SUCCESS)
        return 8;

    DL_CLOSE(lib);

    cairo_destroy(cr);
    assert(1 == cairo_surface_get_reference_count(pdf));
    cairo_surface_destroy(pdf);

    return 0;
}
