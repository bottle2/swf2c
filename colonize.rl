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

%%{
    machine count;
    alphtype unsigned char;
    access cnt.;
    variable p cnt.p;
    variable pe cnt.pe;
    variable eof cnt.eof;

    action newl { putchar('\n'); }
    action stop { fbreak; }
    action numbah {
        ln++;
        char *rec = "\\0\\0" + (6 - lead*2 + (ln >= 10)*2 + (ln >= 100)*2);
        printf("%s%d", rec, ln);
    }

    main := ((("xx" %numbah | "NN" %stop)? any*) >newl :>> ('\r'? '\n'))* %newl;

    write data noerror nofinal noentry;
}%%

void numerize(void) { %% write exec;
}

%%{
    machine process;
    alphtype unsigned char;
    access key.;
    variable p key.p;
    variable pe key.pe;
    variable eof key.eof;

    action stop { fbreak; }
    action indent { putchar('\\'); putchar('0'); }

    action newl { putchar('\n'); }
    action bold {
        if (wants)
            printf("%s%.*s\\fP", le_bold, (int)(fpc - m), m);
        else
            printf("%.*s", (int)(fpc - m), m);
        m = fpc;
    }
    action dump { printf("%.*s", (int)(fpc - m), m); m = fpc; }
    action mark { m = fpc; }
    action end { done++; }

    keyword3 = "procedure" | "for" | "if" | "else" | "while" | "loop";
#             | ('#' ' '* ("ifndef" | "define" | "ifdef" | "include" | "endif"));

    #keyword3 = "if";
    keyword2 = keyword3 >dump;

    nid = [^#_a-zA-Z0-9];

    line3 = keyword2 %bold
          | (keyword2 nid >bold)? (any | nid keyword2 nid >bold)* (nid keyword2 %bold)?
          ;
    line2 = ("$ " %bold)? line3;

    line = (("xx" | "NN" %stop) ' '?)? <: ' '* $indent <: line2? >mark %dump;
    main := (line >newl :>> ('\r'? '\n'))* %end %newl;

    write data noerror nofinal noentry;
}%%

void work(void) {
    unsigned char *m;
    %% write exec;
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
        %%{
            machine lead;
            alphtype unsigned char;
            action cnt { lead++; }
            main := (("xx" %cnt)? any* :>> ('\r'? '\n'))*;
            write data noerror nofinal noentry;
            write init;
            write exec;
        }%%
        lead = 1 + (lead >= 10) + (lead >= 100);
    }

    cnt.p = key.p = buf;
    cnt.pe = key.pe = cnt.eof = key.eof = buf+n;

    %% machine count;
    %% write init;
    %% machine process;
    %% write init;

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
