#!/bin/bash

# Create a test directory with dummy files to practice

echo "Creating test directory with sample files..."

mkdir -p test_folder
cd test_folder

# Create dummy files of different types
touch photo1.jpg photo2.png
touch document1.pdf document2.docx report.txt
touch song1.mp3 song2.wav
touch video.mp4 movie.avi
touch data.csv spreadsheet.xlsx
touch script.py code.rs program.java
touch archive.zip backup.tar.gz
touch random.xyz unknown.abc

echo "✓ Test folder created with sample files!"
echo ""
echo "Files created:"
ls -1
echo ""
echo "Now you can run the file organizer on this test folder:"
echo "  cd .."
echo "  cargo run"
echo "  (then enter: test_folder)"
