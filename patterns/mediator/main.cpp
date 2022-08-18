#include <iostream>


enum TypeComponent {
    A,
    B,
};

enum Event {
    LeftClick, 
    RightClick,

};

class Mediator {
    public:
        virtual void notify(TypeComponent type, Event event) = 0;
};

class Component {
    private:
        Mediator *mediator;

    public:

        Component(Mediator *m): mediator(m) {}
        Mediator *getMediator() {
            return mediator;
        }
        void setMediator(Mediator *m) {
            this->mediator = m;
        }

    
};



class ComponentA: public Component {
    private: 
        bool available;

    public:
        ComponentA(Mediator *m = nullptr): Component(m), available(true) {}

        void rightClick() {
            if(this->available) {
                if(this->getMediator() != nullptr) {
                    this->getMediator()->notify(TypeComponent::A, Event::RightClick);
                }
            } else {
                std::cout << "ComponentA closed" << std::endl;
            }
        }

        void leftClick() {
            if(this->available) {
                if(this->getMediator() != nullptr) {
                    this->getMediator()->notify(TypeComponent::A, Event::LeftClick);
                }
            } else {
                std::cout << "ComponentA closed" << std::endl;
            }
        }

        void close() {
            this->available = false;
            std::cout << "close ComponentA" << std::endl;
        }

        void open() {
            this->available = true;
            std::cout << "open ComponentA" << std::endl;
        }
};


class ComponentB: public Component {
    private: 
        bool available;

    public:
        ComponentB(Mediator *m = nullptr): Component(m), available(true) {}

        void rightClick() {
            if(this->available) {
                if(this->getMediator() != nullptr) {
                    this->getMediator()->notify(TypeComponent::B, Event::RightClick);
                }
            } else {
                std::cout << "ComponentB closed" << std::endl;
            }
            
        }

        void leftClick() {
            if(this->available) {
                if(this->getMediator() != nullptr) {
                    this->getMediator()->notify(TypeComponent::B, Event::LeftClick);
                }
            } else {
                std::cout << "ComponentB closed" << std::endl;
            }
        }

        void close() {
            this->available = false;
            std::cout << "close ComponentB" << std::endl;
        }

        void open() {
            this->available = true;
            std::cout << "open ComponentB" << std::endl;
        }

};

class ConcreteMediator: public Mediator {
    private: 
        ComponentA *componentA;
        ComponentB *componentB;

    public:
        ConcreteMediator(ComponentA *a, ComponentB *b) {
            this->componentA = a;
            this->componentB = b;
            this->componentA->setMediator(this);
            this->componentB->setMediator(this);

        }


        void notify(TypeComponent type, Event event) override {
            if(type == TypeComponent::A) {
                if(event == Event::LeftClick) {
                    this->componentB->close();
                } else if(event == Event::RightClick) {
                    this->componentB->open();
                }


            } else if (type == TypeComponent::B) {
                if(event == Event::LeftClick){
                    this->componentA->close();
                } else if(event == Event::RightClick) {
                    this->componentA->open();
                }
            }
        }



};






int main() {
    ComponentA *a = new ComponentA();
    ComponentB *b = new ComponentB();



    Mediator *mediator = new ConcreteMediator(a, b);

    a->leftClick();
    b->rightClick();
    a->rightClick();
    b->leftClick();
    a->rightClick();
    b->rightClick();
    a->rightClick();



    delete mediator;
    delete a;
    delete b;

}