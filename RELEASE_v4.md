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

## 🤖 Modelli AI usati

```bash
ollama pull gemma3:4b
ollama pull qwen2.5-coder:3b
ollama pull dolphinlabs/llama3.2-3b-uncensored