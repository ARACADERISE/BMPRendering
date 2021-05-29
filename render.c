/*
 * Helper file to write info to the .bmp file.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const unsigned char info_header[2] = { 'B', 'M' };
static unsigned int header[] = {
    0, 0x00,
    0x36, 0x28,
    0, 0,
    0x180001,
    0, 0,
    0x002e23, 0x002e23,
    0, 0
};

void assign(int width, int height)
{
    header[4] = width;
    header[5] = height;
}

void write(FILE* file, int width, int height)
{

    char *pixels = calloc(width * height, sizeof(*pixels));

    if(width * height == 1)
    {
        char pix[] = {
            0x35, 0x41, 0xef, 0x00
        };

        fwrite(&pix, sizeof(pix), 1, file);
        fclose(file);
    } else
    {

        for(int c = 0; c < height; c++)
        {
            for(int r = 0; r < width; r++)
            {
                pixels[c + r] = 0x31;
            }
        }

        fwrite(&pixels, (width * height) * sizeof(pixels), 1, file);
        fclose(file);
    }
}

int main(int argc, char **argv)
{
    if(argc < 3)
    {
        fprintf(stderr, "./main.o width height");
    }

    int width = atoi(argv[1]);
    int height = atoi(argv[2]);
    header[0] = 3 * (width * height);

    assign(width, height);

    FILE* file = fopen("img.bmp", "wb");

    fwrite(&info_header, sizeof(info_header), 1, file);

    if(argc >= 3)
    {
        if(strcmp(argv[3], "write") == 0)
        {
            if(width * height == 1) header[0] = 0x36;
            fwrite(&header, sizeof(header), 1, file);

            write(file, width, height);
        }
        FILE* f = fopen("boiler.bmp", "wb");

        fwrite(&info_header, sizeof(info_header), 1, file);
        fwrite(&header, sizeof(header), 1, file);

        fclose(f);
    }

    fwrite(&header, sizeof(header), 1, file);

    fclose(file);
}