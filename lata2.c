T{
.nf
\01
\02
\03
\04
\05

\06
\07
\08
\09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27

28
29
30
31
32
33
34
35
36
37

38
39
40
41
42
43
44
45
46
47
48
49
50
51
T}	T{
.nf
#define X(ID,F) static void ca_f##ID(\f(CBcairo_t\fP *cr) { \f(CBcairo_set_source_rgba\fP(cr,F); }
#define S(R,G,B,A) R/255.0, G/255.0, B/255.0, A/255.0
\f(CBFILL_XS\fP
#undef S
#undef X

#define MKH(L,T) {.header = {.length=(L),.type=CAIRO_PATH_##T}}
#define MKP(X,Y) {.point={(X), (Y)}}
#define M(X, Y) MKH(2,MOVE_TO),MKP((X),(Y)),
#define L(X, Y) MKH(2,LINE_TO),MKP((X),(Y)),
#define B(X0, Y0, CX, CY, X, Y) MKH(4,CURVE_TO),\e
\0MKP(2.0 / 3.0 * (CX) + 1.0 / 3.0 * (X0), 2.0 / 3.0 * (CY) + 1.0 / 3.0 * (Y0)),\e
\0MKP(2.0 / 3.0 * (CX) + 1.0 / 3.0 * (X) , 2.0 / 3.0 * (CY) + 1.0 / 3.0 * (Y)),\e
\0MKP((X),(Y)),
#define C MKH(1,CLOSE_PATH),
#define A(ID,FID,CTOR) static \f(CBcairo_path_data_t\fP ca_d##ID[] = { CTOR };
#define S(ID,FID,WID,CTOR) static \f(CBcairo_path_data_t\fP ca_d##ID[] = { CTOR };
#define X(ID,CTOR) CTOR
\f(CBSHAPE_XS\fP
#undef X
#undef S
#undef A
#undef C
#undef B
#undef L
#undef M
#undef MKP
#undef MKH

#define MKD(ID) static \f(CBcairo_path_t\fP ca_p##ID = \e
\0{ CAIRO_STATUS_SUCCESS, ca_d##ID, sizeof (ca_d##ID) / sizeof (\f(CBcairo_path_data_t\fP) };
#define A(ID,FID,CTOR) MKD(ID)
#define S(ID,FID,WID,CTOR) MKD(ID)
#define X(ID,CTOR) CTOR
\f(CBSHAPE_XS\fP
#undef X
#undef S
#undef A
#undef MKD

#define A(ID,FID,CTOR) \e
\0ca_f##FID(cr); \e
\0\f(CBcairo_append_path\fP(cr, &ca_p##ID); \e
\0\f(CBcairo_fill\fP(cr);
#define S(ID,FID,WID,CTOR) \e
\0\f(CBcairo_set_line_width\fP(cr,WID); \e
\0\f(CBcairo_append_path\fP(cr, &ca_p##ID); \e
\0ca_f##FID(cr); \e
\0\f(CBcairo_stroke\fP(cr);
#define X(ID, CTOR) static void car##ID(\f(CBcairo_t\fP *cr) { (void)cr; CTOR }
\f(CBSHAPE_XS\fP
#undef X
#undef S
#undef A
T}