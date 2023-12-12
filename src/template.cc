template <typename T>
concept C1 = sizeof(T) != sizeof(int);

template <C1 T> struct S1 {};

template <C1 T> using Ptr = T *;
// template <typename T> struct S2 {
//     Ptr<int> x;
// };
// template <typename T> struct S3 {
//     Ptr<T> x;
// };
// S3<int> x;
// template <template <C1, T class X> struct S4 {
//     X<int> x;
// };

// template <typename T>
// concept C2 = sizeof(T) == 1;
// template <C2 T> struct S {};
// template struct S<char[2]> template <> struct S<char[2]> {};

int main() {
    // won't compile due to type constraint
    // S1<int>* p;
    // Ptr<int> p;
    return 0;
}