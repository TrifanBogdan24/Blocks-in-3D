#include "chunk.h"
#include <stdio.h>
#include <stdlib.h>

char *flatten(char*** chunk, int width, int height, int depth, int *length) {
    (*length) = width * height * depth;
    char *array = (char *) malloc((*length) * sizeof(char));
    for (int y = 0; y < height; y++) {
        for (int z = 0; z < depth; z++) {
            for (int x = 0; x < width; x++) {
                int idx = y * (depth * width) + z * width + x;
                array[idx] = chunk[x][y][z];
            }
        }
    }
    return array;
}

typedef struct {
    int num_occurrences;
    char block;
} Pair;

Pair *get_pairs(char *array, int length, int *num_pairs) {
    (*num_pairs) = 0;
    Pair *pairs = NULL;

    int idx = 0;
    while (idx < length) {
        char block = array[idx];
        int num_occurrences = 0;

        while (idx < length && array[idx] == block) {
            idx++;
            num_occurrences++;

            if (num_occurrences == 4095) {
                (*num_pairs)++;
                pairs = realloc(pairs, (*num_pairs) * sizeof(Pair));
                pairs[(*num_pairs) - 1].block = block;
                pairs[(*num_pairs) - 1].num_occurrences = num_occurrences;
                num_occurrences = 0;
                continue;
            }
        }

        if (num_occurrences > 0) {
            (*num_pairs)++;
            pairs = realloc(pairs, (*num_pairs) * sizeof(Pair));
            pairs[(*num_pairs) - 1].block = block;
            pairs[(*num_pairs) - 1].num_occurrences = num_occurrences;
        }
    }

    return pairs;
}




void or_btye_block(unsigned char *byte, char block) {
    if (!byte)
        return;

    // b1, b0
    if (block == BLOCK_AIR || block == BLOCK_STONE) {
        (*byte) |= block;
    } else if (block == BLOCK_GRASS) {
        (*byte) |= (1 << 1);
    } else if (block == BLOCK_WOOD) {
        (*byte) |= 1;
    }
}

void add_pair_to_bytes(unsigned char **bytes, int *length,
    int num_occurences, char block) {
    if (num_occurences < 32) {
        (*length) += 1;
        *bytes = realloc(*bytes, (*length) * sizeof(unsigned char));
        (*bytes)[*length - 1] = 0;

        or_btye_block(&(*bytes)[*length - 1], block);
        
        for (int i = 4; i >= 0; i--) {
            if (num_occurences >= 1 << i) {
                num_occurences -= 1 << i;
                (*bytes)[*length - 1] |= (1 << (7 - i));
            }
        }
    } else {
        (*length) += 2;
        *bytes = realloc(*bytes, (*length) * sizeof(unsigned char));
        (*bytes)[*length - 2] = 0;
        (*bytes)[*length - 1] = 0;
        or_btye_block(&(*bytes)[*length - 2], block);

        (*bytes)[*length - 2] |= (1 << 2);

        for (int i = 11; i >= 0; i--) {
            if (num_occurences >= 1 << i) {
                num_occurences -= 1 << i;

                if (15 - i >= 8) {
                    (*bytes)[*length - 1] |= 1 << (7 - i);
                } else {
                    (*bytes)[*length - 2] |= 1 << (15 - i);
                }
            }
        }
    }
}



unsigned char* chunk_encode(
    char*** chunk, int width, int height, int depth,
    int* length) {
    int flat_arr_len = 0, num_pairs = 0;
    char *flat_arr = flatten(chunk, width, height, depth, &flat_arr_len);
    Pair *pairs = get_pairs(flat_arr, flat_arr_len, &num_pairs);
    free(flat_arr);

    // TODO: continue
    (*length) = 0;
    unsigned char *bytes = NULL;

    for (int i = 0; i < num_pairs; i++) {
        int num_occurrences = pairs[i].num_occurrences;
        char block = pairs[i].block;

        add_pair_to_bytes(&bytes, &length, pairs[i].num_occurrences, pairs[i].block);
    }

    free(pairs);
    return bytes;
}

char*** chunk_decode(
    unsigned char* code, int width, int height, int depth) {
    return NULL;
}

