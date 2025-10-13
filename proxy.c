#include <fcntl.h>
#include <stdio.h>

#define DIE(N) do { perror("proxy"); return (N); } while(0)

int main(int argc, char *argv[])
{
    if (argc <= 1) return 1;

    if (-1 == setmode(fileno(stdin), O_BINARY)) DIE(2);

    FILE *output;

    if (!(output = fopen(argv[argc-1], "wb"))) DIE(3);

    for (int c; (c = getchar()) != EOF; )
        if (fputc(c, output) == EOF) DIE(4);

    if (ferror(stdin)) DIE(5);

    if (fclose(output)) DIE(6);

    return 0;
}
