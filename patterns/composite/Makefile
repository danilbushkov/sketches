all: run


run: app
	valgrind --leak-check=yes  ./app

app: main.cpp
	g++ main.cpp -o app