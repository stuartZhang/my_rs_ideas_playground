typedef void (*Callback)(int result, void *closure);

void add_two_numbers(int a, int b, Callback cb, void *closure)
{
    int result = a + b;
    cb(result, closure);
}
