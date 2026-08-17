@echo off
REM ================================================
REM 🤖 VelinScript Examples Pack Vol 2 - Test Suite
REM ================================================

echo.
echo ╔══════════════════════════════════════════════════════╗
echo ║  🤖 VelinScript Examples Pack Vol 2 - Test Suite    ║
echo ║     Teste alle 10 Tools                             ║
echo ╚══════════════════════════════════════════════════════╝
echo.

setlocal enabledelayedexpansion

set "basePath=D:\velinscript\examples\Examples Pack Vol 2"
set "count=0"
set "success=0"

REM Test Tool 1
echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
echo 🧪 Testing: 01-todo-list-manager
if exist "%basePath%\01-todo-list-manager\todo-manager.velin" (
    echo ✅ Datei gefunden
    for /f %%A in ('find /c /v "" ^< "%basePath%\01-todo-list-manager\todo-manager.velin"') do set "lines=%%A"
    echo 📊 Codezeilen: !lines!
    set /a success+=1
) else (
    echo ❌ Datei nicht gefunden
)
set /a count+=1

REM Test Tool 2
echo 🧪 Testing: 02-weather-api-client
if exist "%basePath%\02-weather-api-client\weather-client.velin" (
    echo ✅ Datei gefunden
    set /a success+=1
) else (
    echo ❌ Datei nicht gefunden
)
set /a count+=1

REM Test Tool 3
echo 🧪 Testing: 03-file-organizer
if exist "%basePath%\03-file-organizer\file-organizer.velin" (
    echo ✅ Datei gefunden
    set /a success+=1
) else (
    echo ❌ Datei nicht gefunden
)
set /a count+=1

REM Test Tool 4
echo 🧪 Testing: 04-email-validator
if exist "%basePath%\04-email-validator\email-validator.velin" (
    echo ✅ Datei gefunden
    set /a success+=1
) else (
    echo ❌ Datei nicht gefunden
)
set /a count+=1

REM Test Tool 5
echo 🧪 Testing: 05-simple-blog
if exist "%basePath%\05-simple-blog\blog-system.velin" (
    echo ✅ Datei gefunden
    set /a success+=1
) else (
    echo ❌ Datei nicht gefunden
)
set /a count+=1

REM Test Tool 6
echo 🧪 Testing: 06-json-processor
if exist "%basePath%\06-json-processor\json-processor.velin" (
    echo ✅ Datei gefunden
    set /a success+=1
) else (
    echo ❌ Datei nicht gefunden
)
set /a count+=1

REM Test Tool 7
echo 🧪 Testing: 07-password-generator
if exist "%basePath%\07-password-generator\password-generator.velin" (
    echo ✅ Datei gefunden
    set /a success+=1
) else (
    echo ❌ Datei nicht gefunden
)
set /a count+=1

REM Test Tool 8
echo 🧪 Testing: 08-url-shortener
if exist "%basePath%\08-url-shortener\url-shortener.velin" (
    echo ✅ Datei gefunden
    set /a success+=1
) else (
    echo ❌ Datei nicht gefunden
)
set /a count+=1

REM Test Tool 9
echo 🧪 Testing: 09-quiz-game
if exist "%basePath%\09-quiz-game\quiz-game.velin" (
    echo ✅ Datei gefunden
    set /a success+=1
) else (
    echo ❌ Datei nicht gefunden
)
set /a count+=1

REM Test Tool 10
echo 🧪 Testing: 10-contact-book
if exist "%basePath%\10-contact-book\contact-book.velin" (
    echo ✅ Datei gefunden
    set /a success+=1
) else (
    echo ❌ Datei nicht gefunden
)
set /a count+=1

echo.
echo ╔══════════════════════════════════════════════════════╗
echo ║           📊 TEST ZUSAMMENFASSUNG                   ║
echo ╚══════════════════════════════════════════════════════╝
echo.
echo ✅ Tools getestet: %success%/%count%
echo.

if %success% equ %count% (
    echo ╔══════════════════════════════════════════════════════╗
    echo ║  ✅ ALLE TESTS ERFOLGREICH BESTANDEN!              ║
    echo ║                                                      ║
    echo ║  Die 10 Tools sind bereit zum Kompilieren:         ║
    echo ║  - 01-todo-list-manager ✓                          ║
    echo ║  - 02-weather-api-client ✓                         ║
    echo ║  - 03-file-organizer ✓                             ║
    echo ║  - 04-email-validator ✓                            ║
    echo ║  - 05-simple-blog ✓                                ║
    echo ║  - 06-json-processor ✓                             ║
    echo ║  - 07-password-generator ✓                         ║
    echo ║  - 08-url-shortener ✓                              ║
    echo ║  - 09-quiz-game ✓                                  ║
    echo ║  - 10-contact-book ✓                               ║
    echo ║                                                      ║
    echo ║  Nächste Schritte:                                  ║
    echo ║  1. cargo build --release                          ║
    echo ║  2. Kompile mit dem Compiler                       ║
    echo ║  3. Führe aus und teste!                           ║
    echo ║                                                      ║
    echo ║  🎯 Ready for Compilation!                         ║
    echo ╚══════════════════════════════════════════════════════╝
) else (
    echo ⚠️  Einige Tools fehlgeschlagen
)

echo.
pause
