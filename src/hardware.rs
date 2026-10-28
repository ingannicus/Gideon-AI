use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Clone, Serialize)]
pub struct HardwareInfo {
    pub cpu_brand: String,
    pub cpu_cores: usize,
    pub total_ram_gb: f32,
    pub available_ram_gb: f32,
    pub gpu_info: String,
    pub recommended_model: String,
    pub recommended_threads: u32,
    pub recommended_ctx: u32,
    pub tier: String,
    pub recommended_quant: String,
    pub max_model_ram_gb: f32,
}

impl HardwareInfo {
    pub fn detect(nexus_model: &str, gemma_model: &str) -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_brand = sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());

        let cpu_cores = sys.cpus().len();
        let total_ram_gb = sys.total_memory() as f32 / 1_073_741_824.0;
        let available_ram_gb = sys.available_memory() as f32 / 1_073_741_824.0;

        let gpu_info = detect_gpu();
        let max_model_ram_gb = (available_ram_gb - 2.0).max(0.5);

        let (tier, recommended_model, recommended_threads, recommended_ctx) =
            if available_ram_gb >= 14.0 && cpu_cores >= 8 {
                ("high", gemma_model.to_string(), 8u32, 8192u32)
            } else if available_ram_gb >= 6.0 && cpu_cores >= 4 {
                ("mid", nexus_model.to_string(), 4u32, 4096u32)
            } else {
                ("low", nexus_model.to_string(), 2u32, 2048u32)
            };

        let recommended_quant = best_quant_for_ram(max_model_ram_gb).to_string();

        HardwareInfo {
            cpu_brand,
            cpu_cores,
            total_ram_gb,
            available_ram_gb,
            gpu_info,
            recommended_model,
            recommended_threads,
            recommended_ctx,
            tier: tier.to_string(),
            recommended_quant,
            max_model_ram_gb,
        }
    }
}

pub fn best_quant_for_ram(ram_gb: f32) -> &'static str {
    if ram_gb >= 12.0 { "Q6_K" }
    else if ram_gb >= 8.0 { "Q6_K" }
    else if ram_gb >= 5.5 { "Q5_K_M" }
    else if ram_gb >= 3.5 { "Q4_K_M" }
    else if ram_gb >= 2.5 { "Q3_K_M" }
    else { "IQ2_M" }
}

pub fn estimate_ram_for_gguf(filename: &str) -> f32 {
    let f = filename.to_lowercase();
    let params_b: f32 = if f.contains("70b") { 70.0 }
        else if f.contains("34b") { 34.0 }
        else if f.contains("22b") { 22.0 }
        else if f.contains("13b") { 13.0 }
        else if f.contains("8b")  { 8.0 }
        else if f.contains("7b")  { 7.0 }
        else if f.contains("4b")  { 4.0 }
        else if f.contains("3b")  { 3.0 }
        else if f.contains("2b")  { 2.0 }
        else if f.contains("1b")  { 1.0 }
        else { 4.0 };

    let quant_factor: f32 = if f.contains("q8") || f.contains("f16") { 1.0 }
        else if f.contains("q6") { 0.75 }
        else if f.contains("q5") { 0.625 }
        else if f.contains("q4") { 0.5 }
        else if f.contains("q3") { 0.375 }
        else if f.contains("q2") || f.contains("iq2") { 0.25 }
        else { 0.5 };

    params_b * quant_factor + 0.5
}

fn detect_gpu() -> String {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(out) = Command::new("wmic")
            .args(["path", "win32_VideoController", "get", "name"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            let lines: Vec<&str> = s
                .lines()
                .filter(|l| !l.trim().is_empty() && !l.trim().eq_ignore_ascii_case("name"))
                .collect();
            if !lines.is_empty() {
                return lines.join(", ");
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        if let Ok(out) = Command::new("lspci").output() {
            let s = String::from_utf8_lossy(&out.stdout);
            let gpus: Vec<&str> = s
                .lines()
                .filter(|l| l.to_lowercase().contains("vga") || l.to_lowercase().contains("3d"))
                .collect();
            if !gpus.is_empty() {
                return gpus.join("; ");
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        if let Ok(out) = Command::new("system_profiler")
            .args(["SPDisplaysDataType", "-detailLevel", "mini"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines() {
                if line.to_lowercase().contains("chipset model") || line.to_lowercase().contains("gpu") {
                    return line.trim().to_string();
                }
            }
        }
    }
    "GPU non rilevata".to_string()
}