@echo off
REM PoSME Benchmark Runner (Windows)
REM Just double-click this file to run.
REM Pre-compiled binary included. Falls back to compilation if needed.

cd /d "%~dp0"

if exist posme-bench-windows-x64.exe (
    echo.
    echo   Running benchmark (this takes 1-5 minutes) ...
    echo.
    posme-bench-windows-x64.exe
    pause
    exit /b 0
)

where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo.
    echo   Pre-compiled binary not found and Rust compiler missing.
    echo   Downloading rustup installer ...
    echo.
    powershell -Command "Invoke-WebRequest -Uri 'https://win.rustup.rs/x86_64' -OutFile 'rustup-init.exe'"
    rustup-init.exe -y --quiet
    set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
)

echo.
echo   Compiling posme-bench.rs ...
rustc -O posme-bench.rs -o posme-bench.exe
if %errorlevel% neq 0 (
    echo   Compilation failed.
    pause
    exit /b 1
)

echo   Running benchmark (this takes 1-5 minutes) ...
echo.
posme-bench.exe

pause
