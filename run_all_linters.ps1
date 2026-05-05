# run_all_linters.ps1

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  ЗАПУСК ЛИНТЕРОВ ДЛЯ ЛАБОРАТОРНОЙ" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# --- GO ---
Write-Host "`n[1/6] Переход в проект Go..." -ForegroundColor Yellow
cd C:\Users\$env:USERNAME\Desktop\go-lint-lab

Write-Host "[2/6] Запуск go vet..." -ForegroundColor Yellow
go vet ./... 2>&1 | Out-File govet_report.txt -Encoding UTF8
Write-Host "  -> govet_report.txt" -ForegroundColor Green

Write-Host "[3/6] Запуск staticcheck..." -ForegroundColor Yellow
staticcheck ./... 2>&1 | Out-File staticcheck_report.txt -Encoding UTF8
Write-Host "  -> staticcheck_report.txt" -ForegroundColor Green

Write-Host "[4/6] Запуск golangci-lint..." -ForegroundColor Yellow
golangci-lint run ./... 2>&1 | Out-File golangci_report.txt -Encoding UTF8
Write-Host "  -> golangci_report.txt" -ForegroundColor Green

# --- RUST ---
Write-Host "[5/6] Переход в проект Rust..." -ForegroundColor Yellow
cd C:\Users\$env:USERNAME\Desktop\rust-lint-lab

Write-Host "[6/6] Запуск Clippy..." -ForegroundColor Yellow
cargo clippy 2>&1 | Out-File clippy_report.txt -Encoding UTF8
cargo clippy -- -W clippy::all -W clippy::pedantic 2>&1 | Out-File clippy_full_report.txt -Encoding UTF8
Write-Host "  -> clippy_report.txt" -ForegroundColor Green
Write-Host "  -> clippy_full_report.txt" -ForegroundColor Green

# --- РЕЗУЛЬТАТЫ ---
Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "  ГОТОВО! ВСЕ ОТЧЁТЫ СОХРАНЕНЫ" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

Write-Host "`nGo отчёты:" -ForegroundColor Yellow
Write-Host "  govet_report.txt       — строк: " -NoNewline
(Get-Content govet_report.txt | Where-Object { $_ -ne "" }).Count

Write-Host "  staticcheck_report.txt — строк: " -NoNewline
(Get-Content staticcheck_report.txt | Where-Object { $_ -ne "" }).Count

Write-Host "  golangci_report.txt    — строк: " -NoNewline
(Get-Content golangci_report.txt | Where-Object { $_ -ne "" }).Count

Write-Host "`nRust отчёты:" -ForegroundColor Yellow
Write-Host "  clippy_report.txt      — warnings: " -NoNewline
(Select-String -Path clippy_report.txt -Pattern "warning:").Count

Write-Host "  clippy_full_report.txt — warnings: " -NoNewline
(Select-String -Path clippy_full_report.txt -Pattern "warning:").Count

Write-Host "`nНажмите Enter для выхода..." -ForegroundColor Gray
Read-Host
