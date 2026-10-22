"""
Genera start.bat (Windows) e start.sh (Mac/Linux) nella cartella di export.
Chiamato da build_rust.bat / build_rust.sh.
"""
import sys
import os
import stat

out_dir = sys.argv[1] if len(sys.argv) > 1 else "GideonAI_Export"
os.makedirs(out_dir, exist_ok=True)

# ── Windows: start.bat ────────────────────────────────────────────────────────
bat_content = r"""@echo off
title GIDEON AI - Avvio
color 0A
echo.
echo  ===================================================
echo    GIDEON AI - Avvio in corso...
echo  ===================================================
echo.

:: Verifica se Ollama e' installato
ollama --version >nul 2>&1
if errorlevel 1 (
    echo  [!] Ollama non trovato sul tuo PC.
    echo  [!] Aprendo la pagina di download...
    echo.
    start https://ollama.com/download
    echo  Dopo aver installato Ollama, riavvia questo start.bat.
    pause
    exit /b
)

:: Avvia Ollama in background se non gia' attivo
tasklist /fi "imagename eq ollama.exe" 2>nul | find /i "ollama.exe" >nul
if errorlevel 1 (
    echo  [*] Avvio Ollama in background...
    start /b ollama serve
    timeout /t 3 /nobreak >nul
) else (
    echo  [*] Ollama gia' in esecuzione.
)

:: Download modelli se mancanti
echo.
echo  [*] Verifica modelli AI (al primo avvio scarica alcuni GB, attendi)...
ollama pull gemma3:4b
ollama pull qwen2.5-coder:3b
ollama pull dolphinlabs/llama3.2-3b-uncensored

:: Avvia Gideon AI
echo.
echo  [*] Avvio Gideon AI...
start "" "Gideon AI.exe"
echo  [*] Il browser si aprira' automaticamente su http://localhost:5000
echo.
timeout /t 3 /nobreak >nul
"""

bat_path = os.path.join(out_dir, "start.bat")
with open(bat_path, "w", encoding="utf-8") as f:
    f.write(bat_content)
print(f"[OK] start.bat creato in: {bat_path}")

# ── Mac/Linux: start.sh ───────────────────────────────────────────────────────
sh_content = """#!/bin/bash
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
"""

sh_path = os.path.join(out_dir, "start.sh")
with open(sh_path, "w", encoding="utf-8", newline="\n") as f:
    f.write(sh_content)
# Rende eseguibile
os.chmod(sh_path, os.stat(sh_path).st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
print(f"[OK] start.sh creato in: {sh_path}")
