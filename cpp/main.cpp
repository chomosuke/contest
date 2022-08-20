#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

typedef long long ll;

int main() {
    ll n;
    cin >> n;
    ll total = -1;
    ll pp, p;
    cin >> pp >> p;
    bool longing = pp < p;
    total += abs(pp - p);
    pp = p;
    for (ll i = 0; i < n - 2; i++) {
        cin >> p;

        total += abs(pp - p);
        if (pp < p != longing) {
            total -= 1;
        }
        longing = pp < p;

        pp = p;
    }
    cout << total << endl;
}
