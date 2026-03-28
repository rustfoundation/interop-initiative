#include <iostream>
#include <string>

extern "C" {
    void ask_name();
    char* process_name(const char* name);
    void free_string(char* ptr);
}

int main() {
    ask_name();

    std::string name;
    std::getline(std::cin, name);

    char* response = process_name(name.c_str());

    std::cout << "C++ received: " << response << std::endl;

    free_string(response);

    return 0;
}