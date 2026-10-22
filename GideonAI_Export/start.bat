@echo off
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

:: Download modelli se mancanti (al primo avvio possono volerci minuti)
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
