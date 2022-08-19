#include <iostream>
#include <string>
#include <vector>

class Memento {
    public:
        virtual std::string getState() const = 0;
    
};



class ContextMemento: public Memento {
    private:
        std::string state_;

    public:

        ContextMemento(std::string s): state_(s) {}

        std::string getState() const override {
            return this->state_;
        }
};



class Context {
    private: 
        std::string state_;

    public: 

        Context(std::string state): state_(state) {}

        Memento *save() {
            return new ContextMemento(state_);
        }

        void restore(Memento* m) {
            this->state_ = m->getState();
        }

        void setState(std::string state) {
            this->state_ = state;
        }

        void printState() {
            std::cout << this->state_ << std::endl;
        }
};


class Client {
    private:
        Context *context; 
        std::vector<Memento *> history;

    public:
        Client(Context *context): context(context) {}
        ~Client() {
            for(int i = this->history.size()-1; i >= 0; i--) {

                Memento* m = this->history.back();
                this->history.pop_back();
                delete m;
                
            }
        }
        
        void changeState(std::string state) {
            this->history.push_back(context->save());
            this->context->setState(state);
        }
        
        void undo() {
            if(!this->history.empty()) {
                Memento *m = this->history.back();
                this->history.pop_back();
                this->context->restore(m);
                delete m;
            } else {
                std::cout << "history is empty" << std::endl;
            }
        }

        void print() const {
            this->context->printState();
        }

        void printHistory() const {


            for(Memento *m: this->history) {
                std::cout << m->getState() <<std::endl;
            }
        }
};






int main() {
    Context *context = new Context("1");
    Client *client = new Client(context);

    client->changeState("2");
    client->changeState("3");
    client->changeState("10");

    std::cout << "History:" << std::endl;
    client->printHistory();

    std::cout << std::endl;
    client->print();
    client->undo();
    client->print();
    client->undo();
    client->undo();
    client->print();
    client->undo();
    client->print();





    delete client;
    delete context;

    return 0;
}