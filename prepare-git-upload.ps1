# VelinScript 2.0 - Git Upload Vorbereitung für Donnerstag
# Dieses Script bereitet das Repository für den Upload vor

Write-Host "🚀 VelinScript 2.0 - Git Upload Vorbereitung" -ForegroundColor Cyan
Write-Host "==============================================" -ForegroundColor Cyan
Write-Host ""

# Schritt 1: Test-Dateien entfernen
Write-Host "📋 Schritt 1: Entferne temporäre Test-Dateien..." -ForegroundColor Yellow
$testFiles = Get-ChildItem -Path . -Filter "test_*.velin" -ErrorAction SilentlyContinue
if ($testFiles) {
    $testFiles | ForEach-Object {
        Write-Host "  ❌ Lösche: $($_.Name)" -ForegroundColor Red
        Remove-Item $_.FullName -Force
    }
    Write-Host "  ✅ Test-Dateien entfernt" -ForegroundColor Green
} else {
    Write-Host "  ✅ Keine Test-Dateien gefunden" -ForegroundColor Green
}

# Schritt 2: Git Status prüfen
Write-Host ""
Write-Host "📋 Schritt 2: Prüfe Git Status..." -ForegroundColor Yellow
git status --short | Select-Object -First 20
Write-Host ""

# Schritt 3: Wichtige Dateien prüfen
Write-Host "📋 Schritt 3: Prüfe wichtige Dateien..." -ForegroundColor Yellow

$importantFiles = @(
    "README.md",
    "CHANGELOG.md",
    "compiler/src/stdlib/collections.rs",
    "compiler/src/stdlib/http_client.rs",
    "compiler/src/stdlib/rate_limit.rs",
    "docs/guides/tutorial-pattern-matching.md",
    "docs/guides/tutorial-closures.md",
    "docs/guides/tutorial-collections.md",
    "docs/guides/tutorial-http-client.md"
)

$allPresent = $true
foreach ($file in $importantFiles) {
    if (Test-Path $file) {
        Write-Host "  ✅ $file" -ForegroundColor Green
    } else {
        Write-Host "  ❌ $file fehlt!" -ForegroundColor Red
        $allPresent = $false
    }
}

if (-not $allPresent) {
    Write-Host ""
    Write-Host "⚠️  WARNUNG: Einige wichtige Dateien fehlen!" -ForegroundColor Yellow
}

# Schritt 4: Tests ausführen (optional)
Write-Host ""
$runTests = Read-Host "📋 Schritt 4: Tests ausführen? (j/n)"
if ($runTests -eq "j" -or $runTests -eq "J") {
    Write-Host "  🧪 Führe Tests aus..." -ForegroundColor Yellow
    Set-Location compiler
    cargo test 2>&1 | Select-String "test result" | Select-Object -Last 1
    Set-Location ..
    Write-Host "  ✅ Tests abgeschlossen" -ForegroundColor Green
}

# Schritt 5: Build prüfen (optional)
Write-Host ""
$runBuild = Read-Host "📋 Schritt 5: Build prüfen? (j/n)"
if ($runBuild -eq "j" -or $runBuild -eq "J") {
    Write-Host "  🔨 Baue Compiler..." -ForegroundColor Yellow
    Set-Location compiler
    cargo build --release 2>&1 | Select-String -Pattern "Finished|error" | Select-Object -Last 5
    Set-Location ..
    Write-Host "  ✅ Build abgeschlossen" -ForegroundColor Green
}

# Schritt 6: Zusammenfassung
Write-Host ""
Write-Host "==============================================" -ForegroundColor Cyan
Write-Host "✅ Vorbereitung abgeschlossen!" -ForegroundColor Green
Write-Host ""
Write-Host "📝 Nächste Schritte:" -ForegroundColor Yellow
Write-Host "  1. git add -A" -ForegroundColor White
Write-Host "  2. git status (prüfen was gestaged wurde)" -ForegroundColor White
Write-Host "  3. git commit -m 'feat: VelinScript 2.0 Release'" -ForegroundColor White
Write-Host "  4. git push origin main" -ForegroundColor White
Write-Host ""
Write-Host "📖 Siehe GIT_UPLOAD_CHECKLIST.md für Details" -ForegroundColor Cyan
Write-Host ""
