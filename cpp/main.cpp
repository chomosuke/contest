#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

typedef long long ll;

int main() {
    ll c, u;
    cin >> c >> u;
    vector<ll> sizes;
    for (int i = 0; i < c; i++) {
        ll ci;
        cin >> ci;
        sizes.push_back(ci);
    }

    ll sum = 0, biggest = 0;
    for (auto& n : sizes) {
        sum += n;
    }
    for (auto& n : sizes) {
        biggest = max(biggest, n);
    }

    if (sum % 2 != 0) {
        cout << "no" << endl;
    } else {
        if (biggest <= sum / 2) {
            cout << "yes" << endl;
        } else {
            cout << "no" << endl;
        }
    }

    for (int i = 0; i < u; i++) {
        ll j, x;
        cin >> j >> x;
        sizes[j - 1] += x;
        biggest = max(biggest, sizes[j - 1]);
        sum += x;

        if (sum % 2 != 0) {
            cout << "no" << endl;
        } else {
            if (biggest <= sum / 2) {
                cout << "yes" << endl;
            } else {
                cout << "no" << endl;
            }
        }
    }
}
