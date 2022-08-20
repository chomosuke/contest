#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

typedef long long ll;

int main() {
    ll n;
    cin >> n;
    vector<vector<char>> grid;
    for (ll i = 0; i < n; i++) {
        vector<char> row;
        grid.push_back(row);
        for (ll j = 0; j < n; j++) {
            char c;
            cin >> c;
            grid[i].push_back(c);
        }
    }
    for (ll i = 0; i < n; i++) {
        for (ll j = 0; j < n; j++) {
            bool print = true;
            for (ll k = 0; k < n; k++) {
                if (k != i && grid[i][j] == grid[k][j]) {
                    print = false;
                    break;
                }
                if (k != j && grid[i][j] == grid[i][k]) {
                    print = false;
                    break;
                }
            }
            if (print) {
                cout << grid[i][j];
            }
        }
    }
    cout << endl;
}
