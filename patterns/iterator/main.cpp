#include <iostream>
#include <vector>



template <typename T>
class Iterator {
    public: 
        virtual T getNext() = 0;
        virtual bool empty() const = 0; 
};



template <typename T>
class ContainerIterator: public Iterator<T> {
    public: 
        ContainerIterator(std::vector<T> *container) {
            this->container = container;
            this->counter = 0;
        }
        
        T getNext() override {
            return container->at(this->counter++);
        }
        bool empty() const override {
            return this->container->size() == this->counter;
        }

    private:
        int counter;
        std::vector<T> *container;
};





template <typename T>
class Container {
    public:
        Container(std::vector<T> vec) {
            this->vec = vec;
        }

        Iterator<T> *getIterator() {
            return new ContainerIterator<T>(&this->vec);
        }

    private:
        std::vector<T> vec;
};


template <typename T>
void print(Iterator<T> *iterator) {
    while(!iterator->empty()) {
        std::cout << iterator->getNext() << " "; 
    }
    std::cout << std::endl;
}




int main() {
    Container<int> *containerInt = new Container<int>({1, 2, 3, 4, 5});
    Container<char> *containerChar = new Container<char>({'a', 'b', 'c', 'd', 'f'});

    Iterator<int> *iteratorInt = containerInt->getIterator();

    Iterator<char> *iteratorChar = containerChar->getIterator();
    

    print(iteratorChar);
    print(iteratorInt);


    delete iteratorChar;
    delete iteratorInt;
    delete containerChar;
    delete containerInt;

    return 0;
}