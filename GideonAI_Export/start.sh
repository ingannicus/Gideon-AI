#!/bin/bash
echo ""
echo "  ==================================================="
echo "    GIDEON AI - Avvio in corso..."
echo "  ==================================================="
echo ""

# Verifica se Ollama è installato
if ! command -v ollama &> /dev/null; then
    echo "  [!] Ollama non trovato."
    echo "  [!] Aprendo la pagina di download..."
    open "https://ollama.com/download" 2>/dev/null || xdg-open "https://ollama.com/download" 2>/dev/null
    echo "  Dopo aver installato Ollama, riavvia questo start.sh"
    read -p "  Premi INVIO per uscire..."
    exit 1
fi

# Avvia Ollama in background se non già attivo
if ! pgrep -x "ollama" > /dev/null; then
    echo "  [*] Avvio Ollama in background..."
    ollama serve &> /dev/null &
    sleep 3
else
    echo "  [*] Ollama già in esecuzione."
fi

# Download modelli se mancanti
echo ""
echo "  [*] Verifica modelli AI (al primo avvio scarica alcuni GB, attendi)..."
ollama pull gemma3:4b
ollama pull qwen2.5-coder:3b
ollama pull dolphinlabs/llama3.2-3b-uncensored

# Avvia Gideon AI
echo ""
echo "  [*] Avvio Gideon AI..."
DIR="$(cd "$(dirname "$0")" && pwd)"
"$DIR/gideon-ai" &
sleep 2
echo "  [*] Il browser si aprirà automaticamente su http://localhost:5000"
echo ""
