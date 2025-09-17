#include "chunk.h"
#include <stdio.h>
#include <stdlib.h>

#define B0 0
#define B1 1
#define B2 2
#define B3 3
#define B4 4
#define B5 5
#define B6 6
#define B7 7
#define B8 8
#define B9 9
#define B10 10
#define B11 11

#define MAX_NUM_OCCURRENCES 4095


void print_byte(unsigned char byte) {
    for (int i = B7; i >= 0; i--)
        printf("%d", (byte & (1 << i)) ? 1 : 0);
}

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

            if (num_occurrences == MAX_NUM_OCCURRENCES) {
                (*num_pairs)++;
                pairs = realloc(pairs, (*num_pairs) * sizeof(Pair));
                pairs[(*num_pairs) - 1].block = block;
                pairs[(*num_pairs) - 1].num_occurrences = num_occurrences;
                num_occurrences = 0;
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



void add_pair_to_bytes(unsigned char **bytes, int *length,
    Pair pair) {
    int num_occurrences = pair.num_occurrences;
    char block = pair.block;


    (*length) += 1;
    *bytes = realloc(*bytes, (*length) * sizeof(unsigned char));
    (*bytes)[*length - 1] = 0;

    // b1, b0
    if (block == BLOCK_GRASS) {
        (*bytes)[*length - 1] |= (1 << B6);
    } else if (block == BLOCK_WOOD) {
        (*bytes)[*length - 1] |= (1 << B7);
    } else if (block == BLOCK_STONE) {
        (*bytes)[*length - 1] |= (1 << B7);
        (*bytes)[*length - 1] |= (1 << B6);
    }

    if (num_occurrences < (1 << B5)) {
        // bb0nnnnn
        if (num_occurrences & (1 << B4)) (*bytes)[*length - 1] |= (1 << B4);
        if (num_occurrences & (1 << B3)) (*bytes)[*length - 1] |= (1 << B3);
        if (num_occurrences & (1 << B2)) (*bytes)[*length - 1] |= (1 << B2);
        if (num_occurrences & (1 << B1)) (*bytes)[*length - 1] |= (1 << B1);
        if (num_occurrences & (1 << B0)) (*bytes)[*length - 1] |= (1 << B0);
    } else {
        // bb10nnnn nnnnnnnn
        (*bytes)[*length - 1] |= (1 << B5);

        (*length) += 1;
        *bytes = realloc(*bytes, (*length) * sizeof(unsigned char));
        (*bytes)[*length - 1] = 0;

        // Primul octet:
        if (num_occurrences & (1 << B11)) (*bytes)[*length - 2] |= (1 << B3);
        if (num_occurrences & (1 << B10)) (*bytes)[*length - 2] |= (1 << B2);
        if (num_occurrences & (1 << B9))  (*bytes)[*length - 2] |= (1 << B1);
        if (num_occurrences & (1 << B8))  (*bytes)[*length - 2] |= (1 << B0);

        // Al doilea octet:
        if (num_occurrences & (1 << B7)) (*bytes)[*length - 1] |= (1 << B7);
        if (num_occurrences & (1 << B6)) (*bytes)[*length - 1] |= (1 << B6);
        if (num_occurrences & (1 << B5)) (*bytes)[*length - 1] |= (1 << B5);
        if (num_occurrences & (1 << B4)) (*bytes)[*length - 1] |= (1 << B4);
        if (num_occurrences & (1 << B3)) (*bytes)[*length - 1] |= (1 << B3);
        if (num_occurrences & (1 << B2)) (*bytes)[*length - 1] |= (1 << B2);
        if (num_occurrences & (1 << B1)) (*bytes)[*length - 1] |= (1 << B1);
        if (num_occurrences & (1 << B0)) (*bytes)[*length - 1] |= (1 << B0);
    }
}



unsigned char* chunk_encode(
    char*** chunk, int width, int height, int depth,
    int* length) {
    int flat_arr_len = 0, num_pairs = 0;
    char *flat_arr = flatten(chunk, width, height, depth, &flat_arr_len);
    Pair *pairs = get_pairs(flat_arr, flat_arr_len, &num_pairs);
    free(flat_arr);

    (*length) = 0;
    unsigned char *bytes = NULL;

    for (int i = 0; i < num_pairs; i++) {
        add_pair_to_bytes(&bytes, length, pairs[i]);
    }

    free(pairs);
    return bytes;
}



char*** chunk_decode(
    unsigned char* bytes, int width, int height, int depth) {
    char ***chunk = malloc(width * sizeof(char **));
    for (int x = 0; x < width; x++) {
        chunk[x] = malloc(height * sizeof(char *));
        for (int y = 0; y < height; y++) {
            chunk[x][y] = malloc(depth * sizeof(char));
            for (int z = 0; z < depth; z++)
                chunk[x][y][z] = BLOCK_AIR;
        }
    }


    int x = 0;
    int y = 0;
    int z = 0;
    unsigned char *ptr = bytes;

    while (*ptr) {
        unsigned char byte = 0;
        char block = BLOCK_AIR;
        int num_occurrences = 0;

        byte = *ptr;
        ptr++;
        if ((byte & (1 << B7)) && (byte & (1 << B6))) {
            // b1b0 = 11
            block = BLOCK_STONE;
        } else if ((byte & (1 << B7)) && !(byte & (1 << B6))) {
            // b1b0 = 10
            block = BLOCK_WOOD;
        } else if (!(byte & (1 << B7)) && (byte & (1 << B6))) {
            // b1b0 = 01
            block = BLOCK_GRASS;
        } else {
            // b1b0 = 00
            block = BLOCK_AIR;
        }

        if (!(byte & (1 << B5))) {
            // bb0nnnnn
            if (byte & (1 << B4)) num_occurrences |= (1 << B4);
            if (byte & (1 << B3)) num_occurrences |= (1 << B3);
            if (byte & (1 << B2)) num_occurrences |= (1 << B2);
            if (byte & (1 << B1)) num_occurrences |= (1 << B1);
            if (byte & (1 << B0)) num_occurrences |= (1 << B0);
        } else if ((byte & (1 << B5)) && !(byte & (1 << B4))) {
            // bb10nnnn nnnnnnnn

            // Primul octet:
            if (byte & (1 << B3)) num_occurrences |= (1 << B11);
            if (byte & (1 << B2)) num_occurrences |= (1 << B10);
            if (byte & (1 << B1)) num_occurrences |= (1 << B9);
            if (byte & (1 << B0)) num_occurrences |= (1 << B8);

            // Al doilea octet:
            byte = *ptr;
            ptr++;

            if (byte & (1 << B7)) num_occurrences |= (1 << B7);
            if (byte & (1 << B6)) num_occurrences |= (1 << B6);
            if (byte & (1 << B5)) num_occurrences |= (1 << B5);
            if (byte & (1 << B4)) num_occurrences |= (1 << B4);
            if (byte & (1 << B3)) num_occurrences |= (1 << B3);
            if (byte & (1 << B2)) num_occurrences |= (1 << B2);
            if (byte & (1 << B1)) num_occurrences |= (1 << B1);
            if (byte & (1 << B0)) num_occurrences |= (1 << B0);
        }

        for (int i = 0; i < num_occurrences; i++) {
            chunk[x][y][z] = block;

            x++;
            if (x == width) {
                x = 0;
                z++;
            }

            if (z == depth) {
                z = 0;
                y++;
            }

            if (y == height) {
                return chunk;
            }
        }
    }
    return chunk;
}
