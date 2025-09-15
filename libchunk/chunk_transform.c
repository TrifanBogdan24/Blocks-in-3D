#include "chunk.h"
#include <stdlib.h>

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

    for (int i = 0; i < 6; i++) {
        fill_corp_with_air(
            chunk, width, height, depth,
            x + dx[i], y + dy[i], z + dz[i],
            corp, target
        );
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
                fill_corp_with_air(chunk, width, height, depth,x, y, z, &corp, chunk[x][y][z]);

                (*num_corps) += 1;
                corps = realloc(corps, (*num_corps) * sizeof(Corp));
                corps[(*num_corps) - 1] = corp;
            }
        }
    }

    return corps;
}


void place_corp(char*** chunk, Corp *corp, char block) {
    for (int i = 0; i < corp->num_points; i++) {
        int x = corp->points[i].x;
        int y = corp->points[i].y;
        int z = corp->points[i].z;
        chunk[x][y][z] = block;
    }
}


int compute_fall_distance(Corp *corp, char*** chunk, int width, int height, int depth) {
    int max_fall = height; // mai mare decât orice posibil
    
    for (int i = 0; i < corp->num_points; i++) {
        int x = corp->points[i].x;
        int y = corp->points[i].y;
        int z = corp->points[i].z;

        int dist = 0;
        int yy = y - 1;
        while (yy >= 0 && chunk[x][yy][z] == BLOCK_AIR) {
            dist++;
            yy--;
        }
        if (dist < max_fall)
            max_fall = dist;
    }
    return max_fall;
}


int corp_min_y(Corp *corp) {
    int m = corp->points[0].y;
    for (int i = 1; i < corp->num_points; i++) {
        if (corp->points[i].y < m)
            m = corp->points[i].y;
    }
    return m;
}

int cmp_corps(const void *a, const void *b) {
    const Corp *ca = (const Corp*)a;
    const Corp *cb = (const Corp*)b;
    return corp_min_y((Corp*)a) - corp_min_y((Corp*)b);
}

char*** chunk_apply_gravity(
    char*** chunk, int width, int height, int depth, int* new_height) {
    
    int num_corps = 0;
    Corp *corps = get_corps(chunk, width, height, depth, &num_corps);
    qsort(corps, num_corps, sizeof(Corp), cmp_corps);

    for (int c = 0; c < num_corps; c++) {
        Corp *corp = &corps[c];

        // află tipul de bloc (toate punctele corpului au fost aer acum!)
        char block_type = corp->block; // trebuie salvat în struct când scoți corpul

        int fall = compute_fall_distance(corp, chunk, width, height, depth);

        for (int i = 0; i < corp->num_points; i++) {
            corp->points[i].y -= fall;
        }

        place_corp(chunk, corp, block_type);
    }

    // elimină straturile goale de sus
    int top = height - 1;
    while (top >= 0) {
        int empty = 1;
        for (int x = 0; x < width && empty; x++) {
            for (int z = 0; z < depth && empty; z++) {
                if (chunk[x][top][z] != BLOCK_AIR)
                    empty = 0;
            }
        }
        if (!empty) break;
        top--;
    }

    *new_height = top + 1; // noua înălțime
    return chunk;
}

