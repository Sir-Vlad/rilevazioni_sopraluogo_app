# 1

Quando ho più stanze con lo stesso codice stanza e destinazione d'uso raddoppiare i dati inseriti. Questo succede
sopratutto quando il codice di stanza è `_`.

Per esempio, ho questa stanza `(_, corridoio)` che è ripetuta 4 volte. Se devo collegare a questa tupla una finestra
questa verrà collegata a tutti gli infissi, quindi nella tabella `stanze_con_infissi` avrò 4 record.

# 2 - Esportazione

Esportazione dei dati in excel (`xlsm`). La libreria da usare
è: [rust_xlsxwriter](https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/index.html).

# 3 - Multi-edifici nello stesso fascicolo

Gestire quando in un fascicolo ci sono due edifici indentificati tramite la colonna `chiave`. Creare un modo nella UI
per selezionare quale edificio si sta analizzando, questo è fondamentale perchè alcune stanze potrebbero essere
duplicate.

# 4 - Dare un codice di stanza a chi non c'è l'ha

In molti edifici non ci sono dei numeri per le stanze, questo fa collassare il sistema perchè non si riesce più a
identificare univocamente le stanze. Questo processo può essere fatto sia al sistema, ma è meglio che l'utente prima di
caricare il file dei dati inserisca i codici delle stanze.

I codici di stanza seguono questa logica. Il codice è composto da 3 numeri, dove:

- il primo identifica il piano, se il piano è interrato ci sarà un meno d'avanti
- il secondo e il terzo rappresentano il numero della stanza in maniera incrementale

Per esempio,

- la stanza 34 al piano 2 avrà codice 234
- la stanza 5 al piano -1 avrà codice -105
- la stanza 23 al piano terra avrà codice 023

# 5 - Aggiungere nuovi tipi

Quando vengono inseriti tipi che non sono presenti all'interno del sistema vanno inseriti anche all'interno delle
tabelle.

IDEA: Creare una pagina dove posso inserire o modificare i tipi inseriti con i loro relativi dati aggiuntivi.
Questo implica che il campo altro va eliminato e sostituito da un form per inserire un nuovo tipo.