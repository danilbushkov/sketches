#include <iostream>

class IConnector {
    public:
        virtual ~IConnector() {}
        virtual void connect() = 0;
};


class ConnectorA {
    public:
        void concreteConnect() const {
            std::cout << "ConnectorA connect" << std::endl;
        }
};


class Adapter: public IConnector {
    private:
        ConnectorA* connector;


    public:
        Adapter(ConnectorA* connector): connector(connector) {}
    
        void connect() override {
            this->connector->concreteConnect();
        }
};





int main() {
    
    ConnectorA *connectorA = new ConnectorA();

    IConnector *connector = new Adapter(connectorA);

    connector->connect();


    delete connector;
    delete connectorA;
    
    return 0;
}