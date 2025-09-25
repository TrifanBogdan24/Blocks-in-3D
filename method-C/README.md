# Blocks in 3D

Structura proiectului:

- `chunk_gen.c`:
  - Validare punct în spațiu
  - Amplasare bloc
  - Generare cuboid/sferă  
- `chunk_process.c`:
  - Algoritmi de umplere
  - Creare înveliș  
- `chunk_transform.c`:
  - Rotație 90° pe axa Oy (plan xOz)
  - TODO: gravitație  
- `chunk_compress.c`: compresia/decompresia matricii 3D într-un șir de octeți  

## Creare cuboid



```c
for (int x = MIN(x0, x1); x <= MAX(x0, x1); x++)
  for (int y = MIN(y0, y1); y <= MAX(y0, y1); y++)
    for (int z = MIN(z0, z1); z <= MAX(z0, z1); z++)
``` 

## Creare sferă



```c
int r = (int) ceil(radius);

for (int i = -r; i <= r; i++)
  for (int j = -r; j <= r; j++)
    for (int k = -r; k <= r; k++) {
      double dist = euclidian_dist(x, y, z, x + i, y + j, z + k);

      if (dist > radius) continue;
      chunk_place_block(...)
    }
```


## Înveliș

Dacă `target_block` = `shell_block`, algoritmul clasic ar umple matricea complet.  
Pentru a evita această problemă, folosesc o **coadă de coordonate (X,Y,Z)**,
având o implementare minimală sub forma unui vector de puncte 3D.  

Parcurgând matricea, coordonatele fiecarărui `target_block` se adaugă la finalul cozii
(alocând memorie dinamic cu `realloc` pentru noul punct). 

Apoi, pentru fiecare block, functia `wrapper` plasează `shell_block` în locul vecinilor diferiți de `target_block`.

Pentru a înveli un bloc, este nevoie de a verifica 8 puncte alăturate:
**(x±1, y±1, z±1)**.



## Gravitație (TODO)

Chiar dacă algoritmul nu este corect, iată care sunt pașii:
- Am definit o structură în care am memorat coordonatele tuturor punctelor dintr-un corp
- Un corp se obține în urma rulării unui algoritm de umplere (**Fill**),
  blocurile se vor înlocui cu `BLOCK_AIR` pe masură ce vecinii sunt parcurși
- Se calculează distanța de cădere pentru fiecare block în parte
- La final, corpurile sunt repoziționate pe axa **Oy**
- Âtâta timp cât planul superior (paralel cu xOz) este gol: micșorez înălțimea





