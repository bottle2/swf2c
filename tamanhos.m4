dnl d is for Makefile dependency
dnl t is for table data
changequote([,])dnl
define(just,there_she_is)dnl
define(junk,j/there_she_is)dnl
ifdef([d], [dnl
TAMANHOS_DATA=\
define(itself, [divert(1)$1\
divert(-1)])dnl
define(build, [divert(1)$1\
divert(2)dnl
$1:$2
	$3
divert(-1)dnl
])dnl
define(time,[])dnl
divert(2)
divert(-1)dnl
])dnl
ifdef([t], [dnl
define(size, [syscmd([python -c "print(f'{$(wc -c<$1) / 2**20:.2f}'.replace('.','\\\&,'), end='')"])])dnl
define(itself, [size($1)])dnl
define(build, [size($1)])dnl
define(time, [syscmd([awk '2==NF {n++; s+=$]2[-$]1[} END {split(sprintf("%.2f", s/n),r,"."); ORS=""; print r[1] "\\&," r[2]}' < $1.t])])dnl
dnl define(time, [syscmd([])]dnl
dnl ])dnl
.TS H CENTER
center;
cB cB cB.
_
Artefato	Tamanho em MiB	Tempo em s
_
.TH
.T&
l nB
l s
a n.
Arquivo SWF original	itself(just().swf)
Vídeos substitutivos da Newgrounds
Com qualidade \0360p	itself(just()_360p.mp4)
Com qualidade \0720p	itself(just()_720p.mp4)
Com qualidade 1080p	itself(just()_1080p.mp4)
.T&
l n n
l s
afC n n.
Código-fonte C gerado	itself(just().c)	time(just().c)
Arquivos objeto
gcc -c	build(junk()_gcc.o,just().c,$(CS) gcc -DFEAT_PLUTOVG -Iplutovg/include -DPLUTOVG_BUILD -c $< -o $@ $(CE))	time(junk()_clang.o)
clang -c	build(junk()_clang.o,just().c,$(CS) clang -DFEAT_PLUTOVG -Iplutovg/include -DPLUTOVG_BUILD -c $< -o $@ $(CE))	time(junk()_gcc.o)
emcc -c	build(junk()_emcc.o,just().c,$(CS) emcc -DFEAT_HTML5 -mreference-types -Oz -flto -c $< -o $@ $(CE))	time(junk()_emcc.o)
dnl .T&
dnl l n n n
dnl l s s s
dnl afC n n n
dnl l n n n.
dnl Código-fonte C gerado	sizes(there_she_is.c)
dnl Arquivos objeto
dnl gcc -Og -g3
dnl gcc -Oz -flto
dnl clang -Oz -flto -c	sizes(there_she_is.o)
dnl emcc -Og -g3 -fsanitize=address,undefined
dnl emcc -Oz -flto
dnl Executável do Windows	sizes(demo.exe)
dnl Executável para Web	size(demo.zip) MiB	2 MiB	3 MiB
_
.T&
lp-2 s.
changequote(,)dnl
\*[FonteEu]
.TE
