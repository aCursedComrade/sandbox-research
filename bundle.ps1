"[*] Just a simple script to prep a project bundle"

$bProfile = $args[0]
switch -Regex ($bProfile) {
    "release|debug" { 
        "[*] You may need to move the BoxDrv package manually"
        if ($false -eq (Test-Path ".\bundle")) { New-Item -ItemType Directory ".\bundle" }
        if ($false -eq (Test-Path ".\bundle\utils")) { New-Item -ItemType Directory ".\bundle\utils" }

        Copy-Item ".\setup.ps1" ".\bundle"
        Copy-Item ".\target\x86_64-pc-windows-msvc\$($bProfile)\box_ui.exe" ".\bundle"
        Copy-Item ".\target\x86_64-pc-windows-msvc\$($bProfile)\box_broker.exe" ".\bundle"
        Copy-Item ".\target\x86_64-pc-windows-msvc\$($bProfile)\hooked.dll" ".\bundle\utils"
        Copy-Item ".\target\x86_64-pc-windows-msvc\$($bProfile)\whisper.exe" ".\bundle\utils"
        "[+] Done"
    }
    Default {
        "[!] Usage: $($MyInvocation.MyCommand.Name) {release|debug}"
    }
}
