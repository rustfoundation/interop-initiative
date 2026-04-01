#include <iostream>
#include <cstdint>  // for int64_t

// Define the same struct as in Rust
struct LogMessage {
    const char* msg;
    int64_t time;
    bool check;
};

// Declare Rust function
extern "C" void log_struct(const LogMessage* log);

int main() {
    // Create struct instances
    LogMessage log1 = {"System started", 1001, true};
    LogMessage log2 = {"Low memory warning", 1002, false};
    LogMessage log3 = {"Critical failure!", 1003, true};

    // Pass struct to Rust
    log_struct(&log1);
    log_struct(&log2);
    log_struct(&log3);

    return 0;
}
