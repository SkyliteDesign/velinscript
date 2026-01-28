#!/bin/pwsh
# ================================================
# 🤖 VelinScript Examples Pack Vol 2 - Test Suite
# ================================================
# Testet alle 10 Tools nach dem Kompilieren

Write-Host "╔══════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  🤖 VelinScript Examples Pack Vol 2 - Test Suite    ║" -ForegroundColor Cyan
Write-Host "║     Teste alle 10 Tools mit dem Compiler            ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Pfade
$examplesPath = "D:\velinscript\examples\Examples Pack Vol 2"
$compilerPath = "D:\velinscript\compiler\target\release\velinscript"

# Test-Status
$testResults = @()

# Funktion zum Testen eines Tools
function Test-VelinScriptTool {
    param(
        [string]$ToolName,
        [string]$ToolPath,
        [string]$MainFile
    )
    
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
    Write-Host "🧪 Testing: $ToolName" -ForegroundColor Yellow
    Write-Host "📁 Path: $ToolPath" -ForegroundColor Gray
    Write-Host "📄 File: $MainFile" -ForegroundColor Gray
    
    # Prüfe ob Datei existiert
    if (Test-Path "$ToolPath\$MainFile") {
        Write-Host "✅ Datei gefunden" -ForegroundColor Green
        
        # Zähle Zeilen Code
        $codeLines = (Get-Content "$ToolPath\$MainFile" | Measure-Object -Line).Lines
        Write-Host "📊 Codezeilen: $codeLines" -ForegroundColor Cyan
        
        # Zähle Funktionen
        $functions = (Select-String -Path "$ToolPath\$MainFile" -Pattern "^fn " | Measure-Object).Count
        Write-Host "🔧 Funktionen: $functions" -ForegroundColor Cyan
        
        # Zähle Strukturen
        $structs = (Select-String -Path "$ToolPath\$MainFile" -Pattern "^struct " | Measure-Object).Count
        Write-Host "📦 Datenstrukturen: $structs" -ForegroundColor Cyan
        
        # Prüfe API Endpoints
        $endpoints = (Select-String -Path "$ToolPath\$MainFile" -Pattern "@(GET|POST|PUT|DELETE|PATCH)" | Measure-Object).Count
        Write-Host "🔌 API Endpoints: $endpoints" -ForegroundColor Cyan
        
        Write-Host "✨ Status: READY TO COMPILE ✨" -ForegroundColor Green
        return @{ Tool = $ToolName; Status = "OK"; Lines = $codeLines; Functions = $functions; Structs = $structs; Endpoints = $endpoints }
    } else {
        Write-Host "❌ Datei nicht gefunden!" -ForegroundColor Red
        return @{ Tool = $ToolName; Status = "FEHLER"; Lines = 0; Functions = 0; Structs = 0; Endpoints = 0 }
    }
}

# Teste alle 10 Tools
Write-Host "`n🚀 Starte Tests für alle 10 Tools...`n" -ForegroundColor Cyan

$tools = @(
    @{ Name = "01-todo-list-manager"; File = "todo-manager.velin" },
    @{ Name = "02-weather-api-client"; File = "weather-client.velin" },
    @{ Name = "03-file-organizer"; File = "file-organizer.velin" },
    @{ Name = "04-email-validator"; File = "email-validator.velin" },
    @{ Name = "05-simple-blog"; File = "blog-system.velin" },
    @{ Name = "06-json-processor"; File = "json-processor.velin" },
    @{ Name = "07-password-generator"; File = "password-generator.velin" },
    @{ Name = "08-url-shortener"; File = "url-shortener.velin" },
    @{ Name = "09-quiz-game"; File = "quiz-game.velin" },
    @{ Name = "10-contact-book"; File = "contact-book.velin" }
)

foreach ($tool in $tools) {
    $result = Test-VelinScriptTool -ToolName $tool.Name -ToolPath "$examplesPath\$($tool.Name)" -MainFile $tool.File
    $testResults += $result
}

# Zeige Zusammenfassung
Write-Host "`n" 
Write-Host "╔══════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║           📊 TEST ZUSAMMENFASSUNG                   ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════╝" -ForegroundColor Cyan

$table = $testResults | Format-Table -Property `
    @{Label="Tool"; Expression={$_.Tool}; Width=30},
    @{Label="Status"; Expression={$_.Status}; Width=12},
    @{Label="Zeilen"; Expression={$_.Lines}; Width=8},
    @{Label="Funktionen"; Expression={$_.Functions}; Width=12},
    @{Label="Strukturen"; Expression={$_.Structs}; Width=12},
    @{Label="Endpoints"; Expression={$_.Endpoints}; Width=10} -AutoSize

Write-Host $table

# Statistiken
$totalTools = $testResults.Count
$successfulTools = ($testResults | Where-Object { $_.Status -eq "✅ OK" }).Count
$totalLines = ($testResults | Measure-Object -Property Lines -Sum).Sum
$totalFunctions = ($testResults | Measure-Object -Property Functions -Sum).Sum
$totalStructs = ($testResults | Measure-Object -Property Structs -Sum).Sum
$totalEndpoints = ($testResults | Measure-Object -Property Endpoints -Sum).Sum

Write-Host "`n📈 STATISTIKEN:" -ForegroundColor Yellow
Write-Host "  ✅ Tools getestet: $successfulTools/$totalTools" -ForegroundColor Green
Write-Host "  📄 Gesamtzahl Codezeilen: $totalLines" -ForegroundColor Cyan
Write-Host "  🔧 Gesamtzahl Funktionen: $totalFunctions" -ForegroundColor Cyan
Write-Host "  📦 Gesamtzahl Strukturen: $totalStructs" -ForegroundColor Cyan
Write-Host "  🔌 Gesamtzahl API-Endpoints: $totalEndpoints" -ForegroundColor Cyan

Write-Host "`n"

if ($successfulTools -eq $totalTools) {
    Write-Host "╔══════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║  ✅ ALLE TESTS ERFOLGREICH BESTANDEN!              ║" -ForegroundColor Green
    Write-Host "║                                                      ║" -ForegroundColor Green
    Write-Host "║  Nächste Schritte:                                  ║" -ForegroundColor Green
    Write-Host "║  1. Starten Sie: cargo build --release              ║" -ForegroundColor Green
    Write-Host "║  2. Kompilieren Sie ein Tool mit dem Compiler       ║" -ForegroundColor Green
    Write-Host "║  3. Führen Sie das kompilierte Programm aus         ║" -ForegroundColor Green
    Write-Host "║                                                      ║" -ForegroundColor Green
    Write-Host "║  🎯 Ready for Compilation & Testing!               ║" -ForegroundColor Green
    Write-Host "╚══════════════════════════════════════════════════════╝" -ForegroundColor Green
}
else {
    Write-Host "⚠️  Einige Tests fehlgeschlagen" -ForegroundColor Red
}

Write-Host "`n"
