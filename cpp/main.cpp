#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

typedef long long ll;

ll key(ll i, char c) {
    return i ^ (((ll)c) << 48);
}

int main() {
    int x;
    cin >> x >> x;
    string s, t;
    cin >> s >> t;

    // at i, the next char c is at
    unordered_map<ll, ll> index;

    for (ll i = s.size() - 1; i >= 0; i--) {
        index[key(i, s[i])] = i;
        for (ll j = i - 1; j >= 0 && s[j] != s[i]; j--) {
            index[key(j, s[i])] = i;
        }
    }

    ll i = 0;
    ll n = 0;
    while (i < t.size()) {
        n++;
        ll prevI = i;
        for (ll j = 0; j < s.size() && i < t.size() && 0 < index.count(key(j, t[i])); j++) {
            j = index[key(j, t[i])];
            i++;
        }
        if (i == prevI) {
            n = -1;
            break;
        }
    }
    cout << n << endl;
}
