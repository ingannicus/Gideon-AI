"""
Crea la shortcut 'Avvia Gideon AI.lnk' nella cartella di export.
Chiamato da build_rust.bat.
Uso: python make_shortcut.py <cartella_export_assoluta>
"""
import sys
import os

if len(sys.argv) < 2:
    print("[!] Uso: python make_shortcut.py <cartella_export>")
    sys.exit(1)

export_dir = os.path.abspath(sys.argv[1])
exe_path   = os.path.join(export_dir, "Gideon AI.exe")
lnk_path   = os.path.join(export_dir, "Avvia Gideon AI.lnk")

try:
    import win32com.client
    shell = win32com.client.Dispatch("WScript.Shell")
    shortcut = shell.CreateShortcut(lnk_path)
    shortcut.TargetPath     = exe_path
    shortcut.WorkingDirectory = export_dir
    shortcut.Description    = "Gideon AI - Assistente tecnico locale"
    shortcut.Save()
    print(f"[OK] Shortcut creata: {lnk_path}")
except ImportError:
    # Fallback via PowerShell se pywin32 non e' disponibile
    import subprocess
    ps_cmd = (
        f"$s=(New-Object -COM WScript.Shell).CreateShortcut('{lnk_path}');"
        f"$s.TargetPath='{exe_path}';"
        f"$s.WorkingDirectory='{export_dir}';"
        f"$s.Description='Gideon AI - Assistente tecnico locale';"
        f"$s.Save()"
    )
    result = subprocess.run(
        ["powershell", "-NoProfile", "-Command", ps_cmd],
        capture_output=True, text=True
    )
    if result.returncode == 0:
        print(f"[OK] Shortcut creata via PowerShell: {lnk_path}")
    else:
        print(f"[!] Shortcut fallita: {result.stderr.strip()}")
        sys.exit(1)
