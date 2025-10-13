#include <assert.h>
#include <fcntl.h>
#include <stdio.h>

#include "movies.h"

// Usage: usage: raw <anime> <min> <max>
// First frame is 1

int main(int argc, char *arg[])
{
    if (argc != 4)
        return 1;

    struct movie *chosen = NULL;

    for (int i = 0; i < N_MOVIE; i++)
        if (!strcmp(argv[1], movies[i].id))
        {
            chosen = movies + i;
            break;
        }

    if (!chosen)
        return 2;

    int min = atoi(argv[2]);
    int max = atoi(argv[3]);

    if (min <= 0)
        return 3;
    if (min > chosen->n_frame)
        return 4;
    min--;

    if (0 == max)
        return 5;
    if (max < 0)
    {
        max += chosen->n_frame;
        if (max < 1)
            return 6;
    }
    else if (max > chosen->n_frame)
        return 7;
    max--;

    if (min > max)
        return 8;

    if (-1 == setmode(fileno(stdout), O_BINARY))
    {
        perror(NULL);
        return 9;
    }

    int total = chosen->width * chosen->height;
    int pitch = chosen->width * 4;

    static unsigned char buffer[1000 * 1000];
    assert(sizeof (buffer) < total);

    MOVIE_INIT(chosen);

    for (int i = min; i < max; i++)
    {
        MOVIE_RENDER(chosen, buffer, pitch, i);
        if (fwrite(buffer, 1, total, stdout) != total)
        {
            perror(NULL);
            return 10;
        }
    }

    MOVIE_FREE(chosen);

    return 0;
}
