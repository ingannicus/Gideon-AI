# Gideon AI

> Assistente AI locale, privato e senza censura — costruito in Rust puro.

![Rust](https://img.shields.io/badge/Backend-Rust-orange?style=flat-square&logo=rust)
![Ollama](https://img.shields.io/badge/Engine-Ollama-black?style=flat-square)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue?style=flat-square)
![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)
![Version](https://img.shields.io/badge/Version-4.0-purple?style=flat-square)

---

## Cos'è Gideon AI

Gideon AI è un assistente tecnico locale che gira interamente sul tuo PC — nessun cloud, nessuna API key, nessun abbonamento. Il backend è scritto in Rust nativo con Axum, l'interfaccia è una web UI accessibile dal browser, e il motore AI è Ollama.

Smista automaticamente le richieste al modello più adatto:

| Tipo di richiesta | Modello usato |
|---|---|
| Conversazione generale | `gemma3:4b` |
| Scrittura e analisi codice | `qwen2.5-coder:3b` |
| Sicurezza informatica | `gemma3:4b` (system prompt dedicato) |
| Modalità uncensored | `dolphin llama3.2-3b-uncensored` |
| Modalità uncensored Pc Performant| `bartowski/Qwen2.5-Coder-14B-Instruct-abliterated-GGUF` <--incolla nel campo scansione di Gideon
| Qualsiasi modello HuggingFace | scelto e scaricato in autonomia |

---
##🚀 Gideon AI v4.15
⚡ Quantizzazione automatica intelligente

Gideon AI analizza automaticamente l'hardware del sistema e seleziona la quantizzazione GGUF più adatta alle risorse disponibili. Tra le opzioni supportate:

Q8_0

Q6_K

Q5_K_M

Q4_K_M

Altre varianti compatibili

Questo permette di ottenere il miglior equilibrio tra qualità delle risposte, velocità e consumo di memoria, senza richiedere configurazioni manuali.

🗑️ Gestione avanzata dei modelli

È ora possibile rimuovere i modelli scaricati manualmente direttamente dall'interfaccia tramite il pulsante cestino presente nella barra laterale.

💡 Ottimizzazione per Hardware ad Alte Prestazioni

Se utilizzi un PC dotato di molta RAM, processore potente o GPU dedicata, il Connettore Universale integrato in Gideon AI può interrogare direttamente Hugging Face per individuare modelli più avanzati e ottimizzare automaticamente il carico di lavoro in base all'hardware disponibile.

Per gli utenti che desiderano massimizzare la qualità della generazione di codice e del ragionamento logico, è consigliato il modello:

bartowski/Qwen2.5-Coder-14B-Instruct-abliterated-GGUF

Vantaggi

 ✅ Migliore qualità nella programmazione
 ✅ Maggiore capacità di ragionamento
 ✅ Risposte più accurate e dettagliate
 ✅ Ottimizzato per workstation e PC ad alte prestazioni
 ✅ Utilizzo completamente locale
---
## Funzionalità v4

### 💻 Auto-Hardware Detection
All'avvio, Gideon scansiona automaticamente CPU, RAM e GPU del PC ospite e calcola:
- La **quantizzazione GGUF ottimale** in base alla RAM disponibile
- I **thread e il context window** ideali per l'hardware rilevato
- Il **tier di potenza** (High / Mid / Low) visualizzato nell'interfaccia

| RAM libera per modelli | Quantizzazione consigliata |
|---|---|
| ≥ 12 GB | Q8_0 (qualità massima) |
| ≥ 8 GB | Q6_K (ottima qualità) |
| ≥ 5.5 GB | Q5_K_M (alta qualità) |
| ≥ 3.5 GB | Q4_K_M (uso generale, consigliata) |
| ≥ 2.5 GB | Q3_K_M (compressa) |
| < 2.5 GB | IQ2_M (ultra-leggera) |

### 🔍 GGUF Picker — selezione guidata del modello
1. Inserisci un repo HuggingFace nel connettore (es. `bartowski/Phi-3-mini-4k-instruct-GGUF`)
2. Premi **Scansiona** — Gideon interroga l'API HuggingFace e lista tutti i file `.gguf` disponibili
3. Ogni file mostra: dimensione, RAM stimata richiesta, badge **COMPATIBILE / TROPPO GRANDE** e una stella sul file consigliato per il tuo hardware
4. Puoi filtrare per quantizzazione (Q4, Q5, Q8, ecc.) e scaricare con un clic
5. In alternativa, **Pull diretto (quant auto)** sceglie e scarica il file migliore in autonomia

### 🔌 Connettore Universale
- Dropdown con i modelli Ollama già installati — selezionane uno per usarlo subito
- Pull da HuggingFace con selezione guidata o automatica
- Il modello connesso diventa attivo per tutte le chat successive

### 🎨 Interfaccia rinnovata
- HUD animato canvas con effetti cyberpunk
- Pannello hardware in tempo reale nella sidebar e nella topbar
- Animazioni fluide sui messaggi
- Badge modalità colorati (Generale / Codice / Sicurezza / Uncensored)
- Tema scuro profondo ottimizzato per lunghe sessioni

### Altre funzionalità
- **Routing intelligente** — rileva automaticamente il tipo di richiesta
- **Streaming in tempo reale** — le risposte appaiono parola per parola
- **Bottone Stop** — interrompe la generazione in qualsiasi momento
- **Download diretto** — scarica il codice generato come `.py`, `.js`, `.rs`, ecc.
- **Data contestuale** — il modello conosce sempre data e ora reale
- **Multi-sessione** — reset conversazione con un clic
- **Privacy totale** — zero dati inviati all'esterno, tutto locale

---

## Requisiti

| | Minimo | Consigliato |
|---|---|---|
| RAM | 4 GB | 16 GB |
| Spazio disco | 5 GB | 20 GB |
| CPU | Qualsiasi moderna | 8+ core |
| OS | Windows 10/11, macOS 12+, Linux | — |

> Con una GPU (anche GTX 1060 / RX 580) la velocità migliora di 10–20x.

---

## Installazione rapida

### Windows

1. Scarica `GideonAI_Export.zip` dalla pagina [Releases](../../releases)
2. Estrai la cartella
3. Doppio clic su **`start.bat`**

Lo script verifica automaticamente se Ollama è installato, lo avvia, scarica i modelli base e lancia l'app. Il browser si aprirà su `http://localhost:5000`.

> **Avvii successivi:** doppio clic diretto su `Gideon AI.exe` oppure usa la shortcut `Avvia Gideon AI.lnk`.

### macOS

1. Installa [Ollama](https://ollama.com/download)
2. Scarica `GideonAI_Export_Mac.zip` dalla pagina [Releases](../../releases)
3. Estrai e lancia:
   ```bash
   chmod +x start.sh
   ./start.sh
   ```

### Linux

```bash
# Installa Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Clona e compila
git clone https://github.com/TUO_USERNAME/gideon-ai
cd gideon-ai
cargo run --release
```

---

## Modelli AI base

Scaricati automaticamente al primo avvio da `start.bat` / `start.sh`:

```bash

| Modello | Comando | Ruolo |
|---------|---------|-------|
| **Gemma 3 (4B)** | `ollama pull gemma3:4b` | Generale / Sicurezza |
| **Qwen 2.5 Coder (7B)** | `ollama pull qwen2.5-coder:7b` | Codice |
| **Nexus Coder (1.5B)** | `ollama pull FableForge-AI/nexus-coder` | Completamento rapido |
| **Dolphin Llama 3.2 (3B)** | `ollama pull dolphinlabs/llama3.2-3b-uncensored` | Uncensored |
```

Puoi aggiungere **qualsiasi modello HuggingFace** direttamente dall'interfaccia — basta incollare l'ID del repo.

---

## Compilare da sorgente

### Prerequisiti

- [Rust](https://rustup.rs/) 1.75+
- [Ollama](https://ollama.com/download)
- Python 3 + Pillow (solo per generare l'icona su Windows)
  ```
  pip install Pillow
  ```

### Windows

```bat
git clone https://github.com/TUO_USERNAME/gideon-ai
cd gideon-ai
build_rust.bat
```

### macOS / Linux

```bash
git clone https://github.com/TUO_USERNAME/gideon-ai
cd gideon-ai
chmod +x build_rust.sh
./build_rust.sh
```

### Avvio rapido (sviluppo)

```bash
cargo run
```

L'app si apre automaticamente su `http://localhost:5000`.

---

## Aggiungere un modello HuggingFace

1. Avvia Gideon AI
2. Nel pannello **🔌 CONNETTORE UNIVERSALE** inserisci l'ID del repo, per esempio:
   ```
   bartowski/Phi-3-mini-4k-instruct-GGUF
   ```
3. Premi **Scansiona** per vedere tutti i file disponibili con compatibilità calcolata
4. Oppure premi **Pull diretto (quant auto)** per lasciare che Gideon scelga in autonomia

Per fare pull manuale da terminale:
```bash
# Sintassi: hf.co/<org>/<repo>:<file.gguf>
ollama pull hf.co/bartowski/Phi-3-mini-4k-instruct-GGUF:Phi-3-mini-4k-instruct-Q4_K_M.gguf
```

---

## Route API interne

| Metodo | Endpoint | Descrizione |
|---|---|---|
| `GET` | `/` | Interfaccia web |
| `POST` | `/chat` | Invia un messaggio (streaming) |
| `POST` | `/reset` | Resetta la cronologia |
| `GET` | `/hw` | Info hardware rilevate |
| `GET` | `/models` | Lista modelli Ollama installati |
| `POST` | `/hf-files` | Lista file GGUF di un repo HuggingFace |
| `POST` | `/connect` | Pull di un modello (Ollama o HuggingFace) |

---

## Struttura del progetto

```
gideon-ai/
├── src/
│   └── main.rs          # Backend Rust + HTML/JS embedded
├── assets/
│   ├── icon.ico
│   ├── icon.svg
│   ├── make_icon.py
│   ├── make_startbat.py
│   ├── make_shortcut.py
│   └── LEGGIMI.txt
├── build.rs             # Incorpora icona nell'exe (Windows)
├── Cargo.toml
├── build_rust.bat       # Build script Windows
└── build_rust.sh        # Build script macOS/Linux
```

---

## Roadmap

- [x] Routing intelligente multi-modello
- [x] Streaming con bottone Stop
- [x] Download codice generato
- [x] Auto-Hardware Detection
- [x] GGUF Picker con compatibilità hardware
- [x] Connettore universale HuggingFace
- [ ] Supporto GPU automatico (CUDA / Metal)
- [ ] Esportazione conversazione in PDF
- [ ] Tema chiaro
- [ ] Supporto Linux completo

---

## 📄 Licenza

Copyright (c) 2026 Rocco Degiacomo

Creative Commons Attribution-NonCommercial 4.0 International (CC BY-NC 4.0)

Con la presente si concede il permesso di utilizzare, copiare, modificare e distribuire questo software e la relativa documentazione esclusivamente per scopi personali, didattici, di ricerca o comunque NON commerciali.

Iscrizione del Copyright e dicitura di attribuzione originale devono essere incluse in tutte le copie o parti sostanziali del Software.

È SEVERAMENTE VIETATO qualsiasi utilizzo commerciale, rivendita, inclusione in prodotti a pagamento o utilizzo a scopo di lucro senza la previa autorizzazione scritta di Rocco Degiacomo.

IL SOFTWARE VIENE FORNITO "COSÌ COM'È", SENZA GARANZIA DI ALCUN TIPO.
---

## Autore

Sviluppato da **Rocco Degiacomo**
