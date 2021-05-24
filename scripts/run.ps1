$file = 'venv/Scripts/Activate.ps1'
if (-not(Test-Path -Path $file -PathType Leaf)) {
    $file
}
python -i src\__main__.py