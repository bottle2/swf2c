divert(-1)

# ATTENTION:
# You have to download each SWF manually.
# To compile the PDF and executables with less SWFs,
# prefix the SWFs you don't have with dnl, like so:
M(this is processed)
dnl M(this is excluded)
dnl T(so is this)
M(but processing resumes here)
# m4 is tricky. Don't overthink it.

# We WON'T automate retrieving views and dates using xmllint from libxml2

# TODO
# - Maybe we could also generate parts of Markdown?
# - I totally forgot about proper attribution!

define(`M',`MI($@)`'dnl')
define(`T',`TI($@)`'dnl')

# Hand picked from:
# https://www.newgrounds.com/search/conduct/movies?advanced=1&match=tdtu&before=2005-01-01
# https://www.newgrounds.com/search/conduct/movies?advanced=1&match=tdtu&user=Sirkowski&before=2005-01-01&sort=views-desc

define(`MOVIES',`define(`MI',)define(`TI',)$1`'dnl
M(`there_she_is',`There she is!!!',`SamBakZa')
M(`dad_s_home',  `Dad@aq@s Home',  `Sakupen')
T(`Zhu')
M(`xiao_3',`Xiao Xiao No. 3',`')
M(`xiao_5',`Xiao Xiao No. 5',`')
M(`xiao_7',`Xiao Xiao No. 7',`')
M(`xiao_8',`Xiao Xiao No. 8',`')
T(`Krinkels')
M(`madness_marsh_mellow',`Marsh-Mellow-Madness',`')
M(`madness_combat',      `Madness Combat',      `')
M(`madness_redeemer',    `Madness Redeemer',    `')
M(`madness_avenger',     `Madness Avenger',     `')
M(`madness_apotheosis',  `Madness Apotheosis',  `')
T(`Sirkowski')
M(`miss_dynamite_i_v',      `Miss.Dynamite I to V'   ,`')
M(`miss_dynamite_xiv',      `Miss Dynamite XIV',      `')
M(`miss_dynamite_x',        `Miss.Dynamite X',        `')
M(`miss_dynamite_viii',     `Miss.Dynamite VIII',     `')
M(`miss_dynamite_xi',       `Miss.Dynamite XI',       `')
M(`miss_dynamite_xii',      `Miss.Dynamite XII',      `')
M(`miss_dynamite_xv',       `Miss.Dynamite XV',       `')
M(`miss_dynamite_halloween',`Miss.Dynamite Halloween',`')
dnl M(`miss_dynamite_vii',      `Miss.Dynamite VII',      `')
dnl M(`miss_dynamite_xmas',     `Miss.Dynamite xmas card01',`')
T(`')
M(`de_dust', `DE_dust', `Wei Xing')
M(`de_aztec',`DE_aztec',`')
')

# Should also include Brackenwood...
# https://www.newgrounds.com/series/brackenwood
# And also include Salad Fingers, but it is too slow
# https://www.newgrounds.com/portal/view/178546
# https://www.newgrounds.com/portal/view/181169

define(`AS_INCLUDE',`define(`MI',`#include "$'`1.h"
')')
define(`AS_SUM_ONE',`define(`MI',`+ 1')')
define(`AS_INITIALIZER',`define(`MI',`    CTOR($'`1, "$'`2"),
')')

define(`param',`ifelse(gen,$1,0,-1)')

divert(param(`m'))dnl
# TODO This is a Makefile fragment for inclusion.
divert(param(`h'))dnl
#ifndef MOVIES_H
#define MOVIES_H

MOVIES(`AS_INCLUDE')
enum { N_MOVIE = eval(MOVIES(`AS_SUM_ONE')) };

extern struct movie
{
    int framerate, n_frame, width, height;
    char *filename;

    // Currently mutually exclusive.

    #ifdef FEAT_PLUTOVG
    void (*init)(void);
    void (*free)(void);
    void (*render)(void *, int, int);
    #define MOVIE_INIT(I) movies[i].init()
    #define MOVIE_FREE(I) movies[i].free()
    #define MOVIE_RENDER(I, ...) movies[i].render(__VA_ARGS__)
    #endif

    #ifdef FEAT_HTML5
    void (*render)(__externref_t, int, int);
    #define MOVIE_INIT(I) (void)0
    #define MOVIE_FREE(I) (void)0
    #define MOVIE_RENDER(I, ...) movies[i].render(__VA_ARGS__)
    #endif

} movies[N_MOVIE];

#endif
divert(param(`c'))dnl
#include "movies.h"

#define BASE(ID, NAME) ID##_framerate, ID##_n_frame, ID##_width, ID##_height, NAME

#ifdef FEAT_PLUTOVG
#define CTOR(ID, NAME) { BASE(ID, NAME), ID##_init_plutovg, ID##_free_plutovg, ID##_render_sdl_plutovg }
#endif

#ifdef FEAT_HTML5
#define CTOR(ID, NAME) { BASE(ID, NAME), ID##_render_html5 }
#endif

struct movie movies[] = {
MOVIES(`AS_INITIALIZER')};
divert(param(`t_geral'))dnl
# TODO This is a troff tbl table with general data such as views, publication dates etc.
divert(param(`t_quality'))dnl
# TODO This is a troff tbl table analyzing PSRN and SSM, this calls swivel.exe and ffmpeg
divert(param(`t_efficiency'))dnl
# TODO This is a troff tbl table analyzing compression
