#include <algorithm>     // IWYU pragma: keep
#include <bitset>        // IWYU pragma: keep
#include <cassert>       // IWYU pragma: keep
#include <cmath>         // IWYU pragma: keep
#include <cstdint>       // IWYU pragma: keep
#include <fstream>       // IWYU pragma: keep
#include <iostream>      // IWYU pragma: keep
#include <map>           // IWYU pragma: keep
#include <numeric>       // IWYU pragma: keep
#include <queue>         // IWYU pragma: keep
#include <set>           // IWYU pragma: keep
#include <string>        // IWYU pragma: keep
#include <string_view>   // IWYU pragma: keep
#include <tuple>         // IWYU pragma: keep
#include <unordered_map> // IWYU pragma: keep
#include <unordered_set> // IWYU pragma: keep
#include <vector>        // IWYU pragma: keep

using namespace std;
typedef long long ll;
const ll mod = 1e9 + 7;

ll powmod(ll a, ll b, ll mod) {
    ll curr = a, ans = 1;
    for (; b > 0; b >>= 1) {
        if (b & 1) {
            ans = ans * curr % mod;
        }
        curr = curr * curr % mod;
    }
    return ans;
}

void solve() {
    ll n;
    cin >> n;
    vector<ll> aa(n);
    for (ll i = 0; i < n; ++i)
        cin >> aa[i];
    vector<ll> pows(32);
    for (ll i = 0; i < n; ++i) {
        ll k = 0, x = aa[i];
        while (x % 2 == 0) {
            ++k;
            x >>= 1;
        }
        ++pows[k];
    }

    ll tot = 0;
    for (ll i = 30; ~i; --i) {
        ll rest = powmod(2, pows[i + 1], mod);
        if (i == 0) {
            tot = (tot + (powmod(2, pows[i], mod) - 1 + mod) % mod * rest % mod) % mod;
        } else if (pows[i] > 1) {
            tot =
                (tot + (powmod(2, pows[i], mod) * powmod(2, mod - 2, mod) % mod - 1) %
                mod * rest % mod) % mod;
        }
        pows[i] += pows[i + 1];
    }
    cout << tot << endl;
}

signed main() {
    ll t;
    // cin >> t;
    // while (t--)
        solve();
    return 0;
}
