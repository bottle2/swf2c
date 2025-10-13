divert(-1)

define(`F',`define(`ID',`$1')$2`'dnl')

define(`PICTURES',`$1`'dnl
F(`there_she_is',`S(2150,`CUTE')`'S(1000,FUCK)')
')

define(`AS_MAKE_LIST',`dnl
`'`PICTURES'=define(`S',` \
work/ID`'_$'`1.pdf')dnl
')

define(`AS_MAKE_RECIPE',`

`'define(`S',`work/ID`'_$'`1.pdf:swf2pdf work/ID.dll
	./swf2pdf work/ID.dll $'`1 work/ID`'_$'`1_tmp.pdf && \
	qpdf work/ID`'_$'`1_tmp.pdf $`'@ && rm work/ID`'_$'`1_tmp.pdf
')`'dnl
')

define(`AS_MOM',`dnl
`'define(`S',`dnl
.de PIC_'$`'2`
. PDF_IMAGE -C work/ID`'_'$`'1`.pdf syscmd(./swf2c -s3 work/ID.swf)
..
')`'dnl
')

divert(0)PICTURES(`gen')`'dnl
