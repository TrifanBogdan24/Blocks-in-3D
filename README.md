# Blocks in 3D

Structura proiectului:

- `chunk_gen.c`:
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

Funcția primește 2 colțuri opuse ale unui paralelipiped dreptunghic.  
Coordonatele de iterație pornesc de la minimul fiecărei axe și merg până la maxim.  
Astfel, chiar dacă ordinea colțurilor este arbitrară, volumul este parcurs corect. 

```c
for (int x = MIN(x0, x1); x <= MAX(x0, x1); x++)
  for (int y = MIN(y0, y1); y <= MAX(y0, y1); y++)
    for (int z = MIN(z0, z1); z <= MAX(z0, z1); z++)
``` 

## Creare sferă

Notez cu `r` = întregul cel mai mare (`ceil`) la care se rotunjeste raza.
o raza de **-1.2** se va rotunji la **-2**, iar pentru **1.2** `r` va fi egal cu **2**.

Pentru fiecare offset din intervalul `[-r, r]` pe cele 3 axe, se calculează distanța euclidiană
față de centru. Dacă distanța ≤ raza reală, plasez blocul; altfel, îl ignor.  

> **Distanta euclidiana** se calculeaza (folosind functiile `pow` si `sqrt` din biblioteca `math.h`)
> ca suma patratelor diferentelor coordontalor pe cele 3 axe.
>
> `dist(P1, P2) = sqrt((P1.x - P2.x)^ 2 + (P1.y - P2.y)^ 2 + (P1.z - P2.z)^2 )`

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


## Fill

Algoritm recursiv de umplere: pornește dintr-un punct și vizitează vecinii de același tip,
fără să mai fie nevoie de memorie suplimentară pentru marcarea blocurilor vizitate.

Cazul în care `target_block` este egal cu `new_block` este tratat separat
inaintea rularii algoritmului de umplere: se va returna matricea intiala.

## Rotirea în jurul axei Oy

Se alocă o nouă matrice:
- `new_width = old_depth`
- `new_depth = old_width`

> Practic interschimbă dimensiunile pentru **lățime** și **adâncime**.


Valorile sunt copiate conform regulii:  
```c
new_mat[x][y][z] = chunk[z][y][depth - 1 - x]
```

## Gravitație (TODO)

Chiar dacă algoritmul nu este corect, iată care sunt pașii:
- Am definit o structură în care am memorat coordonatele tuturor punctelor dintr-un corp
- Un corp se obține în urma rulării unui algoritm de umplere (**Fill**),
  blocurile se vor înlocui cu `BLOCK_AIR` pe masură ce vecinii sunt parcurși
- Se calculează distanța de cădere pentru fiecare block în parte
- La final, corpurile sunt repoziționate pe axa **Oy**
- Âtâta timp cât planul superior (paralel cu xOz) este gol: micșorez înălțimea

## Compresie

1. Serializare: matricea 3D este aplatizată într-un vector liniar (ordinea **y->z->x**).  
2. Vectorul e parcurs pentru a determina secvențe de blocuri identice consecutive: 
   - Se generează un vector de perechi `(num_occurrences, tip_block)`
   - Dacă numărul depășește 4095, se începe un nou **run**
3. Transformarea în octeți:  
   - <32 apariții => codificare pe 1 octet  
   - ≥32 apariții => codificare pe 2 octeți  

Pentru că dimensiunea finală este necunoscută,
buffer-ul se redimensionează dinamic.  

## Decompresie

1. Se alocă matricea 3D pe baza dimensiunilor cunoscute.  
2. Se parcurg octeții cu un pointer, reconstruind blocurile.  
3. Decodificarea se bazează pe markerii din biții 5 și 6:  
   - `0` => **run** pe 1 octet (5 biți pentru numărul de aparitii)  
   - `10` => **run** pe 2 octeți (11 biți pentru număr de aparitii)  

Coordonatele **(x,y,z)** sunt actualizate pe măsură ce vectorul este reconstruit.  

## Operații pe biți

- Putere a lui 2: `1 << n`  
- Setare bit: `byte |= (1 << i)`  
- Verificare valoare (0/1) bit: `if (byte & (1 << i))`  

## Semnificația biților

| Interval n    | Primul octet (MSB -> LSB)| Al doilea octet (MSB -> LSB) |
|:--------------|:-------------------------|:-----------------------------|
| 1 ≤ n < 32    | b1 b0 0 n4 n3 n2 n1 n0   | -                            |
| 32 ≤ n < 4096 | b1 b0 1 0 n11 n10 n9 n8  | n7 n6 n5 n4 n3 n2 n1 n0      |

- `b1 b0` = tipul blocului (cei mai semnificativi 2 biți)  
- `0` / `10` = markeri pentru lungimea run-ului (1 sau 2 octeți)  
- `nX` = biții care codifică numărul de apariții  
- **MSB** = primul bit din stânga al unui octet  
- **LSB** = ultimul bit din dreapta al unui octet  

Un octet codifică atât tipul blocului, cât și numărul de apariții.  
- 1 ≤ n < 32 => codificare pe 1 octet  
- 32 ≤ n < 4096 => codificare pe 2 octeți  

Bitul 6 este markerul:  
- `0` => 1 octet  
- `10` => 2 octeți  
Tipul blocului ocupă cei mai semnificativi 2 biți.

