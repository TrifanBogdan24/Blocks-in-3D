#include "chunk.h"
#include <stdlib.h>
#include <stdio.h>

#define LEN_DIFF_ARRAY_3D 6


typedef struct {
    int x, y, z;
} Point;

typedef struct {
    Point *points;
    int num_points;
    char block;
} Corp;


char*** chunk_rotate_y(char*** chunk, int width, int height, int depth) {
    char ***new_mat = (char ***) malloc(depth * sizeof(char **));
    for (int x = 0; x < depth; x++) {
        new_mat[x] = (char **) malloc(height * sizeof(char *));
        for (int y = 0; y < height; y++) {
            new_mat[x][y] = (char *) malloc(width * sizeof(char));
            for (int z = 0; z < width; z++) {
                new_mat[x][y][z] = chunk[z][y][depth - 1 - x];
            }
        }
    }
    return new_mat;
}


void fill_corp_with_air(
    char*** chunk, int width, int height, int depth,
    int x, int y, int z,
    Corp *corp, char target) {
    int dx[] = {0, 0, 0, 0, -1, 1};
    int dy[] = {0, 0, -1, 1, 0, 0};
    int dz[] = {-1, 1, 0, 0, 0, 0};

    if (!is_inside(width, height, depth, x, y, z))
        return;
    if (chunk[x][y][z] != target)
        return;

    chunk[x][y][z] = BLOCK_AIR;

    corp->num_points++;
    corp->points = realloc(corp->points, corp->num_points * sizeof(Point));
    int idx = corp->num_points - 1;
    corp->points[idx].x = x;
    corp->points[idx].y = y;
    corp->points[idx].z = z;

    for (int i = 0; i < LEN_DIFF_ARRAY_3D; i++) {
        fill_corp_with_air(
            chunk, width, height, depth,
            x + dx[i], y + dy[i], z + dz[i],
            corp, target);
    }
}


Corp *get_corps(char*** chunk, int width, int height, int depth, int* num_corps) {
    Corp *corps = NULL;
    (*num_corps) = 0;

    for (int x = 0; x < width; x++) {
        for (int y = 0; y < height; y++) {
            for (int z = 0; z < depth; z++) {
                if (chunk[x][y][z] == BLOCK_AIR)
                    continue;
                Corp corp;
                corp.num_points = 0;
                corp.points = NULL;
                corp.block = chunk[x][y][z];
                fill_corp_with_air(chunk, width, height, depth, x, y, z, &corp, chunk[x][y][z]);

                (*num_corps) += 1;
                corps = realloc(corps, (*num_corps) * sizeof(Corp));
                corps[(*num_corps) - 1] = corp;
            }
        }
    }
    return corps;
}


void place_corp(char*** chunk, Corp *corp) {
    for (int i = 0; i < corp->num_points; i++) {
        int x = corp->points[i].x;
        int y = corp->points[i].y;
        int z = corp->points[i].z;
        chunk[x][y][z] = corp->block;
    }
}


// compute fall distance for a corp in presence of all other corps
int compute_fall_distance_global(Corp *corp, Corp *corps, int num_corps, int width, int height, int depth) {
    int min_fall = height;

    for (int i = 0; i < corp->num_points; i++) {
        int x = corp->points[i].x;
        int y = corp->points[i].y;
        int z = corp->points[i].z;

        int dist = 0;
        int yy = y - 1;
        while (yy >= 0) {
            int occupied = 0;

            // check if another corp has a block at (x, yy, z)
            for (int c = 0; c < num_corps && !occupied; c++) {
                Corp *other = &corps[c];
                if (other == corp) continue;
                for (int p = 0; p < other->num_points; p++) {
                    if (other->points[p].x == x &&
                        other->points[p].y == yy &&
                        other->points[p].z == z) {
                        occupied = 1;
                        break;
                    }
                }
            }

            if (occupied) break;

            dist++;
            yy--;
        }

        if (dist < min_fall)
            min_fall = dist;
    }

    return min_fall;
}


int is_plan_filled_with_air(char*** chunk, int width, int height, int depth, int y) {
    if (y < 0 || y >= height) return 0;
    for (int x = 0; x < width; x++)
        for (int z = 0; z < depth; z++)
            if (chunk[x][y][z] != BLOCK_AIR)
                return 0;
    return 1;
}


char*** chunk_apply_gravity(
    char*** chunk, int width, int height, int depth, int* new_height) {

    int num_corps = 0;
    Corp *corps = get_corps(chunk, width, height, depth, &num_corps);

    // compute fall distances for all corps simultaneously
    int *falls = (int*) malloc(num_corps * sizeof(int));
    for (int c = 0; c < num_corps; c++) {
        falls[c] = compute_fall_distance_global(&corps[c], corps, num_corps, width, height, depth);
    }

    // apply fall distances
    for (int c = 0; c < num_corps; c++) {
        for (int i = 0; i < corps[c].num_points; i++) {
            corps[c].points[i].y -= falls[c];
        }
    }

    // clear chunk
    for (int x = 0; x < width; x++)
        for (int y = 0; y < height; y++)
            for (int z = 0; z < depth; z++)
                chunk[x][y][z] = BLOCK_AIR;

    // place all corps back
    for (int c = 0; c < num_corps; c++) {
        place_corp(chunk, &corps[c]);
    }

    // compute new height
    *new_height = height - 1;
    while (is_plan_filled_with_air(chunk, width, height, depth, *new_height))
        (*new_height) -= 1;
    (*new_height) += 1;
    free(falls);
    return chunk;
}
