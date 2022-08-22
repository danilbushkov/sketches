#include <iostream>

class ConcreteComponent1;
class ConcreteComponent2;

class Visitor {
    public:
        virtual ~Visitor() {}
        virtual void visitComponent1(const ConcreteComponent1 *component) const = 0;
        virtual void visitComponent2(const ConcreteComponent2 *component) const = 0;
};


class Component {
    public:
        virtual ~Component() {}
        virtual void accept(const Visitor *v) = 0;
};

class ConcreteComponent1: public Component {
    public:
        void accept(const Visitor *v) override {
            v->visitComponent1(this);
        }

        void print() const {
            std::cout << "Component1" << std::endl;
        }
};

class ConcreteComponent2: public Component {
    public:
        void accept(const Visitor *v) override {
            v->visitComponent2(this);
        }

        void print() const {
            std::cout << "Component2" << std::endl;
        }
};

class ConcreteVisitor: public Visitor {
    public: 
        virtual void visitComponent1(const ConcreteComponent1 *component) const {
            std::cout << "Visitor ";
            component->print();
        };
        virtual void visitComponent2(const ConcreteComponent2 *component) const {
            std::cout << "Visitor ";
            component->print();
        };
};


void print(Component *components[2], Visitor* visitor) {
    for(size_t i = 0; i < 2; i++) {
        components[i]->accept(visitor);
    }

}


int main() {
    Component *components[2] = { new ConcreteComponent1(), new ConcreteComponent2() };
    Visitor *visitor = new ConcreteVisitor();


    print(components, visitor);


    delete visitor;
    for(size_t i = 0; i < 2; i++) {
        delete components[i];
    }

    return 0;
}