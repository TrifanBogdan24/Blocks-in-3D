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

## 📌 Amplasare bloc

- `is_inside`: verifică dacă un punct **(x, y, z)** se află în interiorul chunk-ului:
  returnează **1** dacă este valid, altfel **0**

- `chunk_place_block`: dacă coordonatele sunt valide, actualizează tipul blocului de la
  poziția respectivă; dacă nu, funcția nu are efect.

## 🧊 Creare cuboid

- Funcția primește două colțuri opuse ale unui paralelipiped dreptunghic.  
- Coordonatele de iterație pornesc de la **MIN** fiecărei axe și merg până la **MAX**.

```c
for (int x = MIN(x0, x1); x <= MAX(x0, x1); x++)
  for (int y = MIN(y0, y1); y <= MAX(y0, y1); y++)
    for (int z = MIN(z0, z1); z <= MAX(z0, z1); z++)
``` 

## 🪩 Creare sferă

Notez cu `r` = întregul cel mai mare (`ceil`) la care se rotunjeste raza.

Pentru fiecare offset din intervalul `[-r, r]` pe cele 3 axe, se calculează distanța euclidiană
față de centru. Dacă distanța ≤ raza reală, plasez blocul; altfel, îl ignor.

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


## 📦 Înveliș

Structura folosită pentru a reține punctele de interes este o **"coadă"**,
implementată minimal ca o matrice bidimensională cu 3 coloane:
- coloana 1 -> coordonata X
- coloana 2 -> coordonata Y
- coloana 3 -> coordonata Z


Inițial, încep cu o matrice este vidă.
Pe măsură ce descopăr puncte valide, aloc dinamic câte o linie nouă cu `realloc`,
completând-o cu valorile corespunzătoare.

Pentru simplitate, nu dublez memoria la atingerea limitei; aloc incremental (linie cu linie).

Iterarea "cozii" se face ca asupra unui vector, în sensul că nu există operații de `.pop()`,
ci doar de `.push()`.

La final, întreaga structură (matricea) este dezalocată.

## Fill

Algoritm recursiv de umplere: pornește dintr-un punct și vizitează vecinii de același tip.

Dacă `target_block == new_block`, se va returna matricea inițială.


## ⟳ Rotirea în jurul axei Oy

> Se interschimbă dimensiunile pentru **lățime** și **adâncime**.


În 3 **for**-uri imbricate:
se alocă memorie pentru noua matrice și se completează dupa regula:
```c
new_mat[x][y][z] = chunk[z][y][depth - 1 - x];
```



## Gravitație (TODO)

Chiar dacă algoritmul nu este corect, iată care sunt pașii:
- Am definit o structură în care am memorat coordonatele tuturor punctelor dintr-un corp
- Un corp se obține în urma rulării unui algoritm de umplere (**Fill**),
  blocurile se vor înlocui cu `BLOCK_AIR` pe masură ce vecinii sunt parcurși
- Se calculează distanța de cădere pentru fiecare block în parte
- La final, corpurile sunt repoziționate pe axa **Oy**
- Âtâta timp cât planul superior (paralel cu xOz) este gol: micșorez înălțimea



## 📥 Compresie


Pentru a reprezenta un **run** (blocuri consecutive identice), am definit structura:

```c
typedef struct {
  int num_occurrences;
  char block;
} Pair;
```


Pasul 2 al algoritmului de compresie returnează un array cu toate run-urile: `Pair *pair`.

Funcțiile principale pentru compresie:
- `flatten`: aplatizează matricea 3D și returnează un vector linearizat
- `get_pairs`: primește array-ul aplatizat și generează toate run-urile, întorcând un vector `Pair *pair`
- `add_pair_to_bytes`: codifică un run într-unul sau doi octeți și îl adaugă la finalul codificării

## 📤 Decompresie

Dimensiunea array-ului de octeți nu este furnizată,
deci iterarea se face printr-un pointer care parcurge vectorul până la `NULL`.

Exemplu de iterare:
```c
unsigned char *ptr = bytes;

while (*ptr) {
  unsigned char byte = *ptr;
  ptr++;
}
```
