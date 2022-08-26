#include <iostream>
#include <string>
#include <list>


class Component {
    public:
        ~Component() {}
        virtual void execute() = 0;
};


class Composite: public Component {
    private:
        std::list<Component *> components;

    public:
        void add(Component *cmp) {
            this->components.push_back(cmp);
        }

        void execute() override {
            if(!this->components.empty()) {
                auto iter = this->components.begin();
                while(iter!=this->components.end()) {
                    (*iter)->execute();
                    ++iter;
                }
            }
        }

        void remove(Component* cmp) {
            this->components.remove(cmp);
        }
    
};

class Leaf: public Component {
    private:
        std::string str;

    public: 
        Leaf(std::string str): str(str) {}


        void execute() override {
            std::cout << str << std::endl;
        }
};









int main() {

    Composite *composite = new Composite();

    Composite *composite2 = new Composite();

    Component *leaf1 = new Leaf("Leaf1");
    Component *leaf2 = new Leaf("Leaf2");
    Component *leaf3 = new Leaf("Leaf3");

    composite->add(composite2);
    composite->add(leaf1);
    composite2->add(leaf2);
    composite2->add(leaf3);

    composite->execute();
    std::cout << std::endl;
    composite2->execute();
    std::cout << std::endl;
    composite2->remove(leaf3);
    composite->execute();




    delete leaf1;
    delete leaf2;
    delete leaf3;
    delete composite2;
    delete composite;

    return 0;
}