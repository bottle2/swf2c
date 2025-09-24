// SPDX-FileCopyrightText: Copyright (c) 2024 Bento Borges Schirmer 
// SPDX-License-Identifier: MIT

#include <stdbool.h>
#include <stdint.h>

#include <SDL.h>

#ifndef MOVIE
#error define object-like macro MOVIE... see file movies.m4
#endif

#define CAT2(A,B) A##B
#define CAT(A,B) CAT2(A,B)
#define STR2(X) #X
#define STR(X) STR2(X)

#include STR(MOVIE.h)

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
# define INIT  init_canvas();
# define END   (void)0
# define CLEAR (void)0
# define SHOW  CAT(MOVIE,_render_html5)(get_canvas(), frame_i);
#elif defined(FEAT_PLUTOVG) 
static SDL_Renderer *renderer;
static SDL_Texture *texture;
# define INIT CAT(MOVIE,_init_plutovg)(); \
    TRY(!(renderer = SDL_CreateRenderer(window, -1, 0))); \
    TRY(!(texture = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STREAMING, CAT(MOVIE,_width), CAT(MOVIE,_height))))
# define CLEAR \
    SDL_SetRenderDrawColor(renderer, 255, 255, 255, 0); \
    SDL_RenderClear(renderer);
# define END CAT(MOVIE,_free_plutovg)(); \
    SDL_DestroyTexture(texture); \
    SDL_DestroyRenderer(renderer)
# define SHOW \
    void *pixels; \
    int pitch; \
    TRY(SDL_LockTexture(texture, NULL, &pixels, &pitch)); \
    CAT(MOVIE,_render_sdl_plutovg)(pixels, pitch, frame_i); \
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
    #define EVAL_FRAME (frame_0 + (SDL_GetTicks64() - start) / framerate)

    static bool is_play = true;
    static int frame_0 = 0;

    for (SDL_Event event; SDL_PollEvent(&event); )
    {
        if (SDL_QUIT == event.type)
        {
	    END;
            SDL_DestroyWindow(window);
            SDL_QuitSubSystem(SDL_INIT_VIDEO | SDL_INIT_EVENTS);
            LOOP_END;
        }
	else if (SDL_KEYDOWN == event.type)
        {
            if (event.key.keysym.sym >= '0' && event.key.keysym.sym <= '9')
            {
                frame_0 = CAT(MOVIE,_n_frame) / 10 * (event.key.keysym.sym - '0');
                start = SDL_GetTicks64();
            }
            else switch (event.key.keysym.sym)
            {
                case ' ': // Fall through.
                case 'p':
                    if ((is_play = !is_play))
                        start = SDL_GetTicks64();
                    else
                        frame_0 = EVAL_FRAME;
                break;
                case '.':
                    if (!is_play && frame_0 < CAT(MOVIE,_n_frame) - 1)
                        frame_0++;
                break;
                case ',':
                    if (!is_play && frame_0 > 0)
                        frame_0--;
                break;
            }
        }
    }

    CLEAR;

    int frame_i = is_play ? EVAL_FRAME : frame_0;
    if (frame_i < 0)
        frame_i = 0;
    if (frame_i >= CAT(MOVIE,_n_frame))
    {
        start = SDL_GetTicks64();
        frame_i = 0;
        frame_0 = 0;
    }

    SHOW;

    #undef EVAL_FRAME
}

int main(int argc, char *argv[])
{
    (void)argc;
    (void)argv;

    framerate = 1000 / (float)CAT(MOVIE,_framerate);

    SDL_SetHint(SDL_HINT_EMSCRIPTEN_ASYNCIFY, "0");

    atexit(SDL_Quit);

    TRY(SDL_InitSubSystem(SDL_INIT_VIDEO | SDL_INIT_EVENTS));

    TRY(!(window = SDL_CreateWindow(
        "There She Is! but in C",
        SDL_WINDOWPOS_UNDEFINED,
        SDL_WINDOWPOS_UNDEFINED,
        CAT(MOVIE,_width), CAT(MOVIE,_height),
        0
    )));
    // TODO Deal with Apple's high-DPI stuff.

    INIT;

    start = SDL_GetTicks64();

    LOOP(iter);

    return 0;
}
