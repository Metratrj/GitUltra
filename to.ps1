# Alle Ordner bis 3 Ebenen tief erfassen
$directories = Get-ChildItem -Path . -Directory -Recurse -Depth 3

foreach ($dir in $directories) {
    # Prüfen, ob der Ordner leer ist (inkl. versteckter/system Dateien)
    $hasItems = Get-ChildItem -Path $dir.FullName -Force -ErrorAction SilentlyContinue | Select-Object -First 1

    if (-not $hasItems) {
        # Dateipfad erstellen
        $filePath = Join-Path $dir.FullName "ttt.dummy"

        # Dummy-Datei mit Inhalt erstellen
        Set-Content -Path $filePath -Value "lorem ipsum"
        Write-Host "Dummy-Datei erstellt in: $($dir.FullName)"
    }
}

Write-Host "Abgeschlossen!"
