all: run


run: ./app
	./app

check: ./app
	  valgrind --leak-check=yes ./app


./app: main.cpp
	g++ main.cpp -o app