#include "chunk.h"

char*** chunk_rotate_y(char*** chunk, int width, int height, int depth) {
    char ***tmp = (char ***) malloc(depth * sizeof(char **));
    for (int x = 0; x < depth; x++) {
        tmp[x] = (char **) malloc(height * sizeof(char *));
        for (int y = 0; y < height; y++) {
            tmp[x][y] = (char *) malloc(width * sizeof(char));
            for (int z = 0; z < width; z++) {
                tmp[x][y][z] = chunk[z][y][depth - 1 - x];
            }
        }
    }

    return tmp;
}




char*** chunk_apply_gravity(
    char*** chunk, int width, int height, int depth, int* new_height) {
    return chunk;
}

