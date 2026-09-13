@echo off
chcp 65001 > nul
cd /d "%~dp0"

powershell -NoProfile -ExecutionPolicy Bypass -Command "$OutputEncoding = [Console]::OutputEncoding = [System.Text.Encoding]::UTF8; .\rust.exe | Tee-Object -FilePath 'РЕЗУЛЬТАТ.txt'"