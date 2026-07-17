@echo off
chcp 65001 >nul
title BARON Otomatik Guncellemeli Setup Olusturucu
cd /d "%~dp0"

echo.
echo ==================================================
echo   BARON OTOMATIK GUNCELLEMELI WINDOWS SETUP
echo ==================================================
echo.

where node >nul 2>nul || (
  echo HATA: Node.js kurulu degil.
  pause
  exit /b 1
)
where cargo >nul 2>nul || (
  echo HATA: Rust kurulu degil.
  pause
  exit /b 1
)

if not exist "updater-keys\baron-updater.key" (
  echo HATA: updater-keys\baron-updater.key bulunamadi.
  echo Bu anahtar olmadan imzali otomatik guncelleme paketi olusturulamaz.
  pause
  exit /b 1
)

set "TAURI_SIGNING_PRIVATE_KEY_PATH=%~dp0updater-keys\baron-updater.key"
set "TAURI_SIGNING_PRIVATE_KEY_PASSWORD="

echo [1/3] NPM paketleri kuruluyor...
call npm install
if errorlevel 1 goto error

echo [2/3] Imzali NSIS Setup ve updater dosyalari olusturuluyor...
call npm run build:nsis
if errorlevel 1 goto error

echo [3/3] Tamamlandi.
echo.
echo Setup ve updater dosyalari:
echo src-tauri\target\release\bundle\nsis\
echo.
start "" "%~dp0src-tauri\target\release\bundle\nsis"
pause
exit /b 0

:error
echo.
echo DERLEME BASARISIZ.
echo Microsoft C++ Build Tools, WebView2, Rust ve imza anahtarini kontrol edin.
pause
exit /b 1
