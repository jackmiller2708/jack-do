# Jack-Do Installation Script (Windows)

$InstallDir = Join-Path $HOME ".jack-do"
$BinDir = Join-Path $InstallDir "bin"
$ExecName = "jack-do.exe"
$DestPath = Join-Path $BinDir $ExecName

function Write-Log {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Message,
        [Parameter(Mandatory = $true)]
        [ValidateSet("INFO", "SUCCESS", "WARNING", "ERROR")]
        [string]$Level
    )

    $Color = switch ($Level) {
        "INFO" { "Cyan" }
        "SUCCESS" { "Green" }
        "WARNING" { "Yellow" }
        "ERROR" { "Red" }
    }

    $Symbol = switch ($Level) {
        "INFO" { "(i)" }
        "SUCCESS" { "(+)" }
        "WARNING" { "(!)" }
        "ERROR" { "(x)" }
    }

    Write-Host "$Symbol [$Level] $Message" -ForegroundColor $Color
}

function Check-Dependencies {
    Write-Log "Checking dependencies..." "INFO"
    
    $Missing = @()
    if (!(Get-Command "rustc" -ErrorAction SilentlyContinue)) { $Missing += "rustc" }
    if (!(Get-Command "cargo" -ErrorAction SilentlyContinue)) { $Missing += "cargo" }

    if ($Missing.Count -gt 0) {
        Write-Log "Missing dependencies: $($Missing -join ', ')" "ERROR"
        Write-Host "`nPlease install Rust and Cargo from https://rustup.rs/ before continuing." -ForegroundColor White
        exit 1
    }
    Write-Log "Dependencies satisfied." "SUCCESS"
}

function Cleanup-OnFailure {
    Write-Log "Installation encountered an error. Initiating recovery..." "WARNING"
    if (Test-Path $DestPath) {
        Write-Log "Removing partial binary: $DestPath" "INFO"
        Remove-Item $DestPath -Force -ErrorAction SilentlyContinue
    }
    # We won't remove the entire directory in case the user had other things there, 
    # but we'll log what happened.
    Write-Log "Recovery complete. Please check the error above and try again." "INFO"
}

function Install-JackDo {
    try {
        Check-Dependencies

        Write-Log "Building Jack-Do in release mode..." "INFO"
        cargo build --release
        if ($LASTEXITCODE -ne 0) {
            throw "Cargo build failed with exit code $LASTEXITCODE. Ensure you have a stable internet connection and valid Rust installation."
        }

        Write-Log "Setting up directory structure..." "INFO"
        if (!(Test-Path $BinDir)) {
            New-Item -ItemType Directory -Path $BinDir -Force | Out-Null
        }

        Write-Log "Installing binary to $BinDir..." "INFO"
        if (!(Test-Path "target\release\$ExecName")) {
            throw "Could not find compiled binary at target\release\$ExecName"
        }
        Copy-Item "target\release\$ExecName" $DestPath -Force

        Write-Log "Configuring User PATH..." "INFO"
        $ExistingPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
        if ($ExistingPath -split ";" -notcontains $BinDir) {
            $NewPath = "$BinDir;$ExistingPath"
            [Environment]::SetEnvironmentVariable("Path", $NewPath, [EnvironmentVariableTarget]::User)
            Write-Log "PATH updated. You will need to RESTART your terminal to use 'jack-do'." "SUCCESS"
        }
        else {
            Write-Log "$BinDir is already in PATH." "SUCCESS"
        }

        Write-Log "Jack-Do installed successfully!" "SUCCESS"
        Write-Host "Try running: jack-do --help" -ForegroundColor Gray
    }
    catch {
        Write-Log "Installation failed: $($_.Exception.Message)" "ERROR"
        Cleanup-OnFailure
        exit 1
    }
}

# Start installation
Install-JackDo
