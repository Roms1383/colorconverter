set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# create folder
@dir FOLDER:
    if (!(Test-Path "{{FOLDER}}" -PathType container)) { [void](New-Item "{{FOLDER}}" -ItemType Directory); Write-Host "Created folder at {{FOLDER}}"; }

# copy file to another folder
@copy FILE TO:
    if (Test-Path "{{FILE}}" -PathType leaf) { Copy-Item -Path '{{FILE}}' -Destination '{{TO}}' -Force } else {  Write-Host "missing {{FILE}}"; }

# copy folder and its content to another folder
@dcopy FOLDER TO:
    if (Test-Path "{{FOLDER}}" -PathType container) { Copy-Item -Path '{{FOLDER}}' -Destination '{{TO}}' -Force -Recurse } else {  Write-Host "missing {{FOLDER}}"; }

# move a folder to another folder
@move FOLDER TO:
    if (!(Test-Path "{{FOLDER}}" -PathType container)) { throw "missing source {{FOLDER}}"; }
    if (!(Test-Path "{{TO}}" -PathType container)) { throw "missing destination {{TO}}"; }
    Move-Item -Path '{{FOLDER}}' -Destination '{{TO}}'

# check compilation
check:
    cargo check

# lint code
qa:
    @cargo clippy -- -D warnings
    @cargo fix
    @cargo fmt --check

# run tests
test:
    @cargo test

# build binary
build:
    cargo build --release
    just dir '{{ join("red4ext", "plugins", "colorconverter") }}'
    just copy '{{ join("target", "release", "colorconverter.dll") }}' '{{ join("red4ext", "plugins", "colorconverter") }}'

# bundle files in Github Workflow
bundle: (build)
    just dir 'bundle'
    just dcopy 'r6' 'bundle' 
    just move 'red4ext' 'bundle'
