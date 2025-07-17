#include <stddef.h>

typedef void (*Callback1)(int result, void* closure);

void add_two_numbers(int a, int b, Callback1 cb, void* closure)
{
    int result = a + b;
    cb(result, closure);
}

typedef int (*Callback2)(int a);
typedef int (*Callback3)(Callback2 cb2, int a);

int callback2(int result)
{
    return result + result;
}
int register1(Callback3 cb3, int a)
{
    if (cb3 == NULL) {
        return a;
    }
    if (a > 10) {
        return cb3(NULL, a);
    }
    return cb3(callback2, a);
}