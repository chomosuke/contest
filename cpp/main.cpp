#include <algorithm>     // IWYU pragma: keep
#include <bitset>        // IWYU pragma: keep
#include <cmath>         // IWYU pragma: keep
#include <cstdint>       // IWYU pragma: keep
#include <iostream>      // IWYU pragma: keep
#include <map>           // IWYU pragma: keep
#include <queue>         // IWYU pragma: keep
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
    int test_case_num{};
    std::cin >> test_case_num;
    for (int test_case = 0; test_case < test_case_num; test_case++) {
        int n{};
        int k{};
        std::cin >> n >> k;

        std::vector<int> arr1{};
        std::vector<int> arr2{};

        for (int i = 0; i < n; i++) {
            int a{};
            std::cin >> a;
            arr1.push_back(a);
        }
        for (int i = 0; i < n; i++) {
            int a{};
            std::cin >> a;
            arr2.push_back(a);
        }

        sort(arr1.begin(), arr1.end());
        sort(arr2.begin(), arr2.end());

        std::vector<int> arr1_double{};
        std::vector<int> arr1_single{};
        std::vector<int> arr2_double{};
        std::vector<int> arr2_single{};
        for (int i = 0; i < n; i++) {
            if (i < n - 1 && arr1[i] == arr1[i + 1]) {
                arr1_double.push_back(arr1[i]);
            } else if (i == 0 || arr1[i] != arr1[i - 1]) {
                arr1_single.push_back(arr1[i]);
            }
            if (i < n - 1 && arr2[i] == arr2[i + 1]) {
                arr2_double.push_back(arr2[i]);
            } else if (i == 0 || arr2[i] != arr2[i - 1]) {
                arr2_single.push_back(arr2[i]);
            }
        }

        sort(arr1_single.begin(), arr1_single.end());
        sort(arr2_single.begin(), arr2_single.end());

        std::vector<int> l{};
        std::vector<int> r{};
        int i = 0;
        while (k > 0) {
            if (i < arr1_double.size()) {
                l.push_back(arr1_double[i]);
                l.push_back(arr1_double[i]);
                r.push_back(arr2_double[i]);
                r.push_back(arr2_double[i]);
            } else {
                l.push_back(arr1_single[i - arr1_double.size()]);
                r.push_back(arr2_single[i - arr2_double.size()]);
                i++;
                l.push_back(arr1_single[i - arr1_double.size()]);
                r.push_back(arr2_single[i - arr2_double.size()]);
            }
            i++;
            k--;
        }

        for (auto l : l) {
            std::cout << l << " ";
        }
        std::cout << std::endl;
        for (auto r : r) {
            std::cout << r << " ";
        }
        std::cout << std::endl;
    }
}
