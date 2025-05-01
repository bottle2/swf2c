dnl d is for Makefile dependency
dnl t is for table data
changequote([,])dnl
ifdef([d], [dnl
TAMANHOS_DATA=\
define(itself, [divert(1)there_she_is$1\
divert(-1)])dnl
define(build, [divert(1)there_she_is$1\
divert(2)dnl
there_she_is$1:there_she_is$2
	$3
divert(-1)dnl
])dnl
divert(2)
divert(-1)dnl
])dnl
ifdef([t], [dnl
define(size, [syscmd([python -c "print(f'{$(wc -c<$1) / 2**20:.2f}'.replace('.','\\\&,'), end='')"])])dnl
define(itself, [size(there_she_is$1)])dnl
define(build, [size(there_she_is$1)])dnl
])dnl
.TS H CENTER
center;
cB cB.
_
Artefato	Tamanho em MiB
_
.TH
.T&
l nB
l s
a n.
Arquivo SWF original	itself(.swf)
Vídeos substitutivos da Newgrounds
Com qualidade \0360p	itself(_360p.mp4)
Com qualidade \0720p	itself(_720p.mp4)
Com qualidade 1080p	itself(_1080p.mp4)
.T&
l n
l s
afC n.
Código-fonte C gerado	itself(.c)
Arquivos objeto
gcc -Og -g3 -c	build(_gcc_g.o,.c,gcc -DFEAT_PLUTOVG -Iplutovg/include -DPLUTOVG_BUILD -Og -g3 -c $< -o $@)
gcc -Oz -flto	build(_gcc_z.o,.c,gcc -DFEAT_PLUTOVG -Iplutovg/include -DPLUTOVG_BUILD -Oz -flto -c $< -o $@) 
clang -Oz -flto -c	build(_clang_z.o,.c,clang -DFEAT_PLUTOVG -Iplutovg/include -DPLUTOVG_BUILD -Oz -flto -c $< -o $@)
emcc -Og -g3 \e	build(_emcc_g.o,.c,[emcc -DFEAT_HTML5 -mreference-types -Og -g3 -fsanitize=address,undefined -c $< -o $@])
\ \ \ \ -fsanitize=address,undefined	\^
emcc -Oz -flto	build(_clang_z,.c,emcc -DFEAT_HTML5 -mreference-types -Oz -flto -c $< -o $@)
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
