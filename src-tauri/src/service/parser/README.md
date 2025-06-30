Questo modulo serve per parsare delle stringhe per eseguire le operazioni avanzate all'interno del programma.
La sintassi che verrà utilizzata è la seguente:

- `<...>` = parametro obbligatorio
- `[...]` = parametro facoltativo
- `...` = ripetibile
- `{n,m}` = ripetibile da n a m volte

## Inserimento di un infisso

```regex
>fi <altezza>x<larghezza>|<vetro>|<altezza>
```

## Modifica di una stanza/e

Questo è il comando per inserire gli stessi valori dei campi a più stanze

```jsregexp
>s <stanza>[, stanza...] -> fieldUpdate[, fieldUpdate...]
```

Espansione di `fieldUpdate`:

```jsregexp
fieldUpdate => field: valore
```

se il `field` è infissi il valore prende la sintassi `<idInfisso>x<quantificatore>.

Se si volesse inserire un valore di un campo a tutte le stanze:

```jsregexp
>sa fieldUpdate
```

il carattere `a` indica all, ovvero tutto.