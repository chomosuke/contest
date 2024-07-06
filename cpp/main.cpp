#include <algorithm> // IWYU pragma: keep
#include <bitset>    // IWYU pragma: keep
#include <cassert>   // IWYU pragma: keep
#include <cmath>     // IWYU pragma: keep
#include <cstdint>   // IWYU pragma: keep
#include <fstream>   // IWYU pragma: keep
#include <iostream>  // IWYU pragma: keep
#include <map>       // IWYU pragma: keep
#include <numeric>   // IWYU pragma: keep
#include <optional>
#include <queue>         // IWYU pragma: keep
#include <set>           // IWYU pragma: keep
#include <string>        // IWYU pragma: keep
#include <string_view>   // IWYU pragma: keep
#include <tuple>         // IWYU pragma: keep
#include <unordered_map> // IWYU pragma: keep
#include <unordered_set> // IWYU pragma: keep
#include <vector>        // IWYU pragma: keep

using namespace std;

int main() {
    int n, m, k;
    cin >> n >> m >> k;
    vector<optional<int>> last_tap(m, nullopt);
    vector<int> charge(m, 0);
    for (int i{0}; i < k; i++) {
        int p, c;
        cin >> p >> c;
        c--;
        if (last_tap[c].has_value()) {
            if (p == last_tap[c].value()) {
                charge[c] += 100;
            } else {
                charge[c] += abs(p - last_tap[c].value());
            }
            last_tap[c] = nullopt;
        } else {
            last_tap[c] = make_optional(p);
        }
    }
    for (int i{0}; i < last_tap.size(); i++) {
        if (last_tap[i].has_value()) {
            charge[i] += 100;
        }
    }
    for (const auto& c : charge) {
        cout << c << " ";
    }
    cout << endl;
    return 0;
}
