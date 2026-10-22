# Gideon AI

> Assistente AI locale, privato e senza censura — costruito in Rust puro.

![Rust](https://img.shields.io/badge/Backend-Rust-orange?style=flat-square&logo=rust)
![Ollama](https://img.shields.io/badge/Engine-Ollama-black?style=flat-square)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS-blue?style=flat-square)
![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)

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

---

## Funzionalità

- **Routing intelligente** — rileva automaticamente se la richiesta riguarda codice, sicurezza o conversazione e sceglie il modello giusto
- **Streaming in tempo reale** — le risposte appaiono parola per parola, come ChatGPT
- **Bottone Stop** — interrompe la generazione in qualsiasi momento
- **Download diretto** — scarica il codice generato come file `.py`, `.js`, `.rs`, ecc. con un clic
- **Data contestuale** — il modello conosce sempre la data e l'ora reale
- **HUD animato** — interfaccia terminale con canvas animato stile cyberpunk
- **Multi-sessione** — pulsante per resettare la conversazione
- **Privacy totale** — zero dati inviati all'esterno, tutto locale

---

## Requisiti

| | Minimo | Consigliato |
|---|---|---|
| RAM | 8 GB | 16 GB |
| Spazio disco | 10 GB | 15 GB |
| CPU | Qualsiasi moderna | con GPU dedicata |
| OS | Windows 10/11, macOS 12+ | — |

> Con una GPU (anche GTX 1060 / RX 580) la velocità migliora di 10-20x.

---

## Installazione

### Windows

1. Scarica `GideonAI_Export.zip` dalla pagina [Releases](../../releases)
2. Estrai la cartella
3. Doppio clic su **`start.bat`**

Lo script verifica automaticamente se Ollama è installato, lo avvia, scarica i modelli AI e lancia l'app. Il browser si aprirà su `http://localhost:5000`.

> **Avvii successivi:** doppio clic diretto su `Gideon AI.exe` oppure usa la shortcut `Avvia Gideon AI.lnk`.

---

### macOS

1. Installa [Ollama](https://ollama.com/download) se non lo hai già
2. Scarica `GideonAI_Export_Mac.zip` dalla pagina [Releases](../../releases)
3. Estrai la cartella
4. Apri il Terminale nella cartella estratta e lancia:
   ```bash
   chmod +x start.sh
   ./start.sh
   ```
   Oppure doppio clic su **`Avvia Gideon AI.command`**

> **Prima volta:** scarica circa 8GB di modelli, lascia lavorare.

---

## Modelli AI usati

```
ollama pull gemma3:4b
ollama pull qwen2.5-coder:3b
ollama pull dolphinlabs/llama3.2-3b-uncensored
```

Vengono scaricati automaticamente dallo `start.bat` / `start.sh` al primo avvio.

---

## Compilare da sorgente

### Prerequisiti

- [Rust](https://rustup.rs/) (edizione 2021+)
- [Ollama](https://ollama.com/download)
- Python 3 + Pillow (per generare l'icona su Windows)
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

---

## Struttura del progetto

```
gideon-ai/
├── src/
│   └── main.rs          # Backend Rust + HTML/JS embedded
├── assets/
│   ├── icon.ico         # Icona app Windows
│   ├── icon.svg         # Sorgente icona
│   ├── make_icon.py     # Generatore icona
│   ├── make_startbat.py # Generatore script avvio
│   ├── make_shortcut.py # Generatore shortcut Windows
│   └── LEGGIMI.txt      # Guida utente finale
├── build.rs             # Incorpora icona nell'exe (solo Windows)
├── Cargo.toml
├── build_rust.bat       # Build script Windows
└── build_rust.sh        # Build script macOS/Linux
```

---

## Screenshot

> *Aggiungi qui uno screenshot dell'interfaccia*

---

## Roadmap

- [ ] Supporto GPU automatico (CUDA / Metal)
- [ ] Scelta manuale del modello dall'interfaccia
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
