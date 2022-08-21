
#include <iostream>


class Context {
    public:
        void template_method() const {
            this->step1();
            this->step2();
        }

        virtual void step1() const {}

        virtual void step2() const {}
};

class ConcreteContext1: public Context {
    public:
        void step1() const override {
            std::cout << "1: step1" << std::endl;
        } 
        void step2() const override {
            std::cout << "1: step2" << std::endl;
        } 
};

class ConcreteContext2: public Context {
    public:
        void step1() const override {
            std::cout << "2: step1" << std::endl;
        } 
        void step2() const override {
            std::cout << "2: step2" << std::endl;
        } 
};


void client_method(Context *context) {
    context->template_method();
}



int main() {
    Context *context1 = new ConcreteContext1();
    Context *context2 = new ConcreteContext2();

    
    client_method(context1);
    client_method(context2);
    

    delete context1;
    delete context2;

    return 0;
}