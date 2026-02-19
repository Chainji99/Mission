@echo off
echo ==========================================
echo   XUE Project - Zen Minimalist Starter
echo ==========================================

:: Check if server is already running on 3000
netstat -ano | findstr :3000 > nul
if %errorlevel% == 0 (
    echo [!] Server is already running on port 3000.
) else (
    echo [1/2] Starting Backend (Rust)...
    cd server
    start "XUE Backend" cargo run
    cd ..
)

:: Check if client is already running on 4200
netstat -ano | findstr :4200 > nul
if %errorlevel% == 0 (
    echo [!] Frontend is already running on port 4200.
) else (
    echo [2/2] Starting Frontend (Angular)...
    cd client
    start "XUE Frontend" npm start
    cd ..
)

echo.
echo ==========================================
echo   System is starting in new windows...
echo   Backend: http://localhost:3000
echo   Frontend: http://localhost:4200
echo ==========================================
pause
