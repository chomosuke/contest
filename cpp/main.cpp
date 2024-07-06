#include <algorithm>     // IWYU pragma: keep
#include <bitset>        // IWYU pragma: keep
#include <cassert>       // IWYU pragma: keep
#include <cmath>         // IWYU pragma: keep
#include <cstdint>       // IWYU pragma: keep
#include <fstream>       // IWYU pragma: keep
#include <iostream>      // IWYU pragma: keep
#include <map>           // IWYU pragma: keep
#include <numeric>       // IWYU pragma: keep
#include <optional>      // IWYU pragma: keep
#include <queue>         // IWYU pragma: keep
#include <set>           // IWYU pragma: keep
#include <string>        // IWYU pragma: keep
#include <string_view>   // IWYU pragma: keep
#include <tuple>         // IWYU pragma: keep
#include <unordered_map> // IWYU pragma: keep
#include <unordered_set> // IWYU pragma: keep
#include <vector>        // IWYU pragma: keep

using namespace std;

using ll = long long;

int main() {
    int n;
    cin >> n;
    vector<ll> fs{};
    for (int i{0}; i < n; i++) {
        int f;
        cin >> f;
        fs.push_back(f);
    }
    sort(fs.begin(), fs.end());
    ll f_all{0};
    for (int i{n / 3}; i < n; i += 2) {
        f_all += fs[i];
    }
    cout << f_all << endl;
}
