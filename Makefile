.SECONDARY:

#BASIC_CFLAGS=-std=c18 -Wpedantic -Wall -Wextra -Og -g3 -fsanitize=address,undefined
BASIC_CFLAGS=-std=c18 -Wpedantic -Wall -Wextra -Oz -flto
CFLAGS=$(BASIC_CFLAGS) -Iplutovg/include -Iwork -DPLUTOVG_BUILD $$(pkgconf --cflags SDL2)
LDLIBS=$$(pkgconf --libs SDL2)
OBJECT=work/miss_dynamite_xiv.o
SOURCE=main.c
ARCHIVE=plutovg.a

#demo:$(SOURCE) $(OBJECT) $(ARCHIVE)
#	time $(CC) -Iwork -DFEAT_PLUTOVG -DMOVIE=miss_dynamite_xiv $(CFLAGS) \
#		-o $@ $(SOURCE) work/miss_dynamite_xiv.c $(ARCHIVE) $(LDLIBS)

demo:demo.c load.c load.h
	$(CC) -Iwork $(BASIC_FLAGS) -Wl,--stack=16777216 \
		$$(pkg-config --cflags --libs cairo SDL2) \
		-o $@ demo.c load.c

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

TABLES=movies_efficiency.txt movies_geral.txt cronograma.txt \
       bibs_leitura.txt bibs_vector.txt bibs_janelas.txt

#TCC_DRAFT=-d IsDraft=""
tcc.pdf:tcc.mom om.tmac abnt.tmac $(TABLES)
	soelim $< | pdfmom -Kutf8 $(TCC_DRAFT) -M. -pt -mabnt > $@
#	groff -Kutf8 -t -Tpdf om.tmac $< > $@

abnt.pdf:abnt.mom abnt.tmac bib.txt
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

fake:
	touch $(MOVIES_GENERATED_C)
	sleep 1
	touch $(MOVIES_JAVASCRIPT)
	sleep 1
	touch $(MOVIES_COMPRESSED)

movies.a:$(GENERATED_C_TU)
	ar r $@ $(GENERATED_C_TU)

raw:raw.c movies.a $(ARCHIVE)
	$(CC) -DFEAT_PLUTOVG $(BASIC_CFLAGS) -Iplutovg/include -Iwork -DPLUTOVG_BUILD -o $@ raw.c movies.a $(ARCHIVE)

movies.d:movies.m4
	m4 -Dgen=m $< > $@
movies.c:movies.m4
	m4 -Dgen=c $< | nroff -t > $@
movies.h:movies.m4
	m4 -Dgen=h $< > $@
movies_geral.txt:movies.m4 $(MOVIES_360P) hack.awk
	m4 -Dgen=geral $< > $@
movies_stats_1.txt:movies.m4
	m4 -Dgen=stats_1 $< > $@
movies_quality.txt:movies.m4
	m4 -Dgen=quality $< > $@
movies_efficiency.txt:movies.m4 $(MOVIES_COMPRESSED) hack.pl
	m4 -Dgen=efficiency $< > $@

swf2c:main.rs
	cargo b -r
	cp target/release/swf2c ./
	-cp target/debug/swf2c.pdb ./

swf2pdf:swf2pdf.c load.c load.h
	$(CC) $(BASIC_CFLAGS) $$(pkgconf --cflags --libs cairo-pdf) -Wl,--stack=16777216 -o $@ swf2pdf.c load.c

#work/there_she_is.dll:work/there_she_is.c
#work/miss_dynamite_xiv.dll:work/miss_dynamite_xiv.c
#	$(CC) $(DLL_FLAGS) -o $@ -shared $<

image.pdf:swf2pdf work/there_she_is.dll
	./swf2pdf work/there_she_is.dll 2150 $@
image2.pdf:swf2pdf work/miss_dynamite_xiv.dll
	./swf2pdf work/miss_dynamite_xiv.dll 4525 $@

res/juicy.pdf:
	# TODO maybe ffmpeg as well all the way up
	gm convert res/outfile0000001.png -crop +100+0 -crop -100+0 res/juicy1.png
	gm convert res/outfile0000044.png -crop +100+0 -crop -100+0 res/juicy2.png
	gm convert res/outfile0000058.png -crop +100+0 -crop -100+0 res/juicy3.png
	gm convert res/outfile0000079.png -crop +100+0 -crop -100+0 res/juicy4.png
	gm montage -background none -geometry +10+10 -tile 2x res/juicy{1,2,3,4}.png -trim res/juicy_tmp.pdf
	-qpdf res/juicy_tmp.pdf res/juicy.pdf
	rm -f res/juicy_tmp.pdf
	gm convert res/juicy1.png res/juicy1_tmp.pdf
	gm convert res/juicy2.png res/juicy2_tmp.pdf
	gm convert res/juicy3.png res/juicy3_tmp.pdf
	gm convert res/juicy4.png res/juicy4_tmp.pdf
	-qpdf res/juicy1_tmp.pdf res/juicy1.pdf
	-qpdf res/juicy2_tmp.pdf res/juicy2.pdf
	-qpdf res/juicy3_tmp.pdf res/juicy3.pdf
	-qpdf res/juicy4_tmp.pdf res/juicy4.pdf
	rm -f res/juicy1_tmp.pdf res/juicy2_tmp.pdf res/juicy3_tmp.pdf res/juicy4_tmp.pdf

include pictures.d

res/suicide1.pdf:suicide1.pdf
	pdfcrop --margins "-122 -27 -23 -29" suicide1.pdf res/suicide1.pdf
res/suicide2.pdf:suicide2.pdf
	pdfcrop --margins "-108 -27 -23 -82" suicide2.pdf res/suicide2.pdf
res/suicide3.pdf:suicide3.pdf
	pdfcrop --margins "-90 -116 -23 -139" suicide3.pdf res/suicide3.pdf
res/intro2.pdf:res/intro.pdf
	pdfcrop --margins "-550 -147 -855 -163" res/intro.pdf res/intro2.pdf

slides.pdf:slides.mom $(TABLES) bib.txt pictures.txt $(PICTURES)
	soelim $< | pdfmom -Kutf8 -M. -t -mabnt > $@

slides2.pdf:slides2.mom bib2.txt pictures.txt death/gen2.txt death/load2.txt
	soelim $< | pdfmom -Kutf8 -M. -ept -mabnt > $@

pictures.d:pictures.m4
	m4 -Dgen=AS_MAKE_LIST $< > $@ && m4 -Dgen=AS_MAKE_RECIPE $< >> $@
pictures.txt:pictures.m4 swf2c $(PICTURES) hack.sh
	m4 -Dgen=AS_MOM $< > $@

pseudo.txt:col1.txt col2.txt
	paste col1.txt col2.txt > $@

colonize.c:colonize.rl
	ragel -G2 $< -o $@

colonize:colonize.c
	cc -std=c99 -Wpedantic -Wall -Wextra -Wno-string-plus-int -O3 $< -o $@

death/gen2.txt:death/gen.txt
	./colonize death/gen.txt '\f(CB' 0 | tr -d '\r' > $@

death/load2.txt:death/load.h
	./colonize death/load.h '\f(CB' 0 | tr -d '\r' > $@

death/pdf2.txt:death/swf2pdf.c
	./colonize death/swf2pdf.c '\f(CB' 0 | tr -d '\r' > $@

pseudo3.txt:pseudo2.txt colonize
	./colonize pseudo2.txt '\fB' 1 | tr -d '\r' > $@

there2.txt:there_she_is.h colonize
	./colonize there_she_is.h '\f(CB' 0 | tr -d '\r' > $@

usage2.txt:usage.txt colonize
	./colonize usage.txt '\f(CB' 1 | tr -d '\r' > $@

swf2pdf3.txt:swf2pdf2.txt colonize
	./colonize swf2pdf2.txt '\f(CB' 0 | tr -d '\r' > $@

case3.txt:case2.txt colonize
	./colonize case2.txt '\f(CB' 0 | tr -d '\r' > $@

coracao2.c:coracao.c colonize
	./colonize coracao.c '\f(CB' 0 | tr -d '\r' > $@

lata2.c:lata.c colonize
	./colonize lata.c '\f(CB' 0 | tr -d '\r' > $@

tcc2.pdf:tcc2.mom om.tmac abnt.tmac bib2.txt pictures.txt er.txt pseudo3.txt there2.txt usage2.txt swf2pdf3.txt case3.txt coracao2.c lata2.c preproc2.c
	soelim $< | pdfmom -Kutf8 -M. -ept -mabnt > $@

count:
	{ echo .nh ;sed -n '/^.FUCK/,$${p;/^.FUCK OFF/q}' < tcc2.mom; } | nroff -Kutf8 -Tascii | wc
count2:
	{ echo .nh ;sed -n '/^.FUCK2/,$${p;/^.FUCK2 OFF/q}' < tcc2.mom; } | nroff -Kutf8 -Tascii | wc

proxy:proxy.c
	$(CC) -std=c11 -O3 -o $@ $<

.SUFFIXES: .swf .c .h .o .zip .html .js .dll

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

DLL_FLAGS=-std=c99 -ferror-limit=1 \
	  -Wpedantic -Wall -Wextra -Wno-unused-function \
	  $$(pkgconf --cflags --libs cairo) -DFEAT_CAIRO
	  #-g3 -Og -fanalyzer
	  #-g3 -Og -fsanitize=address,undefined

.c.dll:
	$(CC) $(DLL_FLAGS) -o $@ -shared $<
