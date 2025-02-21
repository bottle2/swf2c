// SPDX-FileCopyrightText: Copyright (c) 2024 Bento Borges Schirmer 
// SPDX-License-Identifier: MIT

#include <stdbool.h>
#include <stdint.h>

#include <SDL.h>

#include "there_she_is.h"

#ifdef __EMSCRIPTEN__
#include <emscripten.h>
#define LOOP(ITER) emscripten_set_main_loop((ITER), 0, 1)
#define LOOP_END emscripten_cancel_main_loop(); return
#else
static bool is_playing = true;
#define LOOP(ITER) while (is_playing) (ITER)()
#define LOOP_END is_playing = false; return
#endif

#define TRY_(IT, STR) if ((IT)) { \
    SDL_LogError( \
        SDL_LOG_CATEGORY_ERROR, \
        __FILE__ ":%d: %s\n", __LINE__, (STR) \
    ); exit(EXIT_FAILURE); } else (void)0

#define TRY( IT) TRY_((IT), SDL_GetError())
#define TRYE(IT) TRY_((IT), strerror(errno))
// E as in errno

#ifdef FEAT_HTML5
EM_JS(__externref_t, get_canvas, (), {
    return window.pussy;
});

EM_JS(void, init_canvas, (), {
    // https://developer.mozilla.org/en-US/docs/Web/API/Window#window0
    window.pussy = document.getElementById('canvas').getContext('2d');
});
# define INIT init_canvas();
# define END (void)0
# define CLEAR (void)0
# define SHOW \
    there_she_is_render_html5(get_canvas(), frame_i);
#elif defined(FEAT_PLUTOVG) 
SDL_Renderer *renderer;
SDL_Texture *texture;
# define INIT there_she_is_init_plutovg(); \
    TRY(!(renderer = SDL_CreateRenderer(window, -1, 0))); \
    TRY(!(texture = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STREAMING, there_she_is_width, there_she_is_height)))
# define CLEAR \
    SDL_SetRenderDrawColor(renderer, 255, 255, 255, 0); \
    SDL_RenderClear(renderer);
# define END there_she_is_free_plutovg(); \
    SDL_DestroyTexture(texture); \
    SDL_DestroyRenderer(renderer)
# define SHOW \
    void *pixels; \
    int pitch; \
    TRY(SDL_LockTexture(texture, NULL, &pixels, &pitch)); \
    there_she_is_render_sdl_plutovg(pixels, pitch, frame_i); \
    SDL_UnlockTexture(texture); \
    SDL_RenderCopyF(renderer, texture, NULL, NULL); \
    SDL_RenderPresent(renderer)
#else
# error Define graphics backend as one of -DFEAT_PLUTOVG or -DFEAT_HTML5
#endif

SDL_Window *window;

static float framerate;
static uint64_t start;

static void iter(void)
{
    for (SDL_Event event; SDL_PollEvent(&event); )
    {
        if (SDL_QUIT == event.type)
        {
	    CLEAR;
            SDL_DestroyWindow(window);
            SDL_QuitSubSystem(SDL_INIT_VIDEO | SDL_INIT_EVENTS);
            LOOP_END;
        }
	else if (SDL_KEYDOWN == event.type) switch (event.key.keysym.sym)
        {
            case 'p':
            break;
        }
    }

    CLEAR;

    int frame_i = (SDL_GetTicks64() - start) / framerate;
    if (frame_i < 0)
        frame_i = 0;
    if (frame_i >= there_she_is_n_frame)
    {
        start = SDL_GetTicks64();
        frame_i = 0;
    }

    SHOW;
}

int main(int argc, char *argv[])
{
    (void)argc;
    (void)argv;

    framerate = 1000 / (float)there_she_is_framerate;

    SDL_SetHint(SDL_HINT_EMSCRIPTEN_ASYNCIFY, "0");

    atexit(SDL_Quit);

    TRY(SDL_InitSubSystem(SDL_INIT_VIDEO | SDL_INIT_EVENTS));

    TRY(!(window = SDL_CreateWindow(
        "There She Is! but in C",
        SDL_WINDOWPOS_UNDEFINED,
        SDL_WINDOWPOS_UNDEFINED,
        there_she_is_width, there_she_is_height,
        0
    )));
    // TODO Deal with Apple's high-DPI stuff.

    INIT;

    start = SDL_GetTicks64();

    LOOP(iter);

    return 0;
}
