# Blocks in 3D

În acest proiect am implementat o bibliotecă numită `libchunk`, care permite
modelarea lumilor tri-dimensionale folosind structuri cubice
(idee inspirată din Minecraft).

Proiectul reprezintă un bun exercițiu practic pentru a înțelege:
- matricile și operațiile pe pointeri în C
- structurile vectoriale și referințele mutabile în Rust


În implementarea în Rust, mi-am aprofundat abilitățile de a scrie **teste unitare parametrizate**
cu [`rstest`](https://crates.io/crates/rstest) și am creat de la zero un script
personalizat de `checker` pentru validarea rezultatelor.

## Structura proiectului

- `chunk_gen`
  - Validare punct în spațiu
  - Amplasare bloc
  - Generare cuboid/sferă  
- `chunk_process`:
  - Algoritmi de umplere
  - Creare înveliș  
- `chunk_transform`:
  - Rotație 90° pe axa Oy (plan xOz)
  - TODO: gravitație  
- `chunk_compress`: compresia/decompresia matricii 3D într-un șir de octeți



## 📌 Amplasare bloc

Funcția modifică tipul blocului la coordonatele **(x, y, z)** primite.

Pentru modularitate, am creat o funcție auxiliară care returnează **true** dacă
coordonatele sunt în interiorul chunk-ului și **false** în caz contrar.


`chunk_place_block` si `is_inside` sunt apelate pe tot parcursul proiectului
pentru modificarea blocurilor, respectiv verificarea coordonatelor.


## 🧊 Creare cuboid

Funcția primește 2 colțuri opuse ale unui paralelipiped dreptunghic.  
Coordonatele de iterație pornesc de la minimul fiecărei axe și merg până la maxim.  
Astfel, chiar dacă nu conteaza ordinea colțurilor, chunk-ul este parcurs corect. 

## 🪩 Creare sferă

Notez cu `r` = întregul cel mai mare (`ceil`) la care se rotunjeste raza.
o raza de **-1.2** se va rotunji la **-2**, iar pentru **1.2** `r` va fi egal cu **2**.

Pentru fiecare offset din intervalul `[-r, r]` pe cele 3 axe, se calculează distanța euclidiană
față de centru. Dacă distanța ≤ raza reală, plasez blocul; altfel, îl ignor.

> **Distanta euclidiana** se calculeaza (folosind functiile `pow` si `sqrt` din biblioteca `math.h`)
> ca suma patratelor diferentelor coordontalor pe cele 3 axe.

> `dist(P1, P2) = sqrt((P1.x - P2.x)^ 2 + (P1.y - P2.y)^ 2 + (P1.z - P2.z)^2 )`


## 📦 Înveliș

Dacă `target_block` este egal cu `shell_block`, un algoritm de umplere
simplu ar completa întreaga matrice.
Pentru a evita acest comportament, mai întâi iterez întreaga matrice 3D
și colectez (într-o structură dedicată)
coordonatele **(x, y, z)** punctelor de interes.

> ⚠️ ATENȚIE: Este esențial ca toate punctele să fie colectate
> înainte de a apela funcția `wrapper` asupra lor.

După colectare, iterez mulțimea de puncte și aplic funcția `wrapper`,
care plasează `shell_block` în locul vecinilor diferiți de `target_block`.

Pentru a înveli un bloc, este nevoie de a verifica 8 puncte alăturate:
**(x±1, y±1, z±1)**.

## Fill

Algoritm recursiv de umplere: pornește dintr-un punct și vizitează vecinii de același tip,
fără să mai fie nevoie de memorie suplimentară pentru marcarea blocurilor vizitate.

Cazul în care `target_block` este egal cu `new_block` este tratat separat
înaintea rulării algoritmului de umplere: se va returna matricea ințială.


## ⟳ Rotirea în jurul axei Oy

Operația presupune crearea unei noi matrice 3D, unde axele **Ox** și **Oz** sunt 
interschimbate.

> Practic: se interschimbă dimensiunile pentru **lățime** și **adâncime**,
> în timp ce înălțimea rămâne neschimbată


Fiecare element este copiat în noua matrice conform regulii:
```c
new_mat[x][y][z] = chunk[z][y][depth - 1 - x];
```

Complexitatea este `θ(N^3)`, deoarece pentru fiecare bloc din noua matrice 
se parcurge volumul inițial prin 3 bucle **for** imbricate, acoperind toate coordonatele.

## 📥 Compresie

1. **Serializare**: matricea 3D este aplatizată într-un vector liniar (ordinea **y->z->x**).  
2. Vectorul este parcurs pentru a determina secvențe de blocuri identice consecutive: 
   - Se generează un vector de perechi `(num_occurrences, tip_block)`
   - Dacă numărul depășește 4095, se începe un nou **run**
3. Transformarea **run**-urilor în octeți:  
   - <32 apariții => codificare pe 1 octet  
   - ≥32 apariții => codificare pe 2 octeți  



Pentru aplatizarea matricii 3D, am folosit formula:
```rs
idx = y * (depth * width) + z * width + x;
array[idx] = chunk[x][y][z];
``` 



## 📤 Decompresie

1. Se alocă matricea 3D pe baza dimensiunilor cunoscute
2. Se itereaza vectorul codificarii (byte cu byte), reconstruind blocurile
3. Decodificarea se bazează pe marcatorii din biții 5 și 6:
   - `0` => **run** pe 1 octet (5 biți pentru numărul de aparitii)
   - `10` => **run** pe 2 octeți (11 biți pentru număr de aparitii)

Coordonatele **(x, y, z)** se actualizează în timpul reconstrucției vectorului astfel:
1. Incrementează `x`
2. Dacă `x` depășește limita, resetează `x = 0` și incrementează `z`
3. Dacă `z` depășește limita, resetează `z = 0` și incrementează `y`
4. Dacă `y` depășește limita (caz anormal), funcția se oprește forțat



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


## Operații pe biți

- Putere a lui 2: `1 << n`  
- Setare bit: `byte |= (1 << i)`  
- Verificare valoare (0/1) bit:
  - In C: `if (byte & (1 << i))`
  - In Rust: `if byte & (1 << i) > 0`
