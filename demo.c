// SPDX-FileCopyrightText: Copyright (c) 2024 Bento Borges Schirmer 
// SPDX-License-Identifier: MIT

#include <stdbool.h>
#include <stdint.h>

#include <SDL.h>

#include <cairo.h>

#include "load.h"

#define TRY_(IT, STR) if ((IT)) { \
    SDL_LogError( \
        SDL_LOG_CATEGORY_ERROR, \
        __FILE__ ":%d: %s\n", __LINE__, (STR) \
    ); exit(EXIT_FAILURE); } else (void)0

#define TRY( IT) TRY_((IT), SDL_GetError())
#define TRYE(IT) TRY_((IT), strerror(errno))
// E as in errno

static SDL_Renderer *renderer;
static SDL_Texture *texture;

static bool is_playing = true;

SDL_Window *window;

static float framerate;
int n_frame, anime_w, anime_h;
SDL_Rect dim = {0};
static uint64_t start;

static void iter(void)
{
    #define EVAL_FRAME (frame_0 + (SDL_GetTicks64() - start) / framerate)

    static bool is_play = true;
    static int frame_0 = 0;
    static bool unlocked = false;

    for (SDL_Event event; SDL_PollEvent(&event); )
    {
        if (SDL_QUIT == event.type)
        {
            SDL_DestroyTexture(texture);
            SDL_DestroyRenderer(renderer);
            SDL_DestroyWindow(window);
            SDL_QuitSubSystem(SDL_INIT_VIDEO | SDL_INIT_EVENTS);
            is_playing = false; return;
        }
        else if (SDL_WINDOWEVENT == event.type
                && SDL_WINDOWEVENT_SIZE_CHANGED == event.window.event)
        {
            dim.w = event.window.data1;
            dim.h = event.window.data2;
        }
	else if (SDL_KEYDOWN == event.type)
        {
            if (event.key.keysym.sym >= '0' && event.key.keysym.sym <= '9')
            {
                frame_0 = n_frame / 10 * (event.key.keysym.sym - '0');
                start = SDL_GetTicks64();
            }
            else switch (event.key.keysym.sym)
            {
                case '[': // Fall through.
                case ']':
                    // TODO Speed
                break;
                case 'i':
                    SDL_Log("Frame %d\n", frame_0);
                break;
                case 'u':
                    if ((unlocked = !unlocked))
                        frame_0 = EVAL_FRAME;
                    else
                        start = SDL_GetTicks64();
                break;
                case ' ': // Fall through.
                case 'p':
                    if ((is_play = !is_play))
                        start = SDL_GetTicks64();
                    else
                        frame_0 = EVAL_FRAME;
                break;
                case '.':
                    if (!is_play && frame_0 < n_frame - 1)
                        frame_0++;
                break;
                case ',':
                    if (!is_play && frame_0 > 0)
                        frame_0--;
                break;
                case 'f':
                {
                    static bool is_full = false;
                    SDL_SetWindowFullscreen(
                        window,
                        (is_full = !is_full) ? SDL_WINDOW_FULLSCREEN_DESKTOP : 0
                    );
                }
                break;
            }
        }
    }

    SDL_RenderClear(renderer);

    int frame_i = is_play ? (unlocked ? ++frame_0 : EVAL_FRAME) : frame_0;
    if (frame_i < 0)
        frame_i = 0;
    if (frame_i >= n_frame)
    {
        start = SDL_GetTicks64();
        frame_i = 0;
        frame_0 = 0;
    }

    {
        float scale;
        float const anime_ratio = anime_w / (float)anime_h;
        float const screen_ratio = dim.w / (float)dim.h;

        if (anime_ratio > screen_ratio)
            scale = dim.w / (float)anime_w;
        else
            scale = dim.h / (float)anime_h;
        // Choose which dimension to glue with which other.

        float const view_width  = scale * anime_w;
        float const view_height = scale * anime_h;

        float const pan_x = (dim.w - view_width) / 2;
        float const pan_y = (dim.h - view_height) / 2;

        transform(scale,0,0,scale,pan_x,pan_y);
    }

    void *pixels;
    int pitch;
    TRY(SDL_LockTexture(texture, &dim, &pixels, &pitch));
    render(
        frame_i+1,
        cairo_image_surface_create_for_data(
            pixels, CAIRO_FORMAT_ARGB32, dim.w, dim.h, pitch
        )
    );
    SDL_UnlockTexture(texture);
    SDL_RenderCopyF(renderer, texture, &dim, NULL);
    SDL_RenderPresent(renderer);

    #undef EVAL_FRAME
}

int main(int argc, char *argv[])
{
    if (argc != 2)
        return 1;

    for (int rt = load(argv[1], &framerate,&n_frame,&anime_w,&anime_h); rt; )
        return rt;
    dim.w = anime_w;
    dim.h = anime_h;

    framerate = 1000 / framerate;

    SDL_SetHint(SDL_HINT_EMSCRIPTEN_ASYNCIFY, "0");

    atexit(SDL_Quit);

    TRY(SDL_InitSubSystem(SDL_INIT_VIDEO | SDL_INIT_EVENTS));

    TRY(!(window = SDL_CreateWindow(
        argv[1],
        SDL_WINDOWPOS_UNDEFINED,
        SDL_WINDOWPOS_UNDEFINED,
        dim.w, dim.h,
        SDL_WINDOW_RESIZABLE
    )));
    // TODO Deal with Apple's high-DPI stuff.

    TRY(!(renderer = SDL_CreateRenderer(window, -1, 0)));
    TRY(!(texture = SDL_CreateTexture(
        renderer,
        SDL_PIXELFORMAT_ARGB8888,
        SDL_TEXTUREACCESS_STREAMING,
        6000, 6000
    )));
    SDL_SetRenderDrawColor(renderer, 255, 255, 255, 0);

    start = SDL_GetTicks64();

    while (is_playing)
        iter();

    unload();

    return 0;
}
