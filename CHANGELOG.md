# Changelog — Gideon AI
## [4.1.0] — 2026-10-24

### Modificato
- **Modello per programmazione aggiornato**: `qwen2.5-coder:3b` → `qwen2.5-coder:7b`
- **Aggiunto `MODEL_FAST`**: `FableForge-AI/nexus-coder` (1.54B, ~986 MB)

### Corretto
- **PdfCraft**: risolto errore `unknown argument "path"` — ora usa `doc_open` + `text_extract` con `doc` e `pages`
- **PdfCraft**: output pulito (solo testo, senza JSON)
- **GridCraft**: rimosso hardcoded "Mario, Luigi" — ora estrae i dati dal prompt

### Note
- Il modello `qwen2.5-coder:3b` non è più necessario: `ollama rm qwen2.5-coder:3b`
- PdfCraft richiede `--root` per trovare i file nella cartella del progetto

Tutte le modifiche rilevanti al progetto sono documentate in questo file.

---

## [v2.0] — 2026-10-22

### Aggiunto

- **Auto-Hardware Detection** — all'avvio Gideon scansiona CPU (brand e core), RAM totale e disponibile, GPU (via `wmic` su Windows, `lspci` su Linux, `system_profiler` su macOS) e calcola automaticamente tier, thread ottimali e context window
- **Quantizzazione GGUF automatica** — in base alla RAM libera il sistema sceglie la quantizzazione ideale (da IQ2_M a Q8_0) senza nessuna configurazione manuale
- **GGUF Picker** — modal con lista completa dei file `.gguf` di qualsiasi repo HuggingFace, con badge COMPATIBILE / TROPPO GRANDE calcolati sull'hardware reale, filtri per quantizzazione e pulsante Pull per ogni file
- **Pull diretto (quant auto)** — scarica in autonomia il file GGUF più adatto al PC ospite senza aprire il picker
- **Connettore Universale** — dropdown con i modelli Ollama installati + campo per collegare repo HuggingFace; il modello selezionato diventa attivo per le chat successive
- **Route `/hw`** — espone i dati hardware rilevati in JSON
- **Route `/models`** — lista i modelli Ollama installati
- **Route `/hf-files`** — interroga l'API HuggingFace e restituisce i file GGUF con metadati di compatibilità
- **Route `/connect`** — avvia il pull di un modello Ollama o HuggingFace in background
- **Topbar hardware** — CPU core, RAM libera e quantizzazione consigliata visibili in tempo reale nella barra superiore
- **Pannello HW nella sidebar** — mostra CPU, core, RAM totale/libera, GPU, quant consigliata, RAM massima per modelli e badge tier
- Dipendenza `sysinfo = "0.32"` aggiunta al `Cargo.toml`

### Migliorato

- UI completamente rinnovata — nuova palette colori, variabili CSS unificate, animazioni `msgIn` sui messaggi, hover con glow, focus ring sul textarea
- HUD canvas aggiornato alla versione v4 con accento verde `#00ff88`
- I parametri `num_thread` e `num_ctx` passati a Ollama ora si basano sull'hardware rilevato invece di valori fissi
- Struttura `AppState` estesa con `hw_info` e `active_model`
- Supporto `model_override` nel payload `/chat` — permette di usare qualsiasi modello connesso dal picker

### Corretto

- Rimossa struct `ModelsResponse` inutilizzata che generava un warning in compilazione

---

## [v1.0] — 2026 (prima release)

### Aggiunto

- Backend Rust nativo con Axum e streaming NDJSON
- Routing automatico tra modelli: `gemma3:4b` (generale/sicurezza), `qwen2.5-coder:3b` (codice), `dolphinlabs/llama3.2-3b-uncensored` (modalità uncensored)
- Interfaccia web embedded in binario — nessun file HTML esterno
- HUD canvas animato stile cyberpunk
- Bottone Stop con cancellazione cooperativa dello stream
- Download del codice generato come file (`.py`, `.js`, `.rs`, `.sql`, ecc.)
- Rimozione automatica dei fence markdown in modalità codice
- Cronologia conversazione con limite `MAX_HISTORY`
- Data e ora contestuale nel system prompt
- Reset sessione
- Apertura automatica del browser all'avvio
- Script di build per Windows (`build_rust.bat`) e macOS/Linux (`build_rust.sh`)
- Icona personalizzata incorporata nell'exe (Windows)
