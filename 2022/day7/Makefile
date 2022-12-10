compiler = g++
flags = -std=c++17
input = input

test2: part2 $(input)
	./part2 < $(input)

test1: part1 $(input)
	./part1 < $(input)

part1: part1.cpp
	$(compiler) $(flags) -o part1 part1.cpp

part2: part2.cpp
	$(compiler) $(flags) -o part2 part2.cpp

