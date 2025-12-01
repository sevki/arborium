@echo off
setlocal enabledelayedexpansion

set COUNT=0
set NAME=World

echo Hello, %NAME%!

for %%f in (*.txt) do (
    set /a COUNT+=1
    echo Found: %%f
)

if %COUNT% gtr 0 (
    echo Total files: %COUNT%
) else (
    echo No .txt files found
)
