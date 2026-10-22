@echo off
title GIDEON AI - Builder Rust
color 0B
echo.
echo  =========================================================
echo    GIDEON AI - COMPILAZIONE E PACCHETTIZZAZIONE
echo    Sviluppato da Rocco Degiacomo
echo  =========================================================
echo.

:: ── Verifica Rust ──────────────────────────────────────────
cargo --version >nul 2>&1
if errorlevel 1 (
    echo  [!] ERRORE: Rust non e installato.
    echo  [!] Scaricalo da https://rustup.rs/ e riprova.
    echo.
    pause
    exit /b 1
)

:: ── Genera icona con Python ────────────────────────────────
echo  [*] Generazione icona...
python assets\make_icon.py
if errorlevel 1 (
    echo  [!] Attenzione: icona non generata. Continuazione senza icona.
)

:: ── Compilazione release ───────────────────────────────────
echo.
echo  [*] Compilazione in corso (attendere)...
echo.
cargo build --release
if errorlevel 1 (
    echo.
    echo  [!] Errore compilazione. Controlla i messaggi sopra.
    pause
    exit /b 1
)

:: ── Preparazione cartella export ──────────────────────────
set "DIR=GideonAI_Export"
if exist "%DIR%" rmdir /s /q "%DIR%"
mkdir "%DIR%"

echo.
echo  [*] Copia file...

copy /y "target\release\gideon-ai.exe" "%DIR%\Gideon AI.exe" >nul
if errorlevel 1 (
    echo  [!] Errore: exe non trovato in target\release\
    pause
    exit /b 1
)

copy /y "assets\LEGGIMI.txt" "%DIR%\LEGGIMI.txt" >nul

:: ── Genera start.bat via Python ────────────────────────────
echo  [*] Creazione start.bat...
python assets\make_startbat.py "%DIR%"
if errorlevel 1 (
    echo  [!] Errore creazione start.bat
    pause
    exit /b 1
)

:: ── Crea shortcut .lnk via Python ─────────────────────────
echo  [*] Creazione shortcut...
python assets\make_shortcut.py "%CD%\%DIR%"
if errorlevel 1 (
    echo  [!] Shortcut non creata, ma il pacchetto e comunque pronto.
)

:: ── Crea archivio ZIP ──────────────────────────────────────
echo  [*] Creazione archivio ZIP...
if exist "%DIR%.zip" del "%DIR%.zip"
powershell -NoProfile -Command "Compress-Archive -Path '%DIR%' -DestinationPath '%DIR%.zip' -Force"
if errorlevel 1 (
    echo  [!] ZIP fallito. La cartella e comunque pronta.
) else (
    echo  [*] ZIP creato: %DIR%.zip
)

:: ── Riepilogo ──────────────────────────────────────────────
echo.
echo  =========================================================
echo   COMPLETATO!
echo.
echo   Cartella:  %DIR%\
echo     - Gideon AI.exe         l app con icona
echo     - start.bat             primo avvio e scarica modelli
echo     - Avvia Gideon AI.lnk   shortcut per il desktop
echo     - LEGGIMI.txt
echo.
echo   Archivio:  %DIR%.zip      distribuisci questo
echo  =========================================================
echo.
pause
