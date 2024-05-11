Write-Host "[!] This script will fail if you are not running from a elevated prompt" -ForegroundColor Yellow

$action = $args[0]
switch ($action) {
    "install" {
        "[*] Setting up and starting services"
        "[*] BoxDrv"
        sc.exe create BoxDrv binpath= $(Resolve-Path ".\BoxDrv\BoxDrv.sys") type= kernel
        sc.exe start BoxDrv
        "[*] BoxBroker"
        sc.exe create BoxBroker binpath= $(Resolve-Path ".\box_broker.exe") type= userown obj= $env:USERNAME
        sc.exe start BoxBroker
        "[+] You can now start the $(Resolve-Path ".\box_ui.exe") binary to start interacting"
    }
    "remove" {
        "[*] Removing all services"
        "[*] BoxDrv"
        sc.exe stop BoxDrv
        sc.exe delete BoxDrv
        "[*] BoxBroker"
        sc.exe stop BoxBroker
        sc.exe delete BoxBroker
        "[+] All services cleaned up"
    }
    Default {
        "[!] Usage: $($MyInvocation.MyCommand.Name) {install|remove}"
    }
}
