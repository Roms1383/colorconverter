set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set dotenv-load

@dir FOLDER:
    if (!(Test-Path "{{FOLDER}}")) { [void](New-Item "{{FOLDER}}" -ItemType Directory); Write-Host "Created folder at {{FOLDER}}"; }

@copy FILE TO:
    if (Test-Path "{{FILE}}" -PathType leaf) { Copy-Item -Path '{{FILE}}' -Destination '{{TO}}' -Force } else {  Write-Host "missing {{FILE}}"; }

build:
    cargo build --release
    just dir '{{ join("red4ext", "plugins", "colorconverter") }}'
    just copy '{{ join("target", "release", "colorconverter.dll") }}' '{{ join("red4ext", "plugins", "colorconverter") }}'