PICTURES= \
work/there_she_is_2150.pdf \
work/there_she_is_1354.pdf \
res/there_cute.pdf \
res/toilet.pdf \
res/137.pdf \
work/bitey_of_brackenwood_4686.pdf \
res/bruxa.pdf \
work/miss_dynamite_x_3430.pdf \
work/miss_dynamite_x_4174.pdf \
res/impala.pdf \
work/miss_dynamite_xiv_1817.pdf \
res/1817.pdf \
work/miss_dynamite_xv_7000.pdf \
work/dad_s_home_880.pdf \
work/dad_s_home_3604.pdf \
res/bagun.pdf \
res/policia.pdf \
work/de_aztec_2318.pdf \
work/de_dust_1132.pdf \
work/de_dust_179.pdf \
work/miss_dynamite_halloween_1369.pdf \
res/monster.pdf \
res/stream.pdf \
res/snapshot.pdf \
res/borderlands2.pdf \
res/juicy.pdf \
res/snapshot3.pdf \
res/snapshot4.pdf \
res/snapshot5.pdf \
res/dota8-cropped.pdf \
res/ching2.pdf \
res/chong.pdf \
res/intro2.pdf \
res/134.pdf \
res/md502.pdf \
res/md503.pdf \
res/md505.pdf \
res/md506.pdf \
res/md507.pdf \
res/md509.pdf \
res/md511.pdf \
res/md512.pdf \
res/md514.pdf \
res/circ.pdf \
res/rect.pdf \
res/omg/contorno1.pdf \
res/omg/ns_contorno-1.pdf \
res/omg/full.pdf \
res/omg/fs1_ruim.pdf \
res/omg/fs1_bom.pdf \
res/omg/fs2_ruim.pdf \
res/omg/fs2_bom.pdf \
res/omg2/good.pdf \
res/omg2/tudo.pdf \
res/omg2/1030.pdf \
res/portal.pdf \
res/kong.pdf

work/there_she_is_2150.pdf:swf2pdf work/there_she_is.dll
	./swf2pdf work/there_she_is.dll 2150 work/there_she_is_2150_tmp.pdf && \
	qpdf work/there_she_is_2150_tmp.pdf $@ && rm work/there_she_is_2150_tmp.pdf
work/there_she_is_1354.pdf:swf2pdf work/there_she_is.dll
	./swf2pdf work/there_she_is.dll 1354 work/there_she_is_1354_tmp.pdf && \
	qpdf work/there_she_is_1354_tmp.pdf $@ && rm work/there_she_is_1354_tmp.pdf
res/137.pdf:res/137.svg
	rsvg-convert -f pdf -o res/137_tmp.pdf res/137.svg
	qpdf res/137_tmp.pdf res/137.pdf; rm res/137_tmp.pdf
work/bitey_of_brackenwood_4686.pdf:swf2pdf work/bitey_of_brackenwood.dll
	./swf2pdf work/bitey_of_brackenwood.dll 4686 work/bitey_of_brackenwood_4686_tmp.pdf && \
	qpdf work/bitey_of_brackenwood_4686_tmp.pdf $@ && rm work/bitey_of_brackenwood_4686_tmp.pdf
work/miss_dynamite_x_3430.pdf:swf2pdf work/miss_dynamite_x.dll
	./swf2pdf work/miss_dynamite_x.dll 3430 work/miss_dynamite_x_3430_tmp.pdf && \
	qpdf work/miss_dynamite_x_3430_tmp.pdf $@ && rm work/miss_dynamite_x_3430_tmp.pdf
work/miss_dynamite_x_4174.pdf:swf2pdf work/miss_dynamite_x.dll
	./swf2pdf work/miss_dynamite_x.dll 4174 work/miss_dynamite_x_4174_tmp.pdf && \
	qpdf work/miss_dynamite_x_4174_tmp.pdf $@ && rm work/miss_dynamite_x_4174_tmp.pdf
work/miss_dynamite_xiv_1817.pdf:swf2pdf work/miss_dynamite_xiv.dll
	./swf2pdf work/miss_dynamite_xiv.dll 1817 work/miss_dynamite_xiv_1817_tmp.pdf && \
	qpdf work/miss_dynamite_xiv_1817_tmp.pdf $@ && rm work/miss_dynamite_xiv_1817_tmp.pdf
res/1817.pdf:res/1817.svg
	rsvg-convert -f pdf -o res/1817_tmp.pdf res/1817.svg
	qpdf res/1817_tmp.pdf res/1817.pdf; rm res/1817_tmp.pdf
work/miss_dynamite_xv_7000.pdf:swf2pdf work/miss_dynamite_xv.dll
	./swf2pdf work/miss_dynamite_xv.dll 7000 work/miss_dynamite_xv_7000_tmp.pdf && \
	qpdf work/miss_dynamite_xv_7000_tmp.pdf $@ && rm work/miss_dynamite_xv_7000_tmp.pdf
work/dad_s_home_880.pdf:swf2pdf work/dad_s_home.dll
	./swf2pdf work/dad_s_home.dll 880 work/dad_s_home_880_tmp.pdf && \
	qpdf work/dad_s_home_880_tmp.pdf $@ && rm work/dad_s_home_880_tmp.pdf
work/dad_s_home_3604.pdf:swf2pdf work/dad_s_home.dll
	./swf2pdf work/dad_s_home.dll 3604 work/dad_s_home_3604_tmp.pdf && \
	qpdf work/dad_s_home_3604_tmp.pdf $@ && rm work/dad_s_home_3604_tmp.pdf
work/de_aztec_2318.pdf:swf2pdf work/de_aztec.dll
	./swf2pdf work/de_aztec.dll 2318 work/de_aztec_2318_tmp.pdf && \
	qpdf work/de_aztec_2318_tmp.pdf $@ && rm work/de_aztec_2318_tmp.pdf
work/de_dust_1132.pdf:swf2pdf work/de_dust.dll
	./swf2pdf work/de_dust.dll 1132 work/de_dust_1132_tmp.pdf && \
	qpdf work/de_dust_1132_tmp.pdf $@ && rm work/de_dust_1132_tmp.pdf
work/de_dust_179.pdf:swf2pdf work/de_dust.dll
	./swf2pdf work/de_dust.dll 179 work/de_dust_179_tmp.pdf && \
	qpdf work/de_dust_179_tmp.pdf $@ && rm work/de_dust_179_tmp.pdf
work/miss_dynamite_halloween_1369.pdf:swf2pdf work/miss_dynamite_halloween.dll
	./swf2pdf work/miss_dynamite_halloween.dll 1369 work/miss_dynamite_halloween_1369_tmp.pdf && \
	qpdf work/miss_dynamite_halloween_1369_tmp.pdf $@ && rm work/miss_dynamite_halloween_1369_tmp.pdf
res/monster.pdf:res/monster.svg
	rsvg-convert -f pdf -o res/monster_tmp.pdf res/monster.svg
	qpdf res/monster_tmp.pdf res/monster.pdf; rm res/monster_tmp.pdf
res/stream.pdf:res/stream.png
	gm convert $< $*_tmp.pdf && \
	qpdf $*_tmp.pdf $@; rm $*_tmp.pdf
res/snapshot.pdf:res/snapshot.jpg
	gm convert $< $*_tmp.pdf && \
	qpdf $*_tmp.pdf $@; rm $*_tmp.pdf
res/borderlands2.pdf:res/borderlands2.jpg
	gm convert $< $*_tmp.pdf && \
	qpdf $*_tmp.pdf $@; rm $*_tmp.pdf
res/snapshot3.pdf:res/snapshot3.jpg
	gm convert $< $*_tmp.pdf && \
	qpdf $*_tmp.pdf $@; rm $*_tmp.pdf
res/snapshot4.pdf:res/snapshot4.jpg
	gm convert $< $*_tmp.pdf && \
	qpdf $*_tmp.pdf $@; rm $*_tmp.pdf
res/snapshot5.pdf:res/snapshot5.jpg
	gm convert $< $*_tmp.pdf && \
	qpdf $*_tmp.pdf $@; rm $*_tmp.pdf
res/ching2.pdf:res/ching2.png
	gm convert $< $*_tmp.pdf && \
	qpdf $*_tmp.pdf $@; rm $*_tmp.pdf
res/chong.pdf:res/chong.jpg
	gm convert $< $*_tmp.pdf && \
	qpdf $*_tmp.pdf $@; rm $*_tmp.pdf
res/134.pdf:res/134.svg
	rsvg-convert -f pdf -o res/134_tmp.pdf res/134.svg
	qpdf res/134_tmp.pdf res/134.pdf; rm res/134_tmp.pdf
res/md502.pdf:res/md502.svg
	rsvg-convert -f pdf -o res/md502_tmp.pdf res/md502.svg
	qpdf res/md502_tmp.pdf res/md502.pdf; rm res/md502_tmp.pdf
res/md503.pdf:res/md503.svg
	rsvg-convert -f pdf -o res/md503_tmp.pdf res/md503.svg
	qpdf res/md503_tmp.pdf res/md503.pdf; rm res/md503_tmp.pdf
res/md505.pdf:res/md505.svg
	rsvg-convert -f pdf -o res/md505_tmp.pdf res/md505.svg
	qpdf res/md505_tmp.pdf res/md505.pdf; rm res/md505_tmp.pdf
res/md506.pdf:res/md506.svg
	rsvg-convert -f pdf -o res/md506_tmp.pdf res/md506.svg
	qpdf res/md506_tmp.pdf res/md506.pdf; rm res/md506_tmp.pdf
res/md507.pdf:res/md507.svg
	rsvg-convert -f pdf -o res/md507_tmp.pdf res/md507.svg
	qpdf res/md507_tmp.pdf res/md507.pdf; rm res/md507_tmp.pdf
res/md509.pdf:res/md509.svg
	rsvg-convert -f pdf -o res/md509_tmp.pdf res/md509.svg
	qpdf res/md509_tmp.pdf res/md509.pdf; rm res/md509_tmp.pdf
res/md511.pdf:res/md511.svg
	rsvg-convert -f pdf -o res/md511_tmp.pdf res/md511.svg
	qpdf res/md511_tmp.pdf res/md511.pdf; rm res/md511_tmp.pdf
res/md512.pdf:res/md512.svg
	rsvg-convert -f pdf -o res/md512_tmp.pdf res/md512.svg
	qpdf res/md512_tmp.pdf res/md512.pdf; rm res/md512_tmp.pdf
res/md514.pdf:res/md514.svg
	rsvg-convert -f pdf -o res/md514_tmp.pdf res/md514.svg
	qpdf res/md514_tmp.pdf res/md514.pdf; rm res/md514_tmp.pdf
res/omg/full.pdf:res/omg/full.svg
	rsvg-convert -f pdf -o res/omg/full_tmp.pdf res/omg/full.svg
	qpdf res/omg/full_tmp.pdf res/omg/full.pdf; rm res/omg/full_tmp.pdf
res/omg2/1030.pdf:res/omg2/1030.svg
	rsvg-convert -f pdf -o res/omg2/1030_tmp.pdf res/omg2/1030.svg
	qpdf res/omg2/1030_tmp.pdf res/omg2/1030.pdf; rm res/omg2/1030_tmp.pdf
res/portal.pdf:res/portal.png
	gm convert $< $*_tmp.pdf && \
	qpdf $*_tmp.pdf $@; rm $*_tmp.pdf
res/kong.pdf:res/kong.png
	gm convert $< $*_tmp.pdf && \
	qpdf $*_tmp.pdf $@; rm $*_tmp.pdf
