divert(-1)

# ATTENTION:
# You have to download each SWF manually.
# I will NOT provide instructions on how to do so.
# To compile the PDF and executables with less SWFs,
# prefix the SWFs you don't have with dnl, like so:
M(this is processed)
dnl M(this is excluded)
dnl F(so is this)
M(but processing resumes here)
# m4 is tricky. Don't overthink it.

# We WON'T automate retrieving views and dates using xmllint from libxml2

# TODO
# - Maybe we could also generate parts of Markdown?
# - I totally forgot about proper attribution!

define(`M',`MI($@)`'dnl')
define(`F',`FI($@)`'dnl')

# Hand picked from:
# https://www.newgrounds.com/search/conduct/movies?advanced=1&match=tdtu&before=2005-01-01
# https://www.newgrounds.com/search/conduct/movies?advanced=1&match=tdtu&user=Sirkowski&before=2005-01-01&sort=views-desc

define(`MOVIES',`define(`MI',)define(`FI',)$1`'dnl
M(`there_she_is',`There she is!!!',`SamBakZa',`10/09/2004')
M(`dad_s_home',  `Dad@aq@s Home',  `Sakupen', `06/10/2004')
F(
.T&
l l s s s s
a l c n n n.
Série Xiao Xiao	Zhu)
dnl M(`xiao_1',`Xiao Xiao No. 1',`',`01/02/2002')
M(`xiao_3',`Xiao Xiao No. 3',`',`19/04/2001')
M(`xiao_5',`Xiao Xiao No. 5',`',`05/01/2002')
M(`xiao_7',`Xiao Xiao No. 7',`',`22/01/2002')
M(`xiao_8',`Xiao Xiao No. 8',`',`16/02/2002')
F(
.T&
l l s s s s
a l c n n n.
Série Madness Combat	Krinkels)
M(`marsh_mellow_madness',`Marsh-Mellow-Madness',`',`19/02/2002')
M(`madness_combat',      `Madness Combat',      `',`25/07/2002')
M(`madness_redeemer',    `Madness Redeemer',    `',`15/01/2003')
M(`madness_avenger',     `Madness Avenger',     `',`09/09/2003')
M(`madness_apotheosis',  `Madness Apotheosis',  `',`03/06/2004')
F(
.T&
l l c s s s
a l c n n n.
Série Miss Dynamite	Sirkowski)
M(`miss_dynamite_i_v',      `Miss.Dynamite I to V'   ,`',`15/07/2001')
M(`miss_dynamite_xiv',      `Miss Dynamite XIV',      `',`24/10/2001')
M(`miss_dynamite_x',        `Miss.Dynamite X',        `',`30/09/2000')
M(`miss_dynamite_viii',     `Miss.Dynamite VIII',     `',`26/07/2000')
M(`miss_dynamite_xi',       `Miss.Dynamite XI',       `',`12/09/2000')
M(`miss_dynamite_xii',      `Miss.Dynamite XII',      `',`21/01/2001')
M(`miss_dynamite_ix',       `Miss.Dynamite IX',       `',`11/08/2000')
M(`miss_dynamite_xv',       `Miss.Dynamite XV',       `',`19/08/2003')
M(`miss_dynamite_halloween',`Miss.Dynamite Halloween',`',`19/10/2000')
dnl M(`the XIII I dont like torture :(')
dnl M(`miss_dynamite_vii',      `Miss.Dynamite VII',      `')
dnl M(`miss_dynamite_xmas',     `Miss.Dynamite xmas card01',`')
F(
.T&
l l c s s s
a l c n n n.
Série Counter-Strike	Wei Xing)
M(`cs_mansion',`CS_mansion', `',`31/05/2003')
M(`cs_assault',`CS_assault', `',`17/09/2003')
M(`de_dust',   `DE_dust',    `',`31/01/2004')
M(`de_aztec',  `DE_aztec',   `',`20/07/2004')
F(
.T&
l l c s s s
a l c n n n.
Série Brackenwood	Adam Phillips)
M(`bitey_of_brackenwood', `Bitey of Brackenwood', `',`29/03/2004')
M(`prowlies_at_the_river',`Prowlies at the River',`',`26/08/2004')
')

# And also include Salad Fingers, but it is too slow
# https://www.newgrounds.com/portal/view/178546
# https://www.newgrounds.com/portal/view/181169

define(`AS_INCLUDE',`define(`MI',`#include "$'`1.h"
')')
define(`AS_ONE',`define(`MI',`1')')
define(`AS_PRE_INITIALIZER',`define(`MI',`"    CTOR($'`1@co@ \"$'`2\")@co@" ')')
define(`AS_GERAL',`dnl
`'define(`FI',`$'`*
')dnl
`'define(`MI',`$'`2	$'`3	$'`4
')dnl
')

define(`divertif',`divert(ifelse(gen,$1,0,-1))dnl')

divertif(`m')
# TODO This is a Makefile fragment for inclusion.
divertif(`h')
#ifndef MOVIES_H
#define MOVIES_H

enum { N_MOVIE = len(MOVIES(`AS_ONE')) };

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
divertif(`c')
MOVIES(`AS_INCLUDE')
#include "movies.h"

#define BASE(ID, NAME) ID##_framerate, ID##_n_frame, ID##_width, ID##_height, NAME

#ifdef FEAT_PLUTOVG
#define CTOR(ID, NAME) { BASE(ID, NAME), ID##_init_plutovg, ID##_free_plutovg, ID##_render_sdl_plutovg }
#endif

#ifdef FEAT_HTML5
#define CTOR(ID, NAME) { BASE(ID, NAME), ID##_render_html5 }
#endif

struct movie movies[] = {
`syscmd(printf "%s\n" MOVIES(`AS_PRE_INITIALIZER') | ./hack)'};
divertif(`t_geral')
.TS H EXPAND
expand;
cB 2 cB 2 cB 2 cB 2 cB 2 cB
^ ^ cB cB cB ^
l l c n n n.
_
Título	Autor	Lança-	Quadros por	Total de	Duração
		mento	segundo	quadros
_
.TH
MOVIES(`AS_GERAL')dnl
.TE
divertif(`t_stats_1')
divertif(`t_stats_2')
divertif(`t_stats_3')
# TODO These are troff tbl tables with some counts and amounts, such as curves, line, PlaceObjectX
divertif(`t_quality')
# TODO This is a troff tbl table analyzing PSRN and SSM, this calls swivel.exe and ffmpeg
divertif(`t_efficiency')
# TODO This is a troff tbl table analyzing compression
