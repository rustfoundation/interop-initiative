- Problem Name: string-interop-interact
- Start Date: 28-02-2026
- Problem Statement PR:

## Summary
[summary]: #summary

This example demonstrates safe and interactive string interoperability between Rust and C++. 

C++ and Rust use different string representations (`std::string` and `String`), which cannot be shared directly across the FFI boundary. To solve this, the example uses C-style strings (`const char*`) as a common interface.

The example showcases a multi-step interaction:
- Rust initiates a prompt to the user
- C++ handles user input
- The input is passed to Rust for processing
- Rust returns a new string back to C++
- C++ prints the result and ensures proper memory deallocation

This highlights key concepts such as ownership transfer, memory safety, and bidirectional communication between Rust and C++.

### Example Code
[example-code]: #example-code

This example highlights ownership transfer across the FFI boundary using three key concepts: **Move**, **Forget**, and **Own**.

---

### 🔹 Move (Rust → C++)

```rust
CString::new(response).unwrap().into_raw();
```
- Converts a Rust String into a C-compatible string
- into_raw() transfers ownership to C++
- Rust no longer manages this memory

### 🔹 Forget (Rust does not free automatically)

- Ownership has been transferred, Rust will not drop this memory

### 🔹 Own + Free (C++ -> Rust)

```cpp
char* response = process_name(name.c_str());
free_string(response);
```
```rust
let _ = CString::from_raw(ptr);
```

- C++ receives ownership of the string
- C++ must explicitly call free_string
- Rust reconstructs and safely frees the memory

### What might go wrong?

- If free_string is not called → memory leak
- If freed twice → double free crash
- If freed by C++ directly (e.g., delete/free) → undefined behavior

## How to Build and Run

### Requirements

- Rust (with `cargo`)
- A C++ compiler (`g++` or `clang++`)

---

### Build and Run

From the example directory:

```bash
./run.sh

## Example Output

```text
Rust: What is your name?
shashank
C++ received: Welcome, shashank!
```

### How it Work (Flow)

This example demonstrates a multi-step interaction between C++, Rust, and the user

- Step-by-step flow

1. **C++ calls Rust to initiate interaction**

   ```cpp
   ask_name();
    ```

   - Rust print: What is your name?

2. **User provides input (handled by C++)**

    ```cpp
    std::getline(std::cin, name);
    ```
    - C++ reads user input into a std::string

3. **C++ sends the string to Rust**

    ```cpp
    process_name(name.c_str());
    ```
    - std::string is converted to const char*
    - This crosses the FFI boundary

4. **Rust processes the input**

    ```rust
    let response = format!("Welcome, {}!", name_str);
    ```

    - Rust safely converts the input using CStr
    - Generates a new response string

5. **Rust returns a new string to C++**

    ```rust
    CString::new(response).unwrap().into_raw()
    ```
    - Ownership is transferred to C++
    - Rust no longer manages this memory

6. **C++ receives and prints the result

    ```cpp
    std::cout << response << std::endl;
    ```

7. **C++ frees memory using Rust**

    ```cpp
    free_string(response);
    ```

    - Rust reconstructs the string and safely frees it

**Summary**

C++ → Rust (prompt)
User → C++ (input)
C++ → Rust (process)
Rust → C++ (response)
C++ → Rust (free memory)

