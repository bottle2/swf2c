divert(-1)

define(`F',`define(`ID',`$1')$2`'dnl')
define(`P',`PI($@)`'dnl')
define(`D',`DI($@)`'dnl')
define(`V',`VI($@)`'dnl')

define(`PICTURES',`$1`'dnl
F(`there_she_is',`S(2150,`CUTE')`'S(1354,TOILET)')
D(`res/there_cute.pdf',`CUTE2')
D(`res/toilet.pdf',`TOILET2')
V(`res/137',`HEART')
F(`bitey_of_brackenwood',`S(4686,`BRUXA')')
D(`res/bruxa.pdf',`BRUXA2')
F(`miss_dynamite_x',`S(3430,`IMPALA')`'S(4174,`INTER')')
D(`res/impala.pdf',`IMPALA2')
F(`miss_dynamite_xiv',`S(1817,`GOSTOSA')')
V(`res/1817',`GOSTOSA2')
F(`miss_dynamite_xv',`S(7000,`COMUNISMO')')
F(`dad_s_home',`S(880,`BAGUN')`'S(3604,`POLICIA')')
D(`res/bagun.pdf',`BAGUN2')
D(`res/policia.pdf',`POLICIA2')
F(`de_aztec',`S(2318,`PISTOLA')')
F(`de_dust',`S(1132,`MATRIX')`'S(179,`DUST1')')
F(`miss_dynamite_halloween',`S(1369,`MONSTER')')
V(`res/monster',`MONSTER2')
P(`res/stream',`STREAM',`png')
P(`res/snapshot',`3DI',`jpg')
P(`res/borderlands2',`BORDERLANDS2',`jpg')
D(`res/juicy.pdf',`BURRITO')
P(`res/snapshot3',`GUMBALL',`jpg')
P(`res/snapshot4',`INCREDIBLES',`jpg')
P(`res/snapshot5',`CEBOLINHA',`jpg')
D(`res/dota8-cropped.pdf',`DOTA2')
P(`res/ching2',`BEAUTIFUL',`png')
P(`res/chong',`BEAUTIFUL2',`jpg')
D(`res/intro2.pdf',`BEAUTIFUL3')
V(`res/134',`TRANS')
V(`res/md502',`MDXVa')
V(`res/md503',`MDXVz')
V(`res/md505',`MDXVa')
V(`res/md506',`MDXVb')
V(`res/md507',`MDXVc')
V(`res/md509',`MDXVx')
V(`res/md511',`MDXVd')
V(`res/md512',`MDXVe')
V(`res/md514',`MDXVf')
D(`res/circ.pdf',`CIRCGRAD')
D(`res/rect.pdf',`RECTGRAD')
D(`res/omg/contorno1.pdf',`OMG_CONTORNO1')
D(`res/omg/ns_contorno-1.pdf',`OMG_CONTORNO2')
V(`res/omg/full',`OMG_FULL')
D(`res/omg/fs1_ruim.pdf',`OMG_AAA')
D(`res/omg/fs1_bom.pdf',`OMG_AAB')
D(`res/omg/fs2_ruim.pdf',`OMG_AAC')
D(`res/omg/fs2_bom.pdf',`OMG_AAD')
D(`res/omg2/good.pdf',`OMG2_GOOD')
D(`res/omg2/tudo.pdf',`OMG2_TUDO')
V(`res/omg2/1030',`OMG2_FULL')
P(`res/portal',`JUEGOS',`png')
P(`res/kong',`KONG',`png')
')

define(`AS_MAKE_LIST',`dnl
`'`PICTURES'=define(`S',` \
work/ID`'_$'`1.pdf')`'define(`PI',` \
$'`1.pdf')`'define(`DI',` \
$'`1')`'define(`VI',` \
$'`1.pdf')`'dnl
')

define(`AS_MAKE_RECIPE',`

`'define(`S',`work/ID`'_$'`1.pdf:swf2pdf work/ID.dll
	./swf2pdf work/ID.dll $'`1 work/ID`'_$'`1_tmp.pdf && \
	qpdf work/ID`'_$'`1_tmp.pdf $`'@ && rm work/ID`'_$'`1_tmp.pdf
')`'dnl
define(`PI',`$'`1'.pdf:`$'`1'.`$'`3'
	gm convert $``''< $``''*_tmp.pdf && \
	qpdf $``''*_tmp.pdf $``''@; rm $``''*_tmp.pdf
)`'dnl
define(`DI',)`'dnl
define(`VI',`$'`1.pdf:$'`1.svg
	rsvg-convert -f pdf -o $'`1_tmp.pdf $'`1.svg
	qpdf $'`1_tmp.pdf $'`1.pdf; rm $'`1_tmp.pdf
')`'dnl
')

define(`AS_MOM',`dnl
`'define(`S',`dnl
.de PIC_'$`'2`
. PDF_IMAGE -C work/ID`'_'$`'1`.pdf syscmd(./swf2c -s3 work/ID.swf) \\$`'*
..
')`'dnl
`'define(`PI',`dnl
.de PIC_'$`'2`
. PDF_IMAGE -C $'`1.pdf syscmd(./hack.sh $'`1.pdf) \\$`'*
..
')`'dnl
`'define(`DI',`dnl
.de PIC_'$`'2`
. PDF_IMAGE -C $'`1 syscmd(./hack.sh $'`1) \\$`'*
..
')`'dnl
`'define(`VI',`dnl
.de PIC_'$`'2`
. PDF_IMAGE -C $'`1.pdf syscmd(./hack.sh $'`1.pdf) \\$`'*
..
')`'dnl
')

divert(0)PICTURES(`gen')`'dnl
