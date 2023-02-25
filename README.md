# Insync parser

Parser of insync exported data for a yearly expenses. 
Just to have clear analysis of my expenses.
Project for myself, helping me in learning and understanding rust ownership

Currently in order for this script to work, follow these steps:
1. Place insync data inside examples folder in root of the project
2. Name data file as `year.pdf`
3. Run `cargo run`
4. Check the output in `output.txt`

Todo: 
* refactoring
* file names from cli
* possibly multithreading (currently it overall runs in about 0.5seconds)
* add tests (why not?)
