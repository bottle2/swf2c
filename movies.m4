divert(-1)

# ATTENTION:
# You have to download each SWF manually and place them under folder work/
# I will NOT provide instructions on how to do so.
# To compile the PDF and executables with less SWFs,
# prefix the SWFs you don't have with dnl, like so:
M(this is processed)
dnl M(this is excluded)
dnl F(so is this)
M(but processing resumes here)
# m4 is tricky. Don't overthink it.
# Do NOT add single quotes or backtick after dnl, unless there is a # before

# I WON'T automate retrieving views and dates using xmllint from libxml2

# TODO
# - Maybe we could also generate parts of Markdown?
# - I totally forgot about proper attribution!

define(`M',`MI($@)`'dnl')
define(`F',`FI($@)`'dnl')
# define(`T',`TI($@)`'dnl')

# Hand picked from:
# https://www.newgrounds.com/search/conduct/movies?advanced=1&match=tdtu&before=2005-01-01
# https://www.newgrounds.com/search/conduct/movies?advanced=1&match=tdtu&user=Sirkowski&before=2005-01-01&sort=views-desc


define(`MOVIES',`define(`MI',)define(`FI',)define(`SLIDE_BREAK',)$1`'dnl
M(`there_she_is',`There she is!!!',`SamBakZa',`10/09/2004',`NONE',    `OKAY')
M(`dad_s_home',  `Dad\(aqs Home',  `Sakupen', `06/10/2004',`START(3)',`OKAY')
F(`Série Xiao Xiao',`Zhu')
dnl M(`xiao_1',`Xiao Xiao No. 1',`',`01/02/2002')
M(`xiao_3',`Xiao Xiao No. 3',`',`19/04/2001',`START(48)',`OKAY')
M(`xiao_5',`Xiao Xiao No. 5',`',`05/01/2002',`MANUAL',   `OKAY')
M(`xiao_7',`Xiao Xiao No. 7',`',`22/01/2002',`NONE',     `OKAY')
M(`xiao_8',`Xiao Xiao No. 8',`',`16/02/2002',`END(-1)',  `OKAY')
F(`Série Madness Combat',`Krinkels')
M(`marsh_mellow_madness',`Marsh-Mellow-Madness',`',`19/02/2002',`NONE',     `OKAY')
M(`madness_combat',      `Madness Combat',      `',`25/07/2002',`NONE',     `OKAY')
M(`madness_redeemer',    `Madness Redeemer',    `',`15/01/2003',`NONE',     `OKAY')
M(`madness_avenger',     `Madness Avenger',     `',`09/09/2003',`START(80)',`OKAY')
M(`madness_apotheosis',  `Madness Apotheosis',  `',`03/06/2004',`START(67)',`OKAY')
F(`Série Miss Dynamite',`Sirkowski')
M(`miss_dynamite_i_v',      `Miss.Dynamite I to V',   `',`15/07/2001',`START(15)',`OKAY')
`'dnl M(Ep. vi is a game)
`'dnl M(`miss_dynamite_vii',      `Miss.Dynamite VII',      `') doesnt have audio
SLIDE_BREAK`'dnl
M(`miss_dynamite_viii',     `Miss.Dynamite VIII',     `',`26/07/2000',`NONE',           `BADAUDIO')
M(`miss_dynamite_ix',       `Miss.Dynamite IX',       `',`11/08/2000',`NONE',           `BADAUDIO')
M(`miss_dynamite_x',        `Miss.Dynamite X',        `',`30/09/2000',`START(576)',     `BADAUDIO')
M(`miss_dynamite_xi',       `Miss.Dynamite XI',       `',`12/09/2000',`NONE',           `BADAUDIO')
M(`miss_dynamite_xii',      `Miss.Dynamite XII',      `',`21/01/2001',`RANGE(374,2550)',`BADAUDIO')
`'dnl M(`the XIII I dont like torture :(')
M(`miss_dynamite_xiv',      `Miss Dynamite XIV',      `',`24/10/2001',`RANGE(465,-1)',`OKAY')
M(`miss_dynamite_xv',       `Miss.Dynamite XV',       `',`19/08/2003',`START(440)',   `OKAY')
M(`miss_dynamite_halloween',`Miss.Dynamite Halloween',`',`19/10/2000',`NONE',         `OKAY')
`'dnl M(`miss_dynamite_xmas',     `Miss.Dynamite xmas card01',`')
F(`Série Counter-Strike',`Wei Xing')
M(`cs_mansion',`CS_mansion', `',`31/05/2003',`MANUAL',  `OKAY')
M(`cs_assault',`CS_assault', `',`17/09/2003',`START(7)',`OKAY')
M(`de_dust',   `DE_dust',    `',`31/01/2004',`START(7)',`OKAY')
M(`de_aztec',  `DE_aztec',   `',`20/07/2004',`MANUAL',  `OKAY')
F(`Série Brackenwood',`Adam Phillips')
M(`bitey_of_brackenwood', `Bitey of Brackenwood', `',`29/03/2004',`RANGE(6,7491)',  `OKAY')
M(`prowlies_at_the_river',`Prowlies at the River',`',`26/08/2004',`RANGE(13,11082)',`OKAY')
')

# And also include Salad Fingers, but it is too slow
# https://www.newgrounds.com/portal/view/178546
# https://www.newgrounds.com/portal/view/181169

# define(`TAGS',`define(`TI',)$1`'dnl
# ')

define(`AS_360P',`define(`MI',` \
work/$'`1_360.mp4')')
define(`AS_GENERATED_C',`define(`MI',` \
work/$'`1.c')')
define(`AS_JAVASCRIPT',`define(`MI',` \
work/$'`1.js')')
define(`AS_COMPRESSED_MACRO',`define(`MI',` \
work/$'`1.js.gz \
work/$'`1.js.br \
work/$'`1.js.zst')')
define(`AS_COMPRESSED_RECIPE',`define(`MI',`dnl
work/$'`1.js.gz:work/$'`1.js
	gzip -fk $<
	touch $`'@
work/$'`1.js.br:work/$'`1.js
	brotli -f $<
	touch $`'@
work/$'`1.js.zst:work/$'`1.js
	zstd -f19 -no-progress $<
	touch $`'@
')')

define(`AS_INCLUDE',`define(`MI',`#include "$'`1.h"
')')
define(`AS_ONE',`define(`MI',`1')')
define(`AS_INITIALIZER',`define(`MI',`CTOR($'`1	, "$'`2"	),
')')
define(`AS_GERAL',`dnl
`'define(`FI',
.`T'&
l l s s s s s s
a l c n n n n n.
`$'`1'	`$'`2'
)dnl
`'define(`MI',`$'`2	$'`3	$'`4	syscmd(./swf2c -s1 work/$'`1.swf)	syscmd(ffprobe -v error -select_streams v:0 -show_streams work/$'`1_360.mp4 | awk -f hack.awk)
')dnl
define(`SLIDE_BREAK',`.PTITULO NAT
')dnl
')
define(`AS_STATS_1',`define(`MI',`$'`2	syscmd(./swf2c -s2 work/$'`1.swf)
')')
define(`AS_QUALITY',`define(`MI',`$'`2
')')
define(`AS_EFFICIENCY',`define(`MI',`dnl
$'`2	syscmd(./hack.pl work/$'`1)
')dnl
define(`SLIDE_BREAK',`.PTITULO NAT
')dnl
')

divert(0)dnl
ifelse(gen,`m',`dnl
MOVIES_360P =MOVIES(`AS_360P')

MOVIES_GENERATED_C =MOVIES(`AS_GENERATED_C')

MOVIES_JAVASCRIPT =MOVIES(`AS_JAVASCRIPT')

MOVIES_COMPRESSED =MOVIES(`AS_COMPRESSED_MACRO')

MOVIES(`AS_COMPRESSED_RECIPE')
',gen,`h',`dnl
#ifndef MOVIES_H
#define MOVIES_H

enum { N_MOVIE = len(MOVIES(`AS_ONE')) };

extern struct movie
{
    float framerate;
    int n_frame, width, height;
    char *id;
    char *fancy;

    // Currently mutually exclusive.

    #ifdef FEAT_PLUTOVG
    void (*init)(void);
    void (*free)(void);
    void (*render)(void *, int, int);
    #define MOVIE_INIT(C) (C).init()
    #define MOVIE_FREE(C) (C).free()
    #define MOVIE_RENDER(C, ...) (C).render(__VA_ARGS__)
    #endif

    #ifdef FEAT_HTML5
    void (*render)(__externref_t, int, int);
    #define MOVIE_INIT(C) (void)0
    #define MOVIE_FREE(C) (void)0
    #define MOVIE_RENDER(C, ...) (C).render(__VA_ARGS__)
    #endif

} movies[N_MOVIE];

#endif
',gen,`c',`dnl
.pl 1000i
.ll 1000i
.po 0
.nf
MOVIES(`AS_INCLUDE')
#include "movies.h"

#define BASE(ID, NAME) ID##_framerate, ID##_n_frame, ID##_width, ID##_height, NAME

#ifdef FEAT_PLUTOVG
#define CTOR(ID, NAME) { BASE(ID, NAME), ID##_init_plutovg, ID##_free_plutovg, ID##_render_sdl_plutovg }
#endif

#ifdef FEAT_HTML5
#define CTOR(ID, NAME) { BASE(ID, NAME), ID##_render_html5 }
#endif

struct movie movies[] =
{
.in 4
.TS
l0l0l.
MOVIES(`AS_INITIALIZER')dnl
.TE
.in 0
};
.pl 1i
',gen,`geral',`dnl
.TS H EXPAND
expand;
cB0cB lB cB cB l   s  cB
^  ^  ^  ^  ^  cBu s  ^
^  ^  ^  ^  ^  l   s  ^
^  ^  ^  ^  ^  cB  cB ^
l  l  c  n  n  n   n  l.
_
Título	Autor	T{
.ce 2
Lança-
mento
T}	T{
.ce 3
Quadros
por
segundo
T}	Resolução		Duração
					Total de quadros
					_
					Presumido	Aferido
_
.TH
MOVIES(`AS_GERAL')dnl
.TE
',gen,`stats_1',`dnl
.TS H CENTER
center
cB cB cB
l  n  n.
Animação	Versão do Flash	Total de pacotes
_
.TH
MOVIES(`AS_STATS_1')dnl
.TE
',gen,`stats_2',`dnl
',gen,`stats_3',`dnl
',gen,`quality',`dnl
# TODO These are troff tbl tables with some counts and amounts, such as curves, line, PlaceObjectX
.TS H EXPAND
expand;
cB
l.
_
Animação
_
.TH
MOVIES(`AS_QUALITY')
.TE
dnl TODO This is a troff tbl table analyzing PSRN and SSM, this calls swivel.exe and ffmpeg
',gen,`efficiency',`dnl
.TS H EXPAND
expand;
cB0cB s  s  s  s  s  s  s  s  s
^  l  s  s  s  s  s  s  s  s  s
^  lB cB s  s  s  l0 cB s  s  s
^  ^  cB s  s  s  l  cB s  s  s
^  ^  l  s  s  s  l  l  s  s  s
^  ^  cB2cB2cB2cB l  cB2cB2cB2cB
^  ^  cB ^  ^  ^  ^  ^  ^  ^  ^
l  n  n  n  n  n  l  n  n  n  n.
_
Animação	Peso em Megabytes
	_
	T{
.ce 2
SWF
original
T}	Artefatos JavaScript		Vídeos exportados,
		comprimidos, por algoritmo		por resolução
		_		_
		Sem	gzip	Brotli	Zstd		360p	720p	1080p	2160p
		compressão					
_
.TH
MOVIES(`AS_EFFICIENCY')dnl
.TE
dnl TODO This is a troff tbl table analyzing compression
')dnl
