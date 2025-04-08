# App per eseguire i sopralluoghi

Questa applicazione ha il compito di semplificare l'inserimento dei dati sia sul campo che in ufficio.

## Tecnologie utilizzate

L'applicazione è stata sviluppata tramite il framework Tauri, che permette lo sviluppo di applicazioni desktop
multipiattaforma che combina frontend web con un backend nativo in Rust. Per il frontend si è deciso di utilizzare
React.

## Funzionamento

Il sistema prende un file `xmlx`, lo elabora e ne crea una copia dei dati all'interno di un database sqlite all'interno
della cartella `Documenti` del proprio pc. Dopo la creazione del database, il sistema lo utilizza per aggiornare i dati
che poi potranno essere esportati in `xmlx` (coming soon).

## Funzionalità

L'applicazione offre:

- Inserimento di nuovi infissi, indicandone la dimensione, la tipologia del serramento e la tipologia del vetro
- Inserimento di nuovi dati per una stanza
- Cambio da un database a un'altro

## TODO

Ecco le future implementazioni:

- [ ] Visualizzazione dei dati dell'edificio
- [ ] Esportazione dei dati in excel
- [ ] Modifica dei dati sulle tabelle di visualizzazione
- [ ] ShortKey per le operazioni eseguite maggiormente
- [ ] Filtri sulla tabella generale