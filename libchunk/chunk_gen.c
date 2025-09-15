#include <math.h>
#include "chunk.h"

#define MIN(x, y) ((x) < (y) ? (x) : (y))
#define MAX(x, y) ((x) > (y) ? (x) : (y))

#define SQUARE (double) 2.0


int is_inside(int width, int height, int depth,
    int x, int y, int z) {
    return (0 <= x && x < width)
        && (0 <= y && y < height)
        && (0 <= z && z < depth);
}

char*** chunk_place_block(
    char*** chunk, int width, int height, int depth,
    int x, int y, int z, char block) {
    if (!is_inside(width, height, depth, x, y, z))
        return chunk;

    chunk[x][y][z] = block;
    return chunk;
}

char*** chunk_fill_cuboid(
    char*** chunk, int width, int height, int depth,
    int x0, int y0, int z0, int x1, int y1, int z1, char block) {
    for (int x = MIN(x0, x1); x <= MAX(x0, x1); x++)
        for (int y = MIN(y0, y1); y <= MAX(y0, y1); y++)
            for (int z = MIN(z0, z1); z <= MAX(z0, z1); z++)
                chunk_place_block(chunk, width, height, depth, x, y, z, block);
    return chunk;
}


double euclidian_dist(int x0, int y0, int z0, int x1, int y1, int z1) {
    return sqrt(
        pow((double) (x1 - x0), SQUARE)
        + pow((double) (y1 - y0), SQUARE)
        + pow((double) (z1 - z0), SQUARE)
    );
}


char*** chunk_fill_sphere(
    char*** chunk, int width, int height, int depth,
    int x, int y, int z, double radius, char block) {
    for (double i = -radius; i <= radius; i++) {
        for (double j = -radius; j <= radius; j++) {
            for (double k = -radius; k <= radius; k++) {
                double dist =
                    euclidian_dist(x, y, z, x + i, y + j, z + k);
                
                if (dist > radius) continue;
                chunk_place_block(chunk, width, height, depth, x + i, y + j, z + k, block);
            }
        }
    }
    return chunk;
}
