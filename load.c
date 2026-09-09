#include <assert.h>
#include <string.h>

#include <libloaderapi.h>
#include <WinDef.h>

#include <cairo.h>

typedef void cycle(void);
typedef void renderer(cairo_t *, int);
typedef void data(float *, int *, int *, int *);
typedef void trans(double,double,double,double,double,double);

struct lib {
    HMODULE lib;
    cycle *free;
    renderer *render;
} curr;
int some = 0;

trans *transform;

void unload(void)
{
    curr.free();
    FreeLibrary(curr.lib);
    some = 0;
}

int load(char *anime, float *framerate, int *n_frame, int *w, int *h)
{
    int rt;
    struct lib next;

    #define CK(T,C,V,RES,CODE) \
    T V = C(RES); if (!V) { rt = CODE; goto skip; } else (void)0
    #define CKV(T,...) CK(T,(T),__VA_ARGS__)

    CK(,,next.lib,LoadLibrary(TEXT(anime)),1);

    char name[1000];
    int len;
    {
        char *slash = strrchr(anime, '/');
        if (!slash) slash = anime;
        else slash++;
        char *dot = strchr(slash, '.');
        if (!dot) dot = strchr(slash, '\0') - 1;
        else dot--;
        len = dot - slash + 1;
        if (len > 900)
            return 2;
        memcpy(name, slash, len);
    }

    #define CRINGE(W) \
    GetProcAddress(next.lib,(memcpy(name+len,(W),sizeof (W)),name))
    CKV(cycle *,init,CRINGE("_init_cairo"),3);
    CK(,(cycle *),next.free,CRINGE("_free_cairo"),4);
    CKV(data *,d,CRINGE("_data"),5);
    CK(,(renderer *),next.render,CRINGE("_render_cairo"),6);
    CKV(trans *,trans2,CRINGE("_transform"),7);

    d(framerate,n_frame,w,h);
    init();

    if (some)
        unload();
    some++;
    curr = next;
    transform = trans2;
    return 0;

skip:
    if (next.lib)
        FreeLibrary(next.lib);
    return rt;
}

void render(int frame, cairo_surface_t *s)
{
    if (cairo_surface_status(s) != CAIRO_STATUS_SUCCESS)
        assert(!"");;
    cairo_t *cr = cairo_create(s);
    curr.render(cr,frame);
    if (cairo_surface_status(s) != CAIRO_STATUS_SUCCESS)
        assert(!"");;
    cairo_destroy(cr);
    assert(1 == cairo_surface_get_reference_count(s));
    cairo_surface_destroy(s);
}
