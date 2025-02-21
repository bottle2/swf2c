BASIC_CFLAGS=-std=c18 -Wpedantic -Wall -Wextra #-Og -g3 -fsanitize=address,undefined
CFLAGS=$(BASIC_CFLAGS) -Iplutovg/include -DPLUTOVG_BUILD $$(pkgconf --cflags SDL2)
LDLIBS=$$(pkgconf --libs SDL2)
OBJECT=there_she_is.o
SOURCE=main.c
ARCHIVE=plutovg.a

demo:$(SOURCE) $(OBJECT) $(ARCHIVE)
	$(CC) -DFEAT_PLUTOVG $(CFLAGS) -o $@ $(SOURCE) $(OBJECT) $(ARCHIVE) $(LDLIBS)

there_she_is.o:there_she_is.c
	time $(CC) -DFEAT_PLUTOVG $(CFLAGS) -O0 -c $<

there_she_is.c:hello-rust/src/main.rs
	pushd hello-rust; cargo run -- c > ../$@

there_she_is.h:hello-rust/src/main.rs
	pushd hello-rust; cargo run -- h > ../$@

main.c:there_she_is.h
	touch $@

plutovg:
	git clone https://github.com/sammycage/plutovg.git

plutovg.a:
	pushd plutovg/source && \
	$(CC) -DPLUTOVG_BUILD -Wno-sign-compare -Wno-unused-function -c *.c -I../include && \
	ar r ../../plutovg.a *.o

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
