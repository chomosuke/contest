#include <algorithm>     // IWYU pragma: keep
#include <bitset>        // IWYU pragma: keep
#include <cassert>       // IWYU pragma: keep
#include <cmath>         // IWYU pragma: keep
#include <cstdint>       // IWYU pragma: keep
#include <fstream>       // IWYU pragma: keep
#include <iomanip>       // IWYU pragma: keep
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
    ll n, m, k;
    cin >> n >> m >> k;
    if (k + m >= n) {
        cout << 100 << endl;
        return 0;
    }
    vector<ll> ss;
    for (int i = 0; i < n; i++) {
        ll s;
        cin >> s;
        ss.push_back(s);
    }
    sort(ss.rbegin(), ss.rend());
    ll sum = 0;
    for (int i = 0; i < n; i++) {
        sum += ss[i];
    }
    ll downloaded = 0;
    for (int i = 0; i < k + m; i++) {
        downloaded += ss[i];
    }
    cout << setprecision(10);
    cout << static_cast<double>(100) / sum * downloaded << endl;
    return 0;
}
