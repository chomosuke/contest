#include <algorithm>
#include <bitset>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <map>
#include <queue>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

typedef intmax_t Int;
typedef long double Float;

vector<Int> coins;

map<tuple<Int, Int>, Int> memoize;
Int min_coin_count(Int target, Int index) {
    if (memoize.count({target, index}) == 1) {
        return memoize[{target, index}];
    }
    if (target == 0) {
        memoize[{target, index}] = 0;
        return memoize[{target, index}];
    }
    if (index == coins.size()) {
        // no coin left
        // since target != 0 => impossible to satisfy
        memoize[{target, index}] = -1;
        return memoize[{target, index}];
    }
    Int exclude = min_coin_count(target, index + 1);
    if (target < coins[index]) {
        // can't include because target will go negative
        memoize[{target, index}] = exclude;
        return memoize[{target, index}];
    }
    Int include = 1 + min_coin_count(target - coins[index], index + 1);
    if (include == 0) {
        memoize[{target, index}] = exclude;
    } else if (exclude == -1) {
        memoize[{target, index}] = include;
    } else {
        memoize[{target, index}] = min(include, exclude);
    }
    return memoize[{target, index}];
}

int main() {
    // input:
    // n c1 c2 c3 ... cn t
    // find the smallest amount of coin that'll make upto t
    // if impossible, print -1
    Int n;
    cin >> n;
    for (Int i = 0; i < n; i++) {
        Int coin;
        cin >> coin;
        coins.push_back(coin);
    }
    Int t;
    cin >> t;
    cout << min_coin_count(t, 0);
}
