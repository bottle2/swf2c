
#line 1 "colonize.rl"
#include <stdio.h>

int lead = 0;
char *le_bold;

_Bool wants = 0;

int n;
static unsigned char buf[500000];
int ln = 0;
_Bool done = 0;

struct {
    int cs;
    unsigned char *p, *pe, *eof;
} cnt, key;


#line 22 "colonize.c"
static const int count_start = 5;


#line 37 "colonize.rl"


void numerize(void) { 
#line 30 "colonize.c"
	{
	if ( ( cnt.p) == ( cnt.pe) )
		goto _test_eof;
	switch (  cnt.cs )
	{
tr4:
#line 27 "colonize.rl"
	{ {( cnt.p)++;  cnt.cs = 5; goto _out;} }
	goto st5;
tr7:
#line 28 "colonize.rl"
	{
        ln++;
        char *rec = "\\0\\0" + (6 - lead*2 + (ln >= 10)*2 + (ln >= 100)*2);
        printf("%s%d", rec, ln);
    }
	goto st5;
tr9:
#line 26 "colonize.rl"
	{ putchar('\n'); }
	goto st5;
st5:
	if ( ++( cnt.p) == ( cnt.pe) )
		goto _test_eof5;
case 5:
#line 56 "colonize.c"
	switch( (*( cnt.p)) ) {
		case 10u: goto tr9;
		case 78u: goto tr10;
		case 120u: goto tr11;
	}
	goto tr8;
tr3:
#line 27 "colonize.rl"
	{ {( cnt.p)++;  cnt.cs = 0; goto _out;} }
	goto st0;
tr6:
#line 28 "colonize.rl"
	{
        ln++;
        char *rec = "\\0\\0" + (6 - lead*2 + (ln >= 10)*2 + (ln >= 100)*2);
        printf("%s%d", rec, ln);
    }
	goto st0;
tr8:
#line 26 "colonize.rl"
	{ putchar('\n'); }
	goto st0;
st0:
	if ( ++( cnt.p) == ( cnt.pe) )
		goto _test_eof0;
case 0:
#line 83 "colonize.c"
	if ( (*( cnt.p)) == 10u )
		goto st5;
	goto st0;
tr10:
#line 26 "colonize.rl"
	{ putchar('\n'); }
	goto st1;
st1:
	if ( ++( cnt.p) == ( cnt.pe) )
		goto _test_eof1;
case 1:
#line 95 "colonize.c"
	switch( (*( cnt.p)) ) {
		case 10u: goto st5;
		case 78u: goto st2;
	}
	goto st0;
st2:
	if ( ++( cnt.p) == ( cnt.pe) )
		goto _test_eof2;
case 2:
	if ( (*( cnt.p)) == 10u )
		goto tr4;
	goto tr3;
tr11:
#line 26 "colonize.rl"
	{ putchar('\n'); }
	goto st3;
st3:
	if ( ++( cnt.p) == ( cnt.pe) )
		goto _test_eof3;
case 3:
#line 116 "colonize.c"
	switch( (*( cnt.p)) ) {
		case 10u: goto st5;
		case 120u: goto st4;
	}
	goto st0;
st4:
	if ( ++( cnt.p) == ( cnt.pe) )
		goto _test_eof4;
case 4:
	if ( (*( cnt.p)) == 10u )
		goto tr7;
	goto tr6;
	}
	_test_eof5:  cnt.cs = 5; goto _test_eof; 
	_test_eof0:  cnt.cs = 0; goto _test_eof; 
	_test_eof1:  cnt.cs = 1; goto _test_eof; 
	_test_eof2:  cnt.cs = 2; goto _test_eof; 
	_test_eof3:  cnt.cs = 3; goto _test_eof; 
	_test_eof4:  cnt.cs = 4; goto _test_eof; 

	_test_eof: {}
	if ( ( cnt.p) == ( cnt.eof) )
	{
	switch (  cnt.cs ) {
	case 5: 
#line 26 "colonize.rl"
	{ putchar('\n'); }
	break;
#line 145 "colonize.c"
	}
	}

	_out: {}
	}

#line 40 "colonize.rl"
}


#line 156 "colonize.c"
static const int process_start = 30;


#line 82 "colonize.rl"


void work(void) {
    unsigned char *m;
    
#line 166 "colonize.c"
	{
	if ( ( key.p) == ( key.pe) )
		goto _test_eof;
	switch (  key.cs )
	{
tr1:
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st30;
tr14:
#line 54 "colonize.rl"
	{
        if (wants)
            printf("%s%.*s\\fP", le_bold, (int)(( key.p) - m), m);
        else
            printf("%.*s", (int)(( key.p) - m), m);
        m = ( key.p);
    }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st30;
tr28:
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st30;
tr50:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 30; goto _out;} }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st30;
tr64:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st30;
st30:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof30;
case 30:
#line 214 "colonize.c"
	switch( (*( key.p)) ) {
		case 10u: goto tr64;
		case 13u: goto tr65;
		case 32u: goto tr66;
		case 35u: goto tr67;
		case 36u: goto tr68;
		case 78u: goto tr69;
		case 95u: goto tr67;
		case 101u: goto tr70;
		case 102u: goto tr71;
		case 105u: goto tr72;
		case 108u: goto tr73;
		case 112u: goto tr74;
		case 119u: goto tr75;
		case 120u: goto tr76;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto tr67;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto tr67;
	} else
		goto tr67;
	goto tr63;
tr2:
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st1;
tr13:
#line 54 "colonize.rl"
	{
        if (wants)
            printf("%s%.*s\\fP", le_bold, (int)(( key.p) - m), m);
        else
            printf("%.*s", (int)(( key.p) - m), m);
        m = ( key.p);
    }
	goto st1;
tr15:
#line 54 "colonize.rl"
	{
        if (wants)
            printf("%s%.*s\\fP", le_bold, (int)(( key.p) - m), m);
        else
            printf("%.*s", (int)(( key.p) - m), m);
        m = ( key.p);
    }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st1;
tr27:
#line 62 "colonize.rl"
	{ m = ( key.p); }
	goto st1;
tr29:
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st1;
tr49:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 1; goto _out;} }
#line 62 "colonize.rl"
	{ m = ( key.p); }
	goto st1;
tr51:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 1; goto _out;} }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st1;
tr63:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 62 "colonize.rl"
	{ m = ( key.p); }
	goto st1;
tr65:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st1;
st1:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof1;
case 1:
#line 308 "colonize.c"
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 101u: goto tr4;
		case 102u: goto tr5;
		case 105u: goto tr6;
		case 108u: goto tr7;
		case 112u: goto tr8;
		case 119u: goto tr9;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
tr40:
#line 54 "colonize.rl"
	{
        if (wants)
            printf("%s%.*s\\fP", le_bold, (int)(( key.p) - m), m);
        else
            printf("%.*s", (int)(( key.p) - m), m);
        m = ( key.p);
    }
	goto st2;
tr31:
#line 62 "colonize.rl"
	{ m = ( key.p); }
	goto st2;
tr53:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 2; goto _out;} }
#line 62 "colonize.rl"
	{ m = ( key.p); }
	goto st2;
tr67:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 62 "colonize.rl"
	{ m = ( key.p); }
	goto st2;
st2:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof2;
case 2:
#line 360 "colonize.c"
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
tr4:
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st3;
tr41:
#line 54 "colonize.rl"
	{
        if (wants)
            printf("%s%.*s\\fP", le_bold, (int)(( key.p) - m), m);
        else
            printf("%.*s", (int)(( key.p) - m), m);
        m = ( key.p);
    }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st3;
tr33:
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st3;
tr55:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 3; goto _out;} }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st3;
tr70:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st3;
st3:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof3;
case 3:
#line 418 "colonize.c"
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 108u: goto st4;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st4:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof4;
case 4:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 115u: goto st5;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st5:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof5;
case 5:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 101u: goto st6;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st6:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof6;
case 6:
	switch( (*( key.p)) ) {
		case 10u: goto tr14;
		case 13u: goto tr15;
		case 35u: goto st2;
		case 95u: goto st2;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto tr13;
tr5:
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st7;
tr42:
#line 54 "colonize.rl"
	{
        if (wants)
            printf("%s%.*s\\fP", le_bold, (int)(( key.p) - m), m);
        else
            printf("%.*s", (int)(( key.p) - m), m);
        m = ( key.p);
    }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st7;
tr34:
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st7;
tr56:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 7; goto _out;} }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st7;
tr71:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st7;
st7:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof7;
case 7:
#line 536 "colonize.c"
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 111u: goto st8;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st8:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof8;
case 8:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 114u: goto st6;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
tr6:
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st9;
tr43:
#line 54 "colonize.rl"
	{
        if (wants)
            printf("%s%.*s\\fP", le_bold, (int)(( key.p) - m), m);
        else
            printf("%.*s", (int)(( key.p) - m), m);
        m = ( key.p);
    }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st9;
tr35:
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st9;
tr57:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 9; goto _out;} }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st9;
tr72:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st9;
st9:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof9;
case 9:
#line 615 "colonize.c"
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 102u: goto st6;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
tr7:
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st10;
tr44:
#line 54 "colonize.rl"
	{
        if (wants)
            printf("%s%.*s\\fP", le_bold, (int)(( key.p) - m), m);
        else
            printf("%.*s", (int)(( key.p) - m), m);
        m = ( key.p);
    }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st10;
tr36:
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st10;
tr58:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 10; goto _out;} }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st10;
tr73:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st10;
st10:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof10;
case 10:
#line 674 "colonize.c"
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 111u: goto st11;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st11:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof11;
case 11:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 111u: goto st12;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st12:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof12;
case 12:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 112u: goto st6;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
tr8:
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st13;
tr45:
#line 54 "colonize.rl"
	{
        if (wants)
            printf("%s%.*s\\fP", le_bold, (int)(( key.p) - m), m);
        else
            printf("%.*s", (int)(( key.p) - m), m);
        m = ( key.p);
    }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st13;
tr37:
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st13;
tr59:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 13; goto _out;} }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st13;
tr74:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st13;
st13:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof13;
case 13:
#line 773 "colonize.c"
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 114u: goto st14;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st14:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof14;
case 14:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 111u: goto st15;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st15:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof15;
case 15:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 99u: goto st16;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st16:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof16;
case 16:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 101u: goto st17;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st17:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof17;
case 17:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 100u: goto st18;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st18:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof18;
case 18:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 117u: goto st19;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st19:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof19;
case 19:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 114u: goto st5;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
tr9:
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st20;
tr46:
#line 54 "colonize.rl"
	{
        if (wants)
            printf("%s%.*s\\fP", le_bold, (int)(( key.p) - m), m);
        else
            printf("%.*s", (int)(( key.p) - m), m);
        m = ( key.p);
    }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st20;
tr38:
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st20;
tr60:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 20; goto _out;} }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st20;
tr75:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 62 "colonize.rl"
	{ m = ( key.p); }
#line 61 "colonize.rl"
	{ printf("%.*s", (int)(( key.p) - m), m); m = ( key.p); }
	goto st20;
st20:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof20;
case 20:
#line 952 "colonize.c"
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 104u: goto st21;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st21:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof21;
case 21:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 105u: goto st22;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st22:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof22;
case 22:
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 35u: goto st2;
		case 95u: goto st2;
		case 108u: goto st5;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
tr30:
#line 51 "colonize.rl"
	{ putchar('\\'); putchar('0'); }
	goto st23;
tr52:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 23; goto _out;} }
	goto st23;
tr66:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 51 "colonize.rl"
	{ putchar('\\'); putchar('0'); }
	goto st23;
st23:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof23;
case 23:
#line 1027 "colonize.c"
	switch( (*( key.p)) ) {
		case 10u: goto tr28;
		case 13u: goto tr29;
		case 32u: goto tr30;
		case 35u: goto tr31;
		case 36u: goto tr32;
		case 95u: goto tr31;
		case 101u: goto tr33;
		case 102u: goto tr34;
		case 105u: goto tr35;
		case 108u: goto tr36;
		case 112u: goto tr37;
		case 119u: goto tr38;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto tr31;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto tr31;
	} else
		goto tr31;
	goto tr27;
tr32:
#line 62 "colonize.rl"
	{ m = ( key.p); }
	goto st24;
tr54:
#line 50 "colonize.rl"
	{ {( key.p)++;  key.cs = 24; goto _out;} }
#line 62 "colonize.rl"
	{ m = ( key.p); }
	goto st24;
tr68:
#line 53 "colonize.rl"
	{ putchar('\n'); }
#line 62 "colonize.rl"
	{ m = ( key.p); }
	goto st24;
st24:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof24;
case 24:
#line 1071 "colonize.c"
	switch( (*( key.p)) ) {
		case 10u: goto tr1;
		case 13u: goto tr2;
		case 32u: goto st25;
		case 35u: goto st2;
		case 95u: goto st2;
		case 101u: goto tr4;
		case 102u: goto tr5;
		case 105u: goto tr6;
		case 108u: goto tr7;
		case 112u: goto tr8;
		case 119u: goto tr9;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto st2;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto st2;
	} else
		goto st2;
	goto st1;
st25:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof25;
case 25:
	switch( (*( key.p)) ) {
		case 10u: goto tr14;
		case 13u: goto tr15;
		case 35u: goto tr40;
		case 95u: goto tr40;
		case 101u: goto tr41;
		case 102u: goto tr42;
		case 105u: goto tr43;
		case 108u: goto tr44;
		case 112u: goto tr45;
		case 119u: goto tr46;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto tr40;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto tr40;
	} else
		goto tr40;
	goto tr13;
tr69:
#line 53 "colonize.rl"
	{ putchar('\n'); }
	goto st26;
st26:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof26;
case 26:
#line 1127 "colonize.c"
	if ( (*( key.p)) == 78u )
		goto st27;
	goto st0;
st0:
 key.cs = 0;
	goto _out;
st27:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof27;
case 27:
	switch( (*( key.p)) ) {
		case 10u: goto tr50;
		case 13u: goto tr51;
		case 32u: goto tr52;
		case 35u: goto tr53;
		case 36u: goto tr54;
		case 95u: goto tr53;
		case 101u: goto tr55;
		case 102u: goto tr56;
		case 105u: goto tr57;
		case 108u: goto tr58;
		case 112u: goto tr59;
		case 119u: goto tr60;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto tr53;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto tr53;
	} else
		goto tr53;
	goto tr49;
tr76:
#line 53 "colonize.rl"
	{ putchar('\n'); }
	goto st28;
st28:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof28;
case 28:
#line 1169 "colonize.c"
	if ( (*( key.p)) == 120u )
		goto st29;
	goto st0;
st29:
	if ( ++( key.p) == ( key.pe) )
		goto _test_eof29;
case 29:
	switch( (*( key.p)) ) {
		case 10u: goto tr28;
		case 13u: goto tr29;
		case 32u: goto st23;
		case 35u: goto tr31;
		case 36u: goto tr32;
		case 95u: goto tr31;
		case 101u: goto tr33;
		case 102u: goto tr34;
		case 105u: goto tr35;
		case 108u: goto tr36;
		case 112u: goto tr37;
		case 119u: goto tr38;
	}
	if ( (*( key.p)) < 65u ) {
		if ( 48u <= (*( key.p)) && (*( key.p)) <= 57u )
			goto tr31;
	} else if ( (*( key.p)) > 90u ) {
		if ( 97u <= (*( key.p)) && (*( key.p)) <= 122u )
			goto tr31;
	} else
		goto tr31;
	goto tr27;
	}
	_test_eof30:  key.cs = 30; goto _test_eof; 
	_test_eof1:  key.cs = 1; goto _test_eof; 
	_test_eof2:  key.cs = 2; goto _test_eof; 
	_test_eof3:  key.cs = 3; goto _test_eof; 
	_test_eof4:  key.cs = 4; goto _test_eof; 
	_test_eof5:  key.cs = 5; goto _test_eof; 
	_test_eof6:  key.cs = 6; goto _test_eof; 
	_test_eof7:  key.cs = 7; goto _test_eof; 
	_test_eof8:  key.cs = 8; goto _test_eof; 
	_test_eof9:  key.cs = 9; goto _test_eof; 
	_test_eof10:  key.cs = 10; goto _test_eof; 
	_test_eof11:  key.cs = 11; goto _test_eof; 
	_test_eof12:  key.cs = 12; goto _test_eof; 
	_test_eof13:  key.cs = 13; goto _test_eof; 
	_test_eof14:  key.cs = 14; goto _test_eof; 
	_test_eof15:  key.cs = 15; goto _test_eof; 
	_test_eof16:  key.cs = 16; goto _test_eof; 
	_test_eof17:  key.cs = 17; goto _test_eof; 
	_test_eof18:  key.cs = 18; goto _test_eof; 
	_test_eof19:  key.cs = 19; goto _test_eof; 
	_test_eof20:  key.cs = 20; goto _test_eof; 
	_test_eof21:  key.cs = 21; goto _test_eof; 
	_test_eof22:  key.cs = 22; goto _test_eof; 
	_test_eof23:  key.cs = 23; goto _test_eof; 
	_test_eof24:  key.cs = 24; goto _test_eof; 
	_test_eof25:  key.cs = 25; goto _test_eof; 
	_test_eof26:  key.cs = 26; goto _test_eof; 
	_test_eof27:  key.cs = 27; goto _test_eof; 
	_test_eof28:  key.cs = 28; goto _test_eof; 
	_test_eof29:  key.cs = 29; goto _test_eof; 

	_test_eof: {}
	if ( ( key.p) == ( key.eof) )
	{
	switch (  key.cs ) {
	case 30: 
#line 63 "colonize.rl"
	{ done++; }
#line 53 "colonize.rl"
	{ putchar('\n'); }
	break;
#line 1242 "colonize.c"
	}
	}

	_out: {}
	}

#line 87 "colonize.rl"
}

int main(int argc, char *argv[])
{
    if (argc != 4) return 1;
    wants = *argv[3]-'0';

    le_bold = argv[2];

    FILE *f = fopen(argv[1], "rb");
    if (!f) return 2;
    n = fread(buf,1,sizeof(buf),f);
    if (ferror(f)) return 3;
    fclose(f);

    {
        int cs;
        unsigned char *p = buf, *pe = buf+n;
        
#line 1269 "colonize.c"
static const int lead_start = 3;


#line 1273 "colonize.c"
	{
	cs = lead_start;
	}

#line 1278 "colonize.c"
	{
	if ( p == pe )
		goto _test_eof;
	switch ( cs )
	{
tr4:
#line 108 "colonize.rl"
	{ lead++; }
	goto st3;
st3:
	if ( ++p == pe )
		goto _test_eof3;
case 3:
#line 1292 "colonize.c"
	switch( (*p) ) {
		case 10u: goto st3;
		case 120u: goto st1;
	}
	goto st0;
tr3:
#line 108 "colonize.rl"
	{ lead++; }
	goto st0;
st0:
	if ( ++p == pe )
		goto _test_eof0;
case 0:
#line 1306 "colonize.c"
	if ( (*p) == 10u )
		goto st3;
	goto st0;
st1:
	if ( ++p == pe )
		goto _test_eof1;
case 1:
	switch( (*p) ) {
		case 10u: goto st3;
		case 120u: goto st2;
	}
	goto st0;
st2:
	if ( ++p == pe )
		goto _test_eof2;
case 2:
	if ( (*p) == 10u )
		goto tr4;
	goto tr3;
	}
	_test_eof3: cs = 3; goto _test_eof; 
	_test_eof0: cs = 0; goto _test_eof; 
	_test_eof1: cs = 1; goto _test_eof; 
	_test_eof2: cs = 2; goto _test_eof; 

	_test_eof: {}
	}

#line 113 "colonize.rl"

        lead = 1 + (lead >= 10) + (lead >= 100);
    }

    cnt.p = key.p = buf;
    cnt.pe = key.pe = cnt.eof = key.eof = buf+n;

    
#line 121 "colonize.rl"
    
#line 1346 "colonize.c"
	{
	 cnt.cs = count_start;
	}

#line 122 "colonize.rl"
    
#line 123 "colonize.rl"
    
#line 1355 "colonize.c"
	{
	 key.cs = process_start;
	}

#line 124 "colonize.rl"

    while (!done)
    {
        static _Bool once = 0;
        printf("%sT{\n.nf", once++ ? "\t" : "");
        numerize();
        printf("T}\tT{\n.nf");
        work();
        printf("T}");
    }

    return 0;
}
