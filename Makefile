BASIC_CFLAGS=-std=c18 -Wpedantic -Wall -Wextra #-Og -g3 -fsanitize=address,undefined
CFLAGS=$(BASIC_CFLAGS) -Iplutovg/include -DPLUTOVG_BUILD $$(pkgconf --cflags SDL2)
LDLIBS=$$(pkgconf --libs SDL2)
OBJECT=there_she_is.o
SOURCE=main.c
ARCHIVE=plutovg.a

demo:$(SOURCE) $(OBJECT) $(ARCHIVE)
	$(CC) -DFEAT_PLUTOVG $(CFLAGS) -o $@ $(SOURCE) $(OBJECT) $(ARCHIVE) $(LDLIBS)

there_she_is.o:there_she_is.c
	time $(CC) -ferror-limit=1 -DFEAT_PLUTOVG $(CFLAGS) -O0 -c $<

there_she_is.c:main.rs there_she_is.swf
there_she_is.h:main.rs there_she_is.swf

main.c:there_she_is.h
	touch $@

plutovg:
	git clone https://github.com/sammycage/plutovg.git

plutovg.a:
	pushd plutovg/source && \
	$(CC) -DPLUTOVG_BUILD -Wno-sign-compare -Wno-unused-function -c *.c -I../include && \
	ar r ../../plutovg.a *.o

# Doens't work
#there_she_is.swf:
#	curl https://archive.org/download/flash_There_She_Is/flash_There_She_Is.swf > $@

#EMSCRIPTEN_FLAGS=-Oz -flto -mreference-types
EMSCRIPTEN_FLAGS=-mreference-types

demo.zip:main.c there_she_is.c shell.html
	emcc -std=c18 -DFEAT_HTML5 $(EMSCRIPTEN_FLAGS) main.c there_she_is.c -sALLOW_MEMORY_GROWTH \
		--use-port=sdl2 \
		-o index.html --shell-file=shell.html
	7z a $@ index.{html,js,wasm}

#SOKOL=https://raw.githubusercontent.com/floooh/sokol-samples/d91015d455409f20fc1b376fae1b29e0cce1e9ef
#shell.html:
#	curl $(SOKOL)/webpage/shell.html > $@

clean:
	rm -f plutovg.a there_she_is.o demo.exe demo

#TABELA_TAMANHOS_DATA=there_she_is.swf    there_she_is.c    there_she_is_clang_O0.o    demo.exe    \
#                     there_she_is.swf.gz there_she_is.c.gz there_she_is_clang_O0.o.gz demo.exe.gz \
#                     there_she_is.swf.br there_she_is.c.br there_she_is_clang_O0.o.br demo.exe.br \
#                                                           there_she_is_clang_Oz.o    \
#                                                           there_she_is_clang_Oz.o.gz \
#                                                           there_she_is_clang_Oz.o.br \
#                                                           there_she_is_gcc_O0.o    \
#                                                           there_she_is_gcc_O0.o.gz \
#                                                           there_she_is_gcc_O0.o.br \
#                                                           there_she_is_gcc_Oz.o    \
#                                                           there_she_is_gcc_Oz.o.gz \
#                                                           there_she_is_gcc_Oz.o.br \
#                                                           there_she_is_emcc_O0.o    \
#                                                           there_she_is_emcc_O0.o.gz \
#                                                           there_she_is_emcc_O0.o.br \
#                                                           there_she_is_emcc_Oz.o    \
#                                                           there_she_is_emcc_Oz.o.gz \
#                                                           there_she_is_emcc_Oz.o.br

there_she_is.swf.gz:there_she_is.swf
	gzip -fk $<
there_she_is.c.gz:there_she_is.c
	gzip -fk $<
demo.exe.gz:demo.exe
	gzip -fk $<
#there_she_is.swf.br:there_she_is.swf
	brotli -fkc $< > $@
there_she_is.c.br:there_she_is.c
	brotli -fkc $< > $@
demo.exe.br:demo.exe
	brotli -fkc $< > $@

#there_she_is_clang_O0.o:there_she_is.c
#	clang -O0 -c $< -o $@
#there_she_is_clang_O0.o.gz:there_she_is_clang_O0.o
#there_she_is_clang_O0.o.br:there_she_is_clang_O0.o
#there_she_is_clang_Oz.o:there_she_is.c
#	clang -Oz -c $< -o $@
#there_she_is_clang_Oz.o.gz:there_she_is_clang_Oz.o
#there_she_is_clang_Oz.o.br:there_she_is_clang_Oz.o
#there_she_is_gcc_O0.o:there_she_is.c
#	gcc -O0 -c $< -o $@
#there_she_is_gcc_O0.o.gz:there_she_is_gcc_O0.o
#there_she_is_gcc_O0.o.br:there_she_is_gcc_O0.o
#there_she_is_gcc_Oz.o:there_she_is.c
#	gcc -Oz -c $< -o $@
#there_she_is_gcc_Oz.o.gz:there_she_is_gcc_Oz.o
#there_she_is_gcc_Oz.o.br:there_she_is_gcc_Oz.o
#there_she_is_emcc_O0.o:there_she_is.c
#	emcc -O0 -c $< -o $@
#there_she_is_emcc_O0.o.gz:there_she_is_emcc_O0.o
#there_she_is_emcc_O0.o.br:there_she_is_emcc_O0.o
#there_she_is_emcc_Oz.o:there_she_is.c
#	emcc -Oz -c $< -o $@
#there_she_is_emcc_Oz.o.gz:there_she_is_emcc_Oz.o
#there_she_is_emcc_Oz.o.br:there_she_is_emcc_Oz.o
#
#tabela-tamanhos.txt:tabela-tamanhos.m4 $(TABELA_TAMANHOS_DATA)
#	m4 < $< > $@

#tabela-tamanhos.d:tabela-tamanhos.py
#	python $< d | tr -d '\r' > $@

#include tabela-tamanhos.d
include tamanhos.d

#tabela-tamanhos.txt:tabela-tamanhos.py tabela-tamanhos.d $(TABELA_TAMANHOS_DATA)
#	python $< s | tr -d '\r' > $@

tamanhos.d:tamanhos.m4
	m4 -Dd $< > $@

tamanhos.txt:tamanhos.m4 tamanhos.d $(TAMANHOS_DATA)
	m4 -Dt $< > $@

TCC_DRAFT=-d IsDraft=""
tcc.pdf:tcc.mom om.tmac tamanhos.txt
	soelim $< | preconv -eutf8 | pdfmom $(TCC_DRAFT) -M. -t > $@
#	groff -Kutf8 -t -Tpdf om.tmac $< > $@
	
MOM_PATH=/usr/share/groff/1.23.0/tmac/om.tmac

tcc.patch:
	-diff -u om.tmac.orig om.tmac > $@

om.tmac:
	cp om.tmac.orig $@ && patch < tcc.patch

see:
	mv tcc.pdf ../storage/downloads/tcc.pdf

hello:
	gcc --version

.SUFFIXES: .swf .c .h .gz .br .o

.swf.c:
	cargo run -- -c $< > $@
.swf.h:
	cargo run -- -h $< > $@
