"[*] Just a simple script to prep a release bundle"

$bProfile = $args[0]
switch -Regex ($bProfile) {
    "release|debug" { 
        "[*] You may need to move the BoxDrv package manually"
        if ($false -eq (Test-Path ".\TheBox")) { New-Item -ItemType Directory ".\TheBox" }
        if ($false -eq (Test-Path ".\TheBox\utils")) { New-Item -ItemType Directory ".\TheBox\utils" }

        Copy-Item ".\setup.ps1" ".\TheBox"
        Copy-Item ".\target\x86_64-pc-windows-msvc\$($bProfile)\box_ui.exe" ".\TheBox"
        Copy-Item ".\target\x86_64-pc-windows-msvc\$($bProfile)\box_broker.exe" ".\TheBox"
        Copy-Item ".\target\x86_64-pc-windows-msvc\$($bProfile)\hooked.dll" ".\TheBox\utils"
        Copy-Item ".\target\x86_64-pc-windows-msvc\$($bProfile)\whisper.exe" ".\TheBox\utils"
        "[+] Done"
    }
    Default {
        "[!] Usage: $($MyInvocation.MyCommand.Name) {release|debug}"
    }
}
