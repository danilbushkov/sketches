#include <iostream>




class IImplementation {
    public:
        virtual ~IImplementation() {}
        virtual void action() = 0;

    
};

class ConcreteImplementation: public IImplementation {
    public:
        void action() override {
            std::cout << "Concrete action" << std::endl;
        }
};


class Abstraction {
    private:
        IImplementation *implementation;
    
    public:
        Abstraction(IImplementation *impl): implementation(impl) {}
        void action() {
            implementation->action();
        }
};


void clientCode(Abstraction *impl) {
    impl->action();
}


int main() {
    IImplementation *impl = new ConcreteImplementation();
    Abstraction *abstraction = new Abstraction(impl);

    clientCode(abstraction);

    delete abstraction;
    delete impl;

    return 0;
}