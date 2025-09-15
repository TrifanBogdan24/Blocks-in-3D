#include "chunk.h"
#include <stdio.h>
#include <stdlib.h>

#define LENGTH_D 18


void wrapper(char*** chunk, int width, int height, int depth,
    int x, int y, int z,
    char target_block, char shell_block) {
    for (int i = -1; i <= 1; i++) {
        for (int j = -1; j <= 1; j++) {
            for (int k = -1; k <= 1; k++) {
                // Exclude colturile:
                if (i * j * k != 0)
                    continue;
                // Exclude poizitia centrala:
                if (i == 0 && j == 0 && k == 0)
                    continue;
                if (!is_inside(width, height, depth, x + i, y + j, z + k))
                    continue;
                if (chunk[x + i][y + j][z + k] == target_block)
                    continue;
                chunk[x + i][y + j][z + k] = shell_block;
            }
        }
    }
}

char*** chunk_shell(
    char*** chunk, int width, int height, int depth,
    char target_block, char shell_block) {
    int **queue_points = NULL;
    int num_points = 0;
    for (int x = 0; x < width; x++) {
        for (int y = 0; y < height; y++) {
            for (int z = 0; z < depth; z++) {
                if (chunk[x][y][z] != target_block)
                    continue;
                num_points += 1;
                queue_points = realloc(queue_points, num_points * sizeof(int **));
                queue_points[num_points - 1] = malloc(3 * sizeof(int));
                queue_points[num_points - 1][0] = x;
                queue_points[num_points - 1][1] = y;
                queue_points[num_points - 1][2] = z;
            }
        }
    }

    for (int i = 0; i < num_points; i++) {
        int px = queue_points[i][0];
        int py = queue_points[i][1];
        int pz = queue_points[i][2];
        wrapper(chunk, width, height, depth,
            px, py, pz,
            target_block, shell_block);
        free(queue_points[i]);
    }

    free(queue_points);
    return chunk;
}


void fill_algorithm_xOz(char*** chunk, int width, int height, int depth,
    int x, int y, int z, int target_block, int new_block) {
    if (!is_inside(width, height, depth, x, y, z))
        return;
    
    if (chunk[x][y][z] != target_block)
        return;
    chunk[x][y][z] = new_block;
    
    int dx[] = {0, 0, -1, 1};
    int dz[] = {-1, 1, 0, 0};

    for (int i = 0; i < 4; i++) {
        fill_algorithm_xOz(chunk, width, height, depth,
            x+dx[i], y, z+dz[i], target_block, new_block);
    }
}


char*** chunk_fill_xz(
    char*** chunk, int width, int height, int depth,
    int x, int y, int z, char block) {
    if (!is_inside(width, height, depth, x, y, z))
        return chunk;
    
    fill_algorithm_xOz(chunk, width, height, depth, x, y, z, chunk[x][y][z], block);
    return chunk;
}


void fill_algorithm_3D(char*** chunk, int width, int height, int depth,
    int x, int y, int z, int target_block, int new_block) {
    if (!is_inside(width, height, depth, x, y, z))
        return;
    
    if (chunk[x][y][z] != target_block)
        return;
    chunk[x][y][z] = new_block;
    
    int dx[] = {0, 0, 0, 0, -1, 1};
    int dy[] = {0, 0, -1, 1, 0, 0};
    int dz[] = {-1, 1, 0, 0, 0, 0};

    for (int i = 0; i < 6; i++) {
        fill_algorithm_3D(chunk, width, height, depth,
            x+dx[i], y+dy[i], z+dz[i], target_block, new_block);
    }
}

char*** chunk_fill(
    char*** chunk, int width, int height, int depth,
    int x, int y, int z, char block) {
    if (!is_inside(width, height, depth, x, y, z))
        return chunk;

    if (chunk[x][y][z] == block)
        return chunk;
    fill_algorithm_3D(chunk, width, height, depth, x, y, z, chunk[x][y][z], block);
    return chunk;
}
