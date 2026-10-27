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
@'
# 🚀 Gideon AI v4.0.0 — Note di rilascio

**Data:** 23 ottobre 2026
**Autore:** Rocco Degiacomo
**Repo:** https://github.com/ingannicus/Gideon-AI
**Licenza:** CC BY-NC 4.0

---

## Panoramica

La v4 estende quanto già introdotto nella v2 (GGUF Picker, connettore universale HuggingFace, UI cyberpunk, routing intelligente, streaming con Stop, download snippet) e aggiunge **tre moduli craft** per la generazione diretta di documenti, **hardware detection più precisa** con quantizzazione GGUF consigliata, e un **backend più solido** nella gestione della cronologia e del contesto.

La v3 non verrà rilasciata: la v4 la sostituisce e ne incorpora le funzionalità previste.

> ⚠️ **Questa release è solo per Windows.** La versione macOS non è inclusa e sarà pubblicata in una release successiva.

---

## 🆕 Novità rispetto alla v2

### 🛠️ Moduli Craft integrati (nuovo in v4)

Tre moduli intercettano il prompt **prima** di Ollama e generano o elaborano file direttamente, tramite i rispettivi CLI MCP.

#### Cosa sono i "craft"

WordCraft, GridCraft e PdfCraft non sono moduli nativi scritti da zero, ma **client che si interfacciano a CLI esterni** del progetto open source **ArtCraft**. Sono reimplementazioni "clean-room" (scritte da zero) di software proprietari, realizzate in **Rust** e distribuite gratuitamente con licenza open source (MIT/Apache-2.0):

- **WordCraft** → alternativa a Microsoft Word, gestisce file `.docx`
- **GridCraft** → alternativa a Microsoft Excel, gestisce fogli di calcolo e formule
- **PdfCraft** → alternativa ad Adobe Acrobat, per operazioni sui PDF (estrazione testo, unione, ecc.)

Sono progettati per essere controllati da agenti AI tramite **MCP** (Model Context Protocol) o CLI: è esattamente il meccanismo che i file `wordcraft.rs`, `gridcraft.rs` e `pdfcraft.rs` usano per lanciare gli eseguibili e fargli eseguire i comandi.

#### Tabella moduli

| Modulo | File | Cosa fa |
|--------|------|---------|
| **WordCraft** | `src/wordcraft.rs` | Genera documenti Word (`.docx`) da prompt strutturati. Supporta anche documenti lunghi (≥100 caratteri) con sintassi `documento ... con:` |
| **GridCraft** | `src/gridcraft.rs` | Genera fogli di calcolo (`.xlsx`) scrivendo i dati nella colonna A |
| **PdfCraft** | `src/pdfcraft.rs` | Estrae testo da file PDF esistenti |

Ogni modulo espone `try_*_action(&prompt) -> Option<String>`: se riconosce l'azione, risponde direttamente con il risultato.

#### Parole chiave che attivano i craft

| Craft | Parole chiave |
|-------|---------------|
| WordCraft | `documento`, `word`, `docx`, `scrivi un file` |
| GridCraft | `foglio`, `excel`, `tabella`, `cella` |
| PdfCraft | `pdf`, `estrai`, `unisci`, `dividi` |

#### Dove reperire i craft

I progetti sono ospitati su GitHub sotto l'organizzazione ufficiale **`storytold`**:

- **WordCraft**: https://github.com/storytold/wordcraft
- **GridCraft**: https://github.com/storytold/gridcraft
- **PdfCraft**: https://github.com/storytold/pdfcraft

Panoramica di tutte le applicazioni ArtCraft: https://getartcraft.com/apps

#### Prerequisiti

I CLI devono essere scaricati dalle release GitHub dei rispettivi progetti e posizionati nei percorsi usati dal codice:

- `C:\Program Files\WordCraft\wordcraft-cli.exe`
- `C:\Program Files\GridCraft\gridcraft-cli.exe`
- `C:\Program Files\PdfCraft\pdfcraft-cli.exe`

💡 Esempi d'uso
Di seguito alcuni esempi pratici di come interagire con Gideon per attivare i moduli craft. Basta scrivere il prompt nella chat: Gideon riconosce le parole chiave e passa la richiesta al craft corrispondente, senza coinvolgere Ollama.

📝 WordCraft — creare un documento Word
Parole chiave: documento, word, docx, scrivi un file

Esempio 1 — documento semplice:

text
Crea un documento word con: Relazione mensile sulle vendite. I dati mostrano una crescita del 12% rispetto al mese precedente.
→ Gideon crea gideon_output.docx con il testo indicato dopo i due punti.

Esempio 2 — nome file personalizzato:

text
Scrivi un file report_vendite.docx con: Il fatturato del trimestre ha raggiunto l'obiettivo previsto.
→ Il file viene salvato come report_vendite.docx.

Esempio 3 — documento lungo (≥100 caratteri):

text
Crea un documento con: Questa è una relazione tecnica dettagliata che descrive l'architettura del sistema Gideon AI, i moduli craft integrati, il connettore universale per HuggingFace e le modalità di routing intelligente dei modelli.
→ WordCraft usa il percorso create_long_document: scrive prima un .txt, lo apre e lo salva come .docx.

📊 GridCraft — creare un foglio di calcolo
Parole chiave: foglio, excel, tabella, cella

Esempio 1 — lista semplice:

text
Crea un foglio excel con: Mario, Luigi, Anna
→ Gideon scrive i tre nomi nella colonna A di gideon_grid.xlsx (3 righe).

Esempio 2 — tabella di valori:

text
Genera una tabella con: Prodotto A, 120, Prodotto B, 85, Prodotto C, 200
→ I valori vengono inseriti in sequenza nella colonna A.

⚠️ GridCraft attualmente scrive solo nella colonna A, una riga per valore. Per tabelle multi-colonna è necessario estendere il modulo.

📄 PdfCraft — estrarre testo da un PDF
Parole chiave: pdf, estrai, unisci, dividi

⚠️ Importante: nella v4 PdfCraft esegue solo l'estrazione di testo da un PDF esistente. Non genera PDF nuovi.

Esempio 1 — estrazione semplice:

text
Estrai il testo da documento.pdf
→ Gideon cerca documento.pdf nella cartella del progetto e ne restituisce il contenuto testuale.

Esempio 2 — file con percorso:

text
Estrai il contenuto da report_2026.pdf
→ Se il file non esiste, Gideon risponde con un errore che invita a verificare la presenza del PDF nella cartella.



Se un CLI non è presente, il craft risponde con un avviso (es. "⚠️ WordCraft non avviato o non trovato") e Gideon **non** passa a Ollama per quella richiesta.

---

### ⚙️ Hardware Detection avanzata (potenziata in v4)

Rispetto alla v2, il rilevamento hardware ora calcola anche:

- **Tier** (`high` / `mid` / `low`) in base a RAM libera e core CPU
- **Quantizzazione GGUF consigliata**: `Q8_0`, `Q6_K`, `Q5_K_M`, `Q4_K_M`, `Q3_K_M`, `IQ2_M`
- **RAM massima stimata per i modelli**, usata per filtrare i file GGUF compatibili
- **Stima RAM per file GGUF** dal nome (parametri + quantizzazione)
- Parametri Ollama adattati (`num_thread`, `num_ctx` calcolato dinamicamente)

### 🧠 Gestione contesto migliorata (nuovo in v4)

- Cronologia limitata a **6 coppie** (12 messaggi), tagliata **sempre da un messaggio `user`** per non rompere il formato chat
- Calcolo dinamico di `num_ctx` in base alla lunghezza della conversazione + margine di risposta
- Rimozione dei fence markdown in modalità codice, con buffer `pending` per non spezzare i blocchi

### 🔌 Connettore universale — esteso in v4

- **Pull diretto con quant auto**: se non specifichi il file, Gideon sceglie la quantizzazione migliore per il tuo hardware e avvia il pull
- **GGUF Picker** con filtri per quantizzazione e badge **Consigliato / Compatibile / Troppo grande**
- API `/hf-files` che restituisce metadati calcolati (RAM stimata, quant, compatibilità, raccomandazione)

---

## 🔍 Funzionalità confermate dalla v2

- **GGUF Picker**: scansione repo HuggingFace, metriche, badge compatibilità, filtri Q4/Q5/Q6/Q8, pull con un clic
- **Connettore universale**: integrazione Ollama + HuggingFace Direct + hot-swapping del modello
- **UI Cyberpunk**: HUD animato su canvas, tema scuro, pannello risorse in tempo reale
- **Chat**: animazioni fluide, badge modalità (Generale / Codice / Sicurezza / Uncensored)
- **Routing intelligente**: instradamento dinamico delle richieste
- **Controllo generazione**: streaming in tempo reale + pulsante Stop
- **Utilità**: download snippet di codice, data/ora contestuale, multi-sessione
- **100% locale e privato**: nessuna dipendenza cloud

---

## 📡 API REST (v4)

| Metodo | Endpoint | Descrizione |
|--------|----------|-------------|
| GET | `/` | Web UI |
| POST | `/reset` | Nuova sessione |
| POST | `/chat` | Chat con streaming |
| GET | `/hw` | Info hardware |
| GET | `/models` | Modelli Ollama installati |
| POST | `/connect` | Pull modello (Ollama o HuggingFace) |
| POST | `/hf-files` | Lista file GGUF di un repo HuggingFace |

---

## 🖥️ Requisiti

| | Minimo | Consigliato |
|---|---|---|
| RAM | 8 GB | 16 GB |
| Spazio disco | 10 GB | 15 GB |
| CPU | qualsiasi moderna | con GPU dedicata |
| OS | **Windows 10/11** | — |

> Con una GPU (anche GTX 1060 / RX 580) la velocità migliora di 10–20x.

**Moduli Craft (opzionali):** WordCraft, GridCraft e PdfCraft richiedono i rispettivi CLI del progetto [ArtCraft](https://getartcraft.com/apps). I binari vanno scaricati da GitHub ([wordcraft](https://github.com/storytold/wordcraft), [gridcraft](https://github.com/storytold/gridcraft), [pdfcraft](https://github.com/storytold/pdfcraft)) e posizionati in `C:\Program Files\<Nome>\<nome>-cli.exe`. Senza di essi Gideon funziona normalmente, ma i craft rispondono con un avviso.

---

## 📦 Installazione

### Windows
1. Scarica `GideonAI_Export.zip` dalla pagina Releases
2. Estrai la cartella
3. Doppio clic su `start.bat`
4. Il browser si apre su `http://localhost:5000`

### macOS
> ⚠️ **La versione macOS NON è inclusa in questa release.**
> `GideonAI_Export_Mac.zip` non è disponibile nella v4.0.0. Il supporto macOS completo (incluso il pacchetto `start.sh` / `Avvia Gideon AI.command` e i tre moduli craft compilati per Mac) sarà pubblicato in una **release successiva** (v4.1.0 o v5).
> Fino ad allora, su macOS è possibile compilare da sorgente seguendo la sezione "Compilare da sorgente", ma i moduli craft non sono garantiti.

---

## 🤖 Modelli AI usati di default

```bash
ollama pull gemma3:4b
ollama pull qwen2.5-coder:3b
ollama pull dolphinlabs/llama3.2-3b-uncensored
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
