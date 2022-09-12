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

namespace hash_tuple {
template <typename TT> struct hash {
    size_t operator()(TT const &tt) const { return std::hash<TT>()(tt); }
};
namespace {

template <class T> inline void hash_combine(std::size_t &seed, T const &v) {
    seed ^= hash_tuple::hash<T>()(v) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
}

template <class Tuple, size_t Index = std::tuple_size<Tuple>::value - 1>
struct HashValueImpl {
    static void apply(size_t &seed, Tuple const &tuple) {
        HashValueImpl<Tuple, Index - 1>::apply(seed, tuple);
        hash_combine(seed, std::get<Index>(tuple));
    }
};

template <class Tuple> struct HashValueImpl<Tuple, 0> {
    static void apply(size_t &seed, Tuple const &tuple) {
        hash_combine(seed, std::get<0>(tuple));
    }
};
} // namespace

template <typename... TT> struct hash<std::tuple<TT...>> {
    size_t operator()(std::tuple<TT...> const &tt) const {
        size_t seed = 0;
        HashValueImpl<std::tuple<TT...>>::apply(seed, tt);
        return seed;
    }
};
} // namespace hash_tuple
// unordered_map<tuple<Int, Float>, Int, hash_tuple::hash<tuple<Int, Float>>>
// memoize;

using namespace std;

typedef int64_t Int;
const Int Int_max = INT64_MAX;
const Int Int_min = INT64_MIN;
typedef long double Float;

vector<Int> coins;

unordered_map<Int, Int> first_coin;

unordered_map<tuple<Int>, Int, hash_tuple::hash<tuple<Int>>> memoize;
Int min_coin_count(Int target) {
    if (memoize.count({target}) == 1) {
        return memoize[{target}];
    }
    if (target == 0) {
        return 0;
    }
    Int best = Int_max;
    for (auto coin : coins) {
        if (target >= coin) {
            Int rec = min_coin_count(target - coin);
            if (rec != Int_max && 1 + rec < best) {
                first_coin[target] = coin;
                best = 1 + rec;
            }
        }
    }
    memoize[{target}] = best;
    return best;
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
    Int count = min_coin_count(t);
    if (count != Int_max) {
        cout << count << endl;
    } else {
        cout << -1 << endl;
        return 0;
    }
    Int target = t;
    if (target > 0){
        cout << first_coin[target];
        target -= first_coin[target];
    }
    while (target > 0) {
        cout << ' ' << first_coin[target];
        target -= first_coin[target];
    }
    cout << endl;
}
