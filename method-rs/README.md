# Blocks in 3D

Structura proiectului:

- `chunk_gen.rs`:
  - Validare punct în spațiu
  - Amplasare bloc
  - Generare cuboid/sferă  
- `chunk_process.rs`:
  - Algoritmi de umplere
  - Creare înveliș  
- `chunk_transform.rs`:
  - Rotație 90° pe axa Oy (plan xOz)
  - TODO: gravitație  
- `chunk_compress.rs`: compresia/decompresia matricii 3D într-un șir de octeți
- `chunk_io.rs`: API pentru citirea și scrierea chunk-urilor (inclusiv compresia datelor în format binar)
    în fișier.

## Creare cuboid

Funcția primește două colțuri opuse ale unui paralelipiped dreptunghic.  


```rs
let min_x = max(min(x0, x1), 0isize);
let min_y = max(min(y0, y1), 0isize);
let min_z = max(min(z0, z1), 0isize);

let max_x = min(max(x0, x1), (width - 1) as isize);
let max_y = min(max(y0, y1), (height - 1) as isize);
let max_z = min(max(z0, z1), (depth - 1) as isize);

for x in min_x..=max_x {
    for y in min_y..=max_y {
        for z in min_z..=max_z {
            chunk_place_block(...);
        }
    }
}
```



## Creare sferă


```rs
let r = radius.ceil() as isize;

for i in -r..=r {
    for j in -r..=r {
        for k in -r..=r {
            let dist: f32 = euclidian_dist(x, y, z, x + i, y + j, z + k);

            if dist > radius {
                continue;
            }

            chunk_place_block(chunk, width, height, depth, x + i, y + j, z + k, block);
        }
    }
}
```

## 📦 Înveliș

Dacă `target_block` = `shell_block`, algoritmul clasic ar umple matricea complet.  
Pentru a evita această problemă, folosesc o **stivă de coordonate (X,Y,Z)**
având o implementare minimală sub forma unui vector de puncte 3D (tupluri).

Parcurgând matricea, coordonatele fiecarărui `target_block` se adaugă la finalul vectorului cu `.push()`

Apoi, pentru fiecare block extras din varful stivei,
functia `wrapper` plasează `shell_block` în locul vecinilor diferiți de `target_block`.

## Rotirea în jurul axei Oy

Se alocă o nouă matrice:
- `new_width = old_depth`
- `new_depth = old_width`

> Practic interschimbă dimensiunile pentru **lățime** și **adâncime**.

```rs
let mut new_mat: Vec<Vec<Vec<u8>>> = vec![vec![vec![BLOCK_AIR; *width]; *height]; *depth];
```


Valorile sunt copiate conform regulii:  
```rs
new_mat[x][y][z] = chunk[z][y][depth - 1 - x]
```
## 📥 Compresie

**Run**-urile se parcurg cu `.iter().for_each()`, iar codificararile se adauga la finalul sirului de octeti.



## ✅ Teste Unitare

Am folosit [`rstest`](https://crates.io/crates/rstest), un framework care permite scrierea
**testelor parametrizate**. Astfel, pot defini o singură funcție pentru a testa fiecare task
și o rulez cu valori diferite, evitând cod duplicat.  

Testele au aceeași structură: se citesc fișiere `.in`, `.out`, `.ref` (eventual si `.param`),
iar diferența este doar indicele `idx` care selectează setul de fișiere.  

Avantajul testelor parametrizate este că nu scriu o funcție separată pentru fiecare test case.
Definind o singură funcție generică, `rstest` generează cazurile de test
în funcție de valorile `[case(...)]`.  

### ▶️ Rularea testelor

- Toate testele:
```sh
$ cargo test
```

- Un test (toate cazurile):
```sh
$ cargo test task1::
```

- Un test case specific:
```sh
$ cargo test test_task1::case_2 -- --exact
```
