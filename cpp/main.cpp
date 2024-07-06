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
    vector<int> ts;
    for (int i{0}; i < n; i++) {
        int t;
        cin >> t;
        ts.push_back(t);
    }
    vector<vector<int>> count_before{{0}, {0}, {0}};
    for (int i{1}; i <= n; i++) {
        for (int j{0}; j < 3; j++) {
            count_before[j].push_back(count_before[j][i - 1]);
        }
        count_before[ts[i - 1]][i]++;
    }

    int max_total{0};

    vector<int> perm{0, 1, 2};
    for (int p{0}; p < 6; p++) {
        int p1{perm[0]};
        int p2{perm[1]};
        int p3{perm[2]};

        int p1_p2{0};

        auto total{[&](int p1_p2, int p2_p3) {
            return count_before[p1][p1_p2] + count_before[p2][p2_p3] -
                   count_before[p2][p1_p2] + count_before[p3][n] -
                   count_before[p3][p2_p3];
        }};

        for (int p2_p3{0}; p2_p3 <= n; p2_p3++) {
            int t1{total(p1_p2, p2_p3)};
            int t2{total(p2_p3, p2_p3)};
            if (t2 > t1) {
                p1_p2 = p2_p3;
            }
            max_total = max(max_total, t1);
            max_total = max(max_total, t2);
        }

        next_permutation(perm.begin(), perm.end());
    }

    cout << max_total << endl;
}
