template<typename T>
T max(T a, T b) {
    return a > b ? a : b;
}

int main() {
    int x = max<int>(5, 10);
    return x;
}
