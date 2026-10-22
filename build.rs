fn main() {
    // Incorpora l'icona nell'exe solo su Windows
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("FileDescription", "Gideon AI - Assistente tecnico locale");
        res.set("ProductName", "Gideon AI");
        res.set("CompanyName", "Rocco Degiacomo");
        res.set("LegalCopyright", "© 2025 Rocco Degiacomo");
        res.set("FileVersion", "1.0.0.0");
        res.set("ProductVersion", "1.0.0.0");
        if let Err(e) = res.compile() {
            eprintln!("[build.rs] Attenzione icona: {}", e);
        }
    }
}
