#include <iostream>

extern "C" void log_message(const char* msg);
extern "C" void log_warning(const char* msg);
extern "C" void log_error(const char* msg);

int main() {
    log_message("System started");
    log_warning("Low memory warning");
    log_error("Critical failure!");

    return 0;
}
