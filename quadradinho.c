#define SHAPE_XS \
X(1, \
 A(1,1, \
  M(0,0) \
  L(0,100) L(50,100) \
  B(50,100,50,50,100,50) \
  L(100,0) L(0,0) \
  C) \
 S(1,2,2, \
  M(50,100) \
  B(50,100,50,50,100,50) \
  L(100,100) L(50,100)))

#define MKH(L,T) { \
        .header = {.length=(L), \
                   .type=CAIRO_PATH_##T}}
#define MKP(X,Y) {.point={(X), (Y)}}
#define M(X, Y) MKH(2,MOVE_TO),MKP((X),(Y)),
#define L(X, Y) MKH(2,LINE_TO),MKP((X),(Y)),
#define C MKH(1,CLOSE_PATH),
#define A(ID,FID,CTOR) \
        static cairo_path_data_t ca_d##ID[] = { CTOR };
#define X(ID,CTOR) CTOR
SHAPE_XS
#undef X
#undef A
#undef C
#undef L
#undef M
#undef MKP
#undef MKH

#define CASZ sizeof (cairo_path_data_t)

#define MKD(ID) static cairo_path_t ca_p##ID = { \
        CAIRO_STATUS_SUCCESS, ca_d##ID, \
        sizeof (ca_d##ID) / CASZ };
#define A(ID,FID,CTOR) MKD(ID)
#define X(ID,CTOR) CTOR
SHAPE_XS
#undef X
#undef A
#undef MKD

#define A(ID,FID,CTOR) \
        ca_f##FID(cr); \
        cairo_append_path(cr, &ca_p##ID); \
        cairo_fill(cr);
#define X(ID, CTOR) static void car##ID(cairo_t *cr) { \
        (void)cr; CTOR }
SHAPE_XS
#undef X
#undef A
