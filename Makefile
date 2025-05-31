BASIC_CFLAGS=-std=c18 -Wpedantic -Wall -Wextra #-Og -g3 -fsanitize=address,undefined
CFLAGS=$(BASIC_CFLAGS) -Iplutovg/include -DPLUTOVG_BUILD $$(pkgconf --cflags SDL2)
LDLIBS=$$(pkgconf --libs SDL2)
OBJECT=there_she_is.o
SOURCE=main.c
ARCHIVE=plutovg.a

# Cronômetro Start/End
CS=echo >> $@.t; date +%s.%N | tr -d '\n' >> $@.t;
CE=&& date  +' %s.%N' | tr -d '\n' >> $@.t

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
#include tamanhos.d

tamanhos.d:tamanhos.m4
	m4 -Dd $< > $@

tamanhos.txt:tamanhos.m4 tamanhos.d #$(TAMANHOS_DATA)
	m4 -Dt $< > $@

TCC_DRAFT=-d IsDraft=""
tcc.pdf:tcc.mom om.tmac tamanhos.txt
	soelim $< | pdfmom -Kutf8 $(TCC_DRAFT) -M. -pt > $@
#	groff -Kutf8 -t -Tpdf om.tmac $< > $@
	
MOM_PATH=/usr/share/groff/1.23.0/tmac/om.tmac

tcc.patch:
	-diff -u om.tmac.orig om.tmac > $@

om.tmac:
	cp om.tmac.orig $@ && patch < tcc.patch

see:
	mv tcc.pdf ../storage/downloads/tcc.pdf

.SUFFIXES: .swf .c .h .gz .br .o

.swf.c:
	$(CS) cargo run -- -c $< > $@ $(CE)
.swf.h:
	cargo run -- -h $< > $@
