#include <algorithm>     // IWYU pragma: keep
#include <bitset>        // IWYU pragma: keep
#include <cmath>         // IWYU pragma: keep
#include <cstdint>       // IWYU pragma: keep
#include <iostream>      // IWYU pragma: keep
#include <map>           // IWYU pragma: keep
#include <queue>         // IWYU pragma: keep
#include <set>           // IWYU pragma: keep
#include <string>        // IWYU pragma: keep
#include <string_view>   // IWYU pragma: keep
#include <tuple>         // IWYU pragma: keep
#include <unordered_map> // IWYU pragma: keep
#include <unordered_set> // IWYU pragma: keep
#include <vector>        // IWYU pragma: keep

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
        size_t seed{0};
        HashValueImpl<std::tuple<TT...>>::apply(seed, tt);
        return seed;
    }
};
} // namespace hash_tuple

// unordered_map<tuple<Int, Float>, Int, hash_tuple::hash<tuple<Int, Float>>>
//     memoize;

// typedef int64_t Int;
// [[maybe_unused]] const Int Int_max = INT64_MAX;
// [[maybe_unused]] const Int Int_min = INT64_MIN;
// typedef long double Float;

int main() {
    int test_cases_count{};
    std::cin >> test_cases_count;
    for (int test_case = 0; test_case < test_cases_count; test_case++) {
        std::vector<int> arr{};
        int n{};
        std::cin >> n;
        for (int i = 0; i < n; i++) {
            int a{};
            std::cin >> a;
            arr.push_back(a);
        }
        std::sort(arr.begin(), arr.end());
        int result{0};
        int mid = (arr.size() - 1) / 2;
        for (int i = 0; i + mid < arr.size(); i++) {
            int j = i + mid;
            if (arr[j] > arr[mid]) {
                result = i;
                break;
            }
        }
        if (result == 0) {
            result = arr.size() - mid;
        }
        std::cout << result << std::endl;
    }
}
