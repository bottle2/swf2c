.SECONDARY:

#BASIC_CFLAGS=-std=c18 -Wpedantic -Wall -Wextra #-Og -g3 -fsanitize=address,undefined
BASIC_CFLAGS=-std=c18 -Wpedantic -Wall -Wextra -Oz -flto
CFLAGS=$(BASIC_CFLAGS) -Iplutovg/include -Iwork -DPLUTOVG_BUILD $$(pkgconf --cflags SDL2)
LDLIBS=$$(pkgconf --libs SDL2)
OBJECT=work/there_she_is.o
SOURCE=main.c
ARCHIVE=plutovg.a

TR=sed 's/@ht@/\t/g;s/@hm@/-/g'

demo:$(SOURCE) $(OBJECT) $(ARCHIVE)
	time $(CC) -DFEAT_PLUTOVG -DMOVIE=there_she_is $(CFLAGS) -o $@ $(SOURCE) $(OBJECT) $(ARCHIVE) $(LDLIBS)

work/there_she_is.o:work/there_she_is.c
	time $(CC) -ferror-limit=1 -DFEAT_PLUTOVG $(CFLAGS) -c $< -o $@

there_she_is.js:work/there_she_is.c
	$(CC) -DFEAT_JAVASCRIPT -E -P $< > $@

#time $(CC) -ferror-limit=1 -DFEAT_PLUTOVG $(CFLAGS) -c -O0 $<

work/there_she_is.c:main.rs work/there_she_is.swf
#there_she_is.h:main.rs there_she_is.swf

plutovg:
	git clone https://github.com/sammycage/plutovg.git

plutovg.a:
	pushd plutovg/source && \
	$(CC) -DPLUTOVG_BUILD -Wno-sign-compare -Wno-unused-function -c *.c -I../include && \
	ar r ../../plutovg.a *.o

#SOKOL=https://raw.githubusercontent.com/floooh/sokol-samples/d91015d455409f20fc1b376fae1b29e0cce1e9ef
#shell.html:
#	curl $(SOKOL)/webpage/shell.html > $@
clean:
	rm -f plutovg.a there_she_is.o demo.exe demo swf2c swf2c.exe \
	work/*.js work/*.gz work/*.c work/*.h work/*.br work/*.zst work/*.o work/*.js work/*.html work/*.wasm

#TCC_DRAFT=-d IsDraft=""
tcc.pdf:tcc.mom om.tmac abnt.tmac movies_geral.txt movies_efficiency.txt
	soelim $< | pdfmom -Kutf8 $(TCC_DRAFT) -M. -pt -mabnt > $@
#	groff -Kutf8 -t -Tpdf om.tmac $< > $@

abnt.pdf:abnt.mom abnt.tmac
	soelim $< | pdfmom -Kutf8 -M. -mabnt > $@

MOM_PATH=/usr/share/groff/1.23.0/tmac/om.tmac

tcc.patch:
	-diff -u om.tmac.orig om.tmac > $@

om.tmac:
	cp om.tmac.orig $@ && patch < tcc.patch

see:abnt.pdf tcc.pdf
	-mv tcc.pdf ../storage/downloads/tcc.pdf
	-mv abnt.pdf ../storage/downloads/abnt.pdf

include movies.d

movies.d:movies.m4
	m4 -Dgen=m $< | nroff -t | $(TR) > $@
movies.c:movies.m4
	m4 -Dgen=c $< | nroff -t > $@
movies.h:movies.m4
	m4 -Dgen=h $< > $@
movies_geral.txt:movies.m4
	m4 -Dgen=geral $< > $@
movies_quality.txt:movies.m4
	m4 -Dgen=quality $< > $@
movies_efficiency.txt:movies.m4 $(MOVIE_COMPRESSED) hack.pl
	m4 -Dgen=efficiency $< > $@

swf2c:main.rs
	cargo b
	cp target/debug/swf2c ./
	-cp target/debug/swf2c.pdb ./

.SUFFIXES: .swf .c .h .o .zip .html .js

.swf.c:
	./swf2c -c $< > $@
	./swf2c -h $< > $*.h
#.swf.h:

EMSCRIPTEN_FLAGS=-Oz -flto

.c.js:
	NODE_OPTIONS=--max-old-space-size-percentage=90 emcc \
	-ferror-limit=1 -std=c18 -mreference-types \
	-Iwork \
	-DFEAT_HTML5 -DMOVIE=$(*F) \
	$(EMSCRIPTEN_FLAGS) \
	main.c $< \
	-sALLOW_MEMORY_GROWTH \
	--use-port=sdl2 \
	-o $*.html \
	--shell-file=shell.html

.js.zip:
	7z a $@ $*.{html,js,wasm}
	7z rn $@ $*.html index.html
