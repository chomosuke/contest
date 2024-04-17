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

int main() {
    int test_count{};
    std::cin >> test_count;
    for (int test_case = 0; test_case < test_count; test_case++) {
        int n{};
        std::cin >> n;
        int a{2};
        for (; a * a * a < n; a++) {
            if (n % a == 0) {
                break;
            }
        }
        if (n % a != 0) {
            std::cout << "NO" << std::endl;
            continue;
        }
        n /= a;
        int b{a + 1};
        for (; b * b < n; b++) {
            if (n % b == 0) {
                break;
            }
        }
        if (b * b < n && n % b == 0) {
            std::cout << "YES" << std::endl;
            std::cout << a << " " << b << " " << n / b << std::endl;
        } else {
            std::cout << "NO" << std::endl;
        }
    }
}
