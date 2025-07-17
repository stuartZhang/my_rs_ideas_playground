#include <stddef.h>
//
// �ص��հ�����Ӧ Rust �˵� bin/ffi-closure-callback.rs �� ffi-closure-callback-by-zst.rs �ļ�
//
typedef void (*Callback1)(int result, void* closure);

void add_two_numbers(int a, int b, Callback1 cb, void* closure)
{
    int result = a + b;
    cb(result, closure);
}
//
// ��ָ���Ż�����Ӧ Ruest �˵� bin/ffi-nullable-pointer-optimization.rs �ļ�
//
typedef int (*Callback2)(int a); // ��Ӧ Rust �˵� Option<Callback2> ���Ͷ���
typedef int (*Callback3)(Callback2 cb2, int a); // ��Ӧ Rust �˵� Option<Callback3> ���Ͷ���

static int callback2(int result)
{
    return result + result;
}
int register1(Callback3 cb3, int a)
{
    if (cb3 == NULL) { // �������� Rust �˵Ŀպ���ָ��
        return a;
    }
    if (a > 10) { // �� Rust �˻ش��պ���ָ��
        return cb3(NULL, a);
    }
    return cb3(callback2, a); // ������Ҳ���ش��պ���ָ��
}