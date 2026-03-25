// this guard is needed bc extern "C" is a c++ thing
// if a plain c compiler sees it, it'll throw a syntax error
// so we only wrap with extern "C" when compiling as c++
#ifdef __cplusplus
extern "C"{
#endif

// our wrapper function declarations so rust (and c) can see them
void greet_number(int x);
void greet_name(const char *name);

#ifdef __cplusplus
}
#endif
