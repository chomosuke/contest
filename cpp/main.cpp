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
    ll n, m;
    cin >> n >> m;
    vector<string> names;
    string n_;
    getline(cin, n_);
    for (int i = 0; i < n; i++) {
        string n;
        getline(cin, n);
        names.push_back(n);
        assert(n.size() == m);
    }
    stringstream ss;
    for (int i = 0; i < m; i++) {
        map<char, ll> chars;
        for (int j = 0; j < n; j++) {
            if (!chars.count(names[j][i])) {
                chars[names[j][i]] = 0;
            }
            chars[names[j][i]] += 1;
        }
        char choosen = 0;
        char choosen_vote = 0;
        for (auto c : chars) {
            assert(c.first >= 'a' && c.first <= 'z');
            if (c.second > choosen_vote || (c.second == choosen_vote && c.first < choosen)) {
                choosen = c.first;
                choosen_vote = c.second;
            }
        }
        ss << choosen;
    }
    cout << ss.str() << endl;
}
