# Jack-Do Installation Script (Windows)

$InstallDir = Join-Path $HOME ".jack-do"
$BinDir = Join-Path $InstallDir "bin"
$ExecName = "jack-do.exe"
$DestPath = Join-Path $BinDir $ExecName

Write-Host "🐦 Installing Jack-Do..." -ForegroundColor Cyan

# 1. Build the project
Write-Host "📦 Building in release mode..."
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build Jack-Do."
    exit $LASTEXITCODE
}

# 2. Create directory structure
if (!(Test-Path $BinDir)) {
    New-Item -ItemType Directory -Path $BinDir -Force | Out-Null
}

# 3. Copy binary
Write-Host "🚚 Copying binary to $BinDir..."
Copy-Item "target\release\$ExecName" $DestPath -Force

# 4. Add to PATH for the current user
$ExistingPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
if ($ExistingPath -split ";" -notcontains $BinDir) {
    Write-Host "🔗 Adding $BinDir to User PATH..."
    $NewPath = "$BinDir;$ExistingPath"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, [EnvironmentVariableTarget]::User)
    Write-Host "✅ PATH updated. You may need to restart your terminal." -ForegroundColor Green
} else {
    Write-Host "✅ $BinDir is already in PATH." -ForegroundColor Green
}

Write-Host "🎉 Jack-Do installed successfully! Try running 'jack-do --help'" -ForegroundColor Green
