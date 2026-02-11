@echo off
REM Create a test directory with dummy files to practice

echo Creating test directory with sample files...

mkdir test_folder 2>nul
cd test_folder

REM Create dummy files of different types
type nul > photo1.jpg
type nul > photo2.png
type nul > document1.pdf
type nul > document2.docx
type nul > report.txt
type nul > song1.mp3
type nul > song2.wav
type nul > video.mp4
type nul > movie.avi
type nul > data.csv
type nul > spreadsheet.xlsx
type nul > script.py
type nul > code.rs
type nul > program.java
type nul > archive.zip
type nul > backup.tar.gz
type nul > random.xyz
type nul > unknown.abc

echo.
echo Test folder created with sample files!
echo.
echo Files created:
dir /b
echo.
echo Now you can run the file organizer on this test folder:
echo   cd ..
echo   cargo run
echo   (then enter: test_folder)
echo.
pause
