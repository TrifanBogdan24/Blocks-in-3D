## Utilizare 3D Viewer

Pentru a vizualiza în interfața grafică un fișier ce conține un chunk, puteți executa:

```bash
./view3d.sh chunk.txt
```

Dacă omiteți fișierul, scriptul va citi datele unui chunk de la intrarea standard.

Dacă folosiți opțiunea `-o`, scriptul de 3D view va salva o imagine în loc să deschidă interfața:

```bash
./view3d.sh chunk.txt -o figure.png
```

Puteți analiza cu `view3d.sh` datele de intrare și ieșire ale testelor. Le găsiți în directorul `tests/`.

## Vizualizare cod binar

În cadrul taskurilor 9 și 10 veți lucra cu șiruri de byți într-un format binar. Ca atare, testele sunt fișiere binare și pot fi vizualizate astfel:

```bash
xxd -b cod_binar.txt
```
