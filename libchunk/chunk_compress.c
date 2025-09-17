#include "chunk.h"
#include <stdio.h>
#include <stdlib.h>


void print_byte(unsigned char byte) {
    for (int i = 7; i >= 0; i--)
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

            if (num_occurrences == 4095) {
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

        add_pair_to_bytes(&bytes, length, pairs[i].num_occurrences, pairs[i].block);
    }

    free(pairs);
    for (int i = 0; i < *length; i++) {
        printf("%2x\n", bytes[i]);
    }
    return bytes;
}

void print_chunk_xz_planes(char ***chunk, int width, int height, int depth) {
    for (int y = 0; y < height; y++) {
        for (int z = 0; z < depth; z++) {
            for (int x = 0; x < width; x++)
                printf("%d ", chunk[x][y][z]);
            printf("\n");
        }
        printf("\n");
    }
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
        int num_occurences = 0;

        byte = *ptr;
        ptr++;
        if ((byte & (1 << 7)) && (byte & (1 << 6))) {
            // b1b0 = 11
            block = BLOCK_STONE;
        } else if ((byte & (1 << 7)) && !(byte & (1 << 6))) {
            // b1b0 = 10
            block = BLOCK_WOOD;
        } else if (!(byte & (1 << 7)) && (byte & (1 << 6))) {
            // b1b0 = 01
            block = BLOCK_GRASS;
        } else {
            // b1b0 = 00
            block = BLOCK_AIR;
        }

        if (!(byte & (1 << 5))) {
            // bb0nnnnn
            if (byte & (1 << 4)) num_occurences |= (1 << 4);
            if (byte & (1 << 3)) num_occurences |= (1 << 3);
            if (byte & (1 << 2)) num_occurences |= (1 << 2);
            if (byte & (1 << 1)) num_occurences |= (1 << 1);
            if (byte & (1 << 0)) num_occurences |= (1 << 0);
        } else if ((byte & (1 << 5)) && !(byte & (1 << 4))) {
            // bb10nnnn nnnnnnnn

            // In primul octet:
            if (byte & (1 << 3)) num_occurences |= (1 << 11);
            if (byte & (1 << 2)) num_occurences |= (1 << 10);
            if (byte & (1 << 1)) num_occurences |= (1 << 9);
            if (byte & (1 << 0)) num_occurences |= (1 << 8);

            // In al doilea octet:
            byte = *ptr;
            ptr++;

            if (byte & (1 << 7)) num_occurences |= (1 << 7);
            if (byte & (1 << 6)) num_occurences |= (1 << 6);
            if (byte & (1 << 5)) num_occurences |= (1 << 5);
            if (byte & (1 << 4)) num_occurences |= (1 << 4);
            if (byte & (1 << 3)) num_occurences |= (1 << 3);
            if (byte & (1 << 2)) num_occurences |= (1 << 2);
            if (byte & (1 << 1)) num_occurences |= (1 << 1);
            if (byte & (1 << 0)) num_occurences |= (1 << 0);
        }

        for (int i = 0; i < num_occurences; i++) {
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

