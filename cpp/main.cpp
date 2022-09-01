#include <algorithm>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

typedef intmax_t Int;
typedef long double Float;

void print_vector(const vector<Int> &v) {
    if (v.size() == 0) {
        return;
    }
    for (auto x : v) {
        cout << x << " ";
    }
    cout << endl;
}

Int find(const vector<Int> &v, Int target) {
    Int i = 0;
    for (Int j = v.size() / 2; j >= 1; j /= 2) {
        while (i + j < v.size() && v[i + j] <= target) {
            i += j;
        }
    }
    if (v[i] == target) {
        return i;
    } else {
        return -1;
    }
}

int main() {
    vector<Int> v = {4, 2, 5, 3, 5, 8, 3};
    sort(v.begin(), v.end());
    print_vector(v);
    sort(v.rbegin(), v.rend());
    print_vector(v);

    // default sort comparitor is [](Int a, Int b) { return a < b; }
    sort(v.begin(), v.end(), [](Int a, Int b) { return a < b; });
    print_vector(v);

    while (true) {
        Int target;
        cin >> target;
        cout << find(v, target) << endl;
    }

    // different way of creating a vector
    // this is calling a constructor
    // size 10, init 0
    vector<Int> v2(10);
    // size 10, init 5
    vector<Int> v3(10, 5);

    // There's also string in cpp
    string a = "hatti";
    // you can concat string like:
    string b = a + a; // hattihatti
    // you can get substr
    string c = b.substr(3, 4); // tiva
    // start at index 3 with len 4
}
