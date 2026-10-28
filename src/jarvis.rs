use std::process::Command;

/// Rileva ed esegue comandi "Jarvis" dal prompt.
/// Ritorna Some(risposta) se il comando è stato gestito, None altrimenti.
pub async fn handle_command(prompt: &str) -> Option<String> {
    let p = prompt.to_lowercase();
    let cmd = p
        .trim_start_matches("jarvis")
        .trim_start_matches(',')
        .trim_start_matches(' ')
        .trim();

    if cmd.is_empty() {
        return Some("Jarvis in ascolto. Prova: \"Jarvis apri blocco note\".".to_string());
    }

    // Comandi whitelist — ognuno restituisce una stringa di conferma
    let result: Result<String, String> = if cmd.contains("blocco note") || cmd.contains("notepad") {
        Command::new("notepad.exe")
            .spawn()
            .map(|_| "Apro Blocco Note.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("calcolatrice") || cmd.contains("calculator") {
        Command::new("calc.exe")
            .spawn()
            .map(|_| "Calcolatrice aperta.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("terminale") || cmd.contains("prompt") || cmd == "cmd" {
        Command::new("cmd.exe")
            .spawn()
            .map(|_| "Terminale aperto.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("browser") || cmd.contains("google") {
        Command::new("cmd")
            .args(["/C", "start", "https://google.com"])
            .spawn()
            .map(|_| "Accesso rete globale.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("youtube") {
        Command::new("cmd")
            .args(["/C", "start", "https://youtube.com"])
            .spawn()
            .map(|_| "Apro YouTube.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("github") {
        Command::new("cmd")
            .args(["/C", "start", "https://github.com"])
            .spawn()
            .map(|_| "Apro GitHub.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("download") {
        let home = std::env::var("USERPROFILE").unwrap_or_else(|_| r"C:\".to_string());
        let dl = format!(r"{}\Downloads", home);
        Command::new("explorer.exe")
            .arg(&dl)
            .spawn()
            .map(|_| "Apro Download.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("wordcraft") {
        Command::new(r"C:\Program Files\WordCraft\wordcraft.exe")
            .spawn()
            .map(|_| "WordCraft avviato.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("gridcraft") {
        Command::new(r"C:\Program Files\GridCraft\gridcraft.exe")
            .spawn()
            .map(|_| "GridCraft avviato.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("pdfcraft") {
        Command::new(r"C:\Program Files\PdfCraft\pdfcraft.exe")
            .spawn()
            .map(|_| "PdfCraft avviato.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("screenshot") || cmd.contains("cattura") {
        Command::new("powershell")
            .args([
                "-Command",
                "Add-Type -AssemblyName System.Windows.Forms; \
                 [System.Windows.Forms.SendKeys]::SendWait('{PRTSC}')",
            ])
            .spawn()
            .map(|_| "Screenshot negli appunti.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("muto") || cmd.contains("silenzia") {
        Command::new("powershell")
            .args([
                "-Command",
                "$obj = New-Object -ComObject WScript.Shell; $obj.SendKeys([char]173)",
            ])
            .spawn()
            .map(|_| "Audio silenziato.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("blocca") || cmd.contains("lock") {
        Command::new("rundll32.exe")
            .args(["user32.dll,LockWorkStation"])
            .spawn()
            .map(|_| "Blocco il sistema.".to_string())
            .map_err(|e| e.to_string())
    } else if cmd.contains("spegni") || cmd.contains("shutdown") {
        return Some(
            "⚠️ Per spegnere, usa il menu di Windows. Non eseguo spegnimenti automatici."
                .to_string(),
        );
    } else {
        return Some(format!(
            "❌ Jarvis: comando '{}' non riconosciuto. Prova: blocco note, calcolatrice, browser, youtube, github, wordcraft, gridcraft, pdfcraft, download, terminale, screenshot, muto, blocca.",
            cmd
        ));
    };

    match result {
        Ok(msg) => Some(format!("✅ Jarvis: {}", msg)),
        Err(e) => Some(format!("❌ Jarvis: errore esecuzione: {}", e)),
    }
}