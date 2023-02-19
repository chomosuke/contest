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
//     memoize;

using namespace std;

typedef int64_t Int;
const Int Int_max = INT64_MAX;
const Int Int_min = INT64_MIN;
typedef long double Float;

struct CycleLenCount {
    Int len;
    Int count;
};

unordered_map<tuple<Int, Int>, Int, hash_tuple::hash<tuple<Int, Int>>> memoize;

// how many swap do we need if only cycle_len_counts[i, end) can be used
Int count_swaps(vector<CycleLenCount> const &cycle_len_counts, Int i,
                Int const target) {
    if (memoize.find(tuple(i, target)) != memoize.end()) {
        return memoize[tuple(i, target)];
    }
    Int swaps = Int_max / 2;
    if (i >= cycle_len_counts.size()) {
        return swaps;
    }
    for (Int include = 0; include <= cycle_len_counts[i].count; include++) {
        if (target - include * cycle_len_counts[i].len > 0) {
            swaps =
                min(swaps,
                    include + count_swaps(
                                  cycle_len_counts, i + 1,
                                  target - include * cycle_len_counts[i].len));
        } else if (target - include * cycle_len_counts[i].len == 0) {
            swaps = min(swaps, include - 1);
        } else {
            break;
        }
    }
    memoize[tuple(i, target)] = swaps;
    return swaps;
}

int main() {
    Int case_count;
    cin >> case_count;
    for (Int case_number = 1; case_number <= case_count; case_number++) {
        memoize.clear();

        Int n;
        cin >> n;
        Int *ps = new Int[n];
        for (Int i = 0; i < n; i++) {
            Int p;
            cin >> p;
            ps[i] = p - 1;
        }

        bool *visited = new bool[n];
        for (int i = 0; i < n; i++) {
            visited[i] = false;
        }
        unordered_map<Int, Int> cycle_counts;
        Int i = 0;
        while (i < n) {
            visited[i] = true;
            Int j = ps[i];
            Int length = 1;
            while (j != i) {
                length++;
                visited[j] = true;
                j = ps[j];
            }
            cycle_counts[length]++;
            while (i < n && visited[i]) {
                i++;
            }
        }

        vector<CycleLenCount> cycle_len_counts;
        for (auto it = cycle_counts.begin(); it != cycle_counts.end(); it++) {
            cycle_len_counts.push_back({.len = it->first, .count = it->second});
        }

        sort(cycle_len_counts.begin(), cycle_len_counts.end(),
             [](CycleLenCount a, CycleLenCount b) { return a.len > b.len; });

        cout << "Case #" << case_number << ":";

        for (Int target = 1; target <= n; target++) {
            // two senario
            // 1. n biggest cycle add together and cut one to form the target =
            // n + 1
            Int sum = 0;
            Int swaps = 0;
            for (Int i = 0; true; i++) {
                if (sum + cycle_len_counts[i].len * cycle_len_counts[i].count >=
                    target) {
                    swaps +=
                        ceil((double)(target - sum) / cycle_len_counts[i].len);
                    break;
                } else {
                    sum += cycle_len_counts[i].len * cycle_len_counts[i].count;
                    swaps += cycle_len_counts[i].count;
                }
            }
            // 2. n cycle add together to form the target = n
            swaps = min(swaps, count_swaps(cycle_len_counts, 0, target));
            cout << " " << swaps;
        }
        cout << endl;
    }
}
