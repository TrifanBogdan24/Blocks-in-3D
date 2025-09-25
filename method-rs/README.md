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

## 📌 Amplasare bloc

- `is_inside`: verifică dacă un punct **(x, y, z)** se află în interiorul chunk-ului:
  returnează **1** dacă este valid, altfel **0**

- `chunk_place_block`: dacă coordonatele sunt valide, actualizează tipul blocului de la
  poziția respectivă; dacă nu, funcția nu are efect.

## 📦 Creare cuboid

- Funcția primește două colțuri opuse ale unui paralelipiped dreptunghic.
- Coordonatele de iterație pornesc de la **MIN** fiecărei axe și merg până la **MAX**.
- Pentru eficiență și siguranță, limitez aceste coordonate de dimensiunile chunk-ului


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


## 🪩 Creare sferă

Notez cu `r` = întregul cel mai mare (`ceil`) la care se rotunjeste raza.

Pentru fiecare offset din intervalul `[-r, r]` pe cele 3 axe, se calculează distanța euclidiană
față de centru. Dacă distanța ≤ raza reală, plasez blocul; altfel, îl ignor.

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

Mulțimea punctelor de interes care trebuiesc înfășurate
este reprezentată de o **stivă de triplete**:
- primul element -> coordonata X
- al doilea  -> coordonata Y
- al treilea -> coordonata Z

> 💡 În Rust, un `Vec` funcționează și ca stivă, având deja implementate metodele `.push()` și `.pop()`.

Stiva folosită de mine are următorul tip:
```rs
Vec<(usize, usize, usize)>
```

Inițial, stiva este goală.
Pe măsură ce descopăr puncte valide, creez un tuplu **(x, y, z)** și îl adaug cu `.push()` în stivă.

După ce toate blocurile din chunk au fost parcurse, încep procesul de învelire:
cât timp stiva mai conține elemente,
extrag un tuplu cu `.pop()`, îl "descompun" în coordonate
și apelez funcția `wrapper` cu aceste valori.




## Fill

Algoritm recursiv de umplere: pornește dintr-un punct și vizitează vecinii de același tip.

Dacă `target_block == new_block`, se va returna matricea inițială.


## Rotirea în jurul axei Oy

> Rotirea presupune schimbarea dimensiunilor dintre **lățime** și **adâncime**.  

Se alocă memorie pentru o nouă matrice 3D, inițializată complet cu **blocuri de aer**.  
```rs
let mut new_mat: Vec<Vec<Vec<u8>>> = vec![vec![vec![BLOCK_AIR; *width]; *height]; *depth];
```


Iar ulterior, valorile sunt copiate (în 3 bucle **for**) din matricea veche în cea nouă,
conform regulii: 
```rs
new_mat[x][y][z] = chunk[z][y][depth - 1 - x]
```

## 📥 Compresie

Urmatoarea structura defineste un **run**:
```rs
#[derive(Default)]
struct Run {
    num_occurrences: usize,
    block: u8,
}
```

Funcțiile principale pentru compresie:
- `flatten`: aplatizează matricea 3D și returnează un vector linearizat
- `get_runs`: primește un slice la array-ul aplatizat și generează toate run-urile,
    întorcând un `Vec<Run>`
- `encode_run`: codifică un run într-unul sau doi octeți și îl adaugă la finalul codificării


În `chunk_encode`, fiecare **run** este parcurs și transmis funcției `encode_run`
folosind un **iterator** `for_each` pentru a adăuga octeții corespunzători în vectorul final.

În stilul **programării funcționale**:
```rs
runs.iter().for_each(|run| encode_run(&mut bytes, run));
```


## 📤 Decompresie

Spre deosebire de versiunea în C, aici dimensiunea codificării este deja cunoscută:
`Vec<u8` păstrează automat numărul de elemente.

Atât la compresie, cât și la decompresie, am folosit **pattern matching** asupra tipului de bloc,
ceea ce face codul mai ușor de scris și de înțeles.


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
