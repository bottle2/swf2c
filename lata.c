xx #define X(ID,F) static void ca_f##ID(\f(CBcairo_t\fP *cr) { \f(CBcairo_set_source_rgba\fP(cr,F); }
xx #define S(R,G,B,A) R/255.0, G/255.0, B/255.0, A/255.0
xx \f(CBFILL_XS\fP
xx #undef S
xx #undef X

xx #define MKH(L,T) {.header = {.length=(L),.type=CAIRO_PATH_##T}}
xx #define MKP(X,Y) {.point={(X), (Y)}}
xx #define M(X, Y) MKH(2,MOVE_TO),MKP((X),(Y)),
xx #define L(X, Y) MKH(2,LINE_TO),MKP((X),(Y)),
xx #define B(X0, Y0, CX, CY, X, Y) MKH(4,CURVE_TO),\e
xx  MKP(2.0 / 3.0 * (CX) + 1.0 / 3.0 * (X0), 2.0 / 3.0 * (CY) + 1.0 / 3.0 * (Y0)),\e
xx  MKP(2.0 / 3.0 * (CX) + 1.0 / 3.0 * (X) , 2.0 / 3.0 * (CY) + 1.0 / 3.0 * (Y)),\e
xx  MKP((X),(Y)),
xx #define C MKH(1,CLOSE_PATH),
xx #define A(ID,FID,CTOR) static \f(CBcairo_path_data_t\fP ca_d##ID[] = { CTOR };
xx #define S(ID,FID,WID,CTOR) static \f(CBcairo_path_data_t\fP ca_d##ID[] = { CTOR };
xx #define X(ID,CTOR) CTOR
xx \f(CBSHAPE_XS\fP
xx #undef X
xx #undef S
xx #undef A
xx #undef C
xx #undef B
xx #undef L
xx #undef M
xx #undef MKP
xx #undef MKH

xx #define MKD(ID) static \f(CBcairo_path_t\fP ca_p##ID = \e
xx  { CAIRO_STATUS_SUCCESS, ca_d##ID, sizeof (ca_d##ID) / sizeof (\f(CBcairo_path_data_t\fP) };
xx #define A(ID,FID,CTOR) MKD(ID)
xx #define S(ID,FID,WID,CTOR) MKD(ID)
xx #define X(ID,CTOR) CTOR
xx \f(CBSHAPE_XS\fP
xx #undef X
xx #undef S
xx #undef A
xx #undef MKD

xx #define A(ID,FID,CTOR) \e
xx  ca_f##FID(cr); \e
xx  \f(CBcairo_append_path\fP(cr, &ca_p##ID); \e
xx  \f(CBcairo_fill\fP(cr);
xx #define S(ID,FID,WID,CTOR) \e
xx  \f(CBcairo_set_line_width\fP(cr,WID); \e
xx  \f(CBcairo_append_path\fP(cr, &ca_p##ID); \e
xx  ca_f##FID(cr); \e
xx  \f(CBcairo_stroke\fP(cr);
xx #define X(ID, CTOR) static void car##ID(\f(CBcairo_t\fP *cr) { (void)cr; CTOR }
xx \f(CBSHAPE_XS\fP
xx #undef X
xx #undef S
xx #undef A
