#!/bin/bash
echo ""
echo "  ========================================================="
echo "    GIDEON AI - COMPILAZIONE E PACCHETTIZZAZIONE (Mac/Linux)"
echo "    Sviluppato da Rocco Degiacomo"
echo "  ========================================================="
echo ""

# ── Verifica Rust ────────────────────────────────────────────
if ! command -v cargo &> /dev/null; then
    echo "  [!] ERRORE: Rust non è installato."
    echo "  [!] Scaricalo da https://rustup.rs/ e riprova."
    exit 1
fi

# ── Genera icona (opzionale su Mac, usata solo su Windows) ──
if command -v python3 &> /dev/null; then
    echo "  [*] Generazione icona..."
    python3 assets/make_icon.py || echo "  [!] Icona non generata, continuo comunque."
fi

# ── Compilazione release ─────────────────────────────────────
echo ""
echo "  [*] Compilazione in corso (attendere)..."
echo ""
cargo build --release
if [ $? -ne 0 ]; then
    echo ""
    echo "  [!] Errore compilazione. Controlla i messaggi sopra."
    exit 1
fi

# ── Preparazione cartella export ────────────────────────────
DIR="GideonAI_Export_Mac"
rm -rf "$DIR"
mkdir -p "$DIR"

echo ""
echo "  [*] Copia file..."

cp "target/release/gideon-ai" "$DIR/gideon-ai"
chmod +x "$DIR/gideon-ai"
cp "assets/LEGGIMI.txt" "$DIR/LEGGIMI.txt"

# ── Genera start.sh via Python ───────────────────────────────
echo "  [*] Creazione start.sh..."
python3 assets/make_startbat.py "$DIR"
if [ $? -ne 0 ]; then
    echo "  [!] Errore creazione start.sh"
    exit 1
fi

# ── Crea file .command (doppio clic su Mac) ──────────────────
echo "  [*] Creazione GideonAI.command..."
COMMAND_FILE="$DIR/Avvia Gideon AI.command"
cat > "$COMMAND_FILE" << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
./start.sh
EOF
chmod +x "$COMMAND_FILE"

# ── Crea archivio ZIP ────────────────────────────────────────
echo "  [*] Creazione archivio ZIP..."
zip -r "${DIR}.zip" "$DIR" > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "  [*] ZIP creato: ${DIR}.zip"
else
    echo "  [!] ZIP fallito. La cartella è comunque pronta."
fi

# ── Riepilogo ────────────────────────────────────────────────
echo ""
echo "  ========================================================="
echo "   COMPLETATO!"
echo ""
echo "   Cartella:  $DIR/"
echo "     - gideon-ai              l'app"
echo "     - start.sh               primo avvio (scarica modelli)"
echo "     - Avvia Gideon AI.command doppio clic per avviare"
echo "     - LEGGIMI.txt"
echo ""
echo "   Archivio:  ${DIR}.zip      distribuisci questo"
echo "  ========================================================="
echo ""
