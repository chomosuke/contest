#include <algorithm>     // IWYU pragma: keep
#include <bitset>        // IWYU pragma: keep
#include <cassert>       // IWYU pragma: keep
#include <cmath>         // IWYU pragma: keep
#include <cstdint>       // IWYU pragma: keep
#include <fstream>       // IWYU pragma: keep
#include <iostream>      // IWYU pragma: keep
#include <map>           // IWYU pragma: keep
#include <numeric>       // IWYU pragma: keep
#include <queue>         // IWYU pragma: keep
#include <set>           // IWYU pragma: keep
#include <string>        // IWYU pragma: keep
#include <string_view>   // IWYU pragma: keep
#include <tuple>         // IWYU pragma: keep
#include <unordered_map> // IWYU pragma: keep
#include <unordered_set> // IWYU pragma: keep
#include <vector>        // IWYU pragma: keep

/*
 * Complete the 'stringSimilarity' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

int common_len(const std::string &s, int i, int j) {
    int k{0};
    for (; k + i < s.size() && k + j < s.size(); k++) {
        if (s[i + k] != s[j + k]) {
            break;
        }
    }
    return k;
}

std::vector<int> zArrayHelper(std::string s) {
    std::vector<int> z(s.size(), s.size());
    int l = 0;
    int r = 0;
    int k;
    for (int i = 1; i < s.size(); i++) {
        if (i > r) {
            l = i;
            r = i;
            while (r < s.size() && s[r - l] == s[r]) {
                r++;
            }
            z[i] = r - l;
            r--;
        } else {
            k = i - l;
            if (z[k] < r + 1 - i) {
                z[i] = z[k];
            } else {
                l = i;
                while (r < s.size() && s[r - l] == s[r]) {
                    r++;
                }
                z[i] = r - l;
                r--;
            }
        }
    }
    return z;
}

int stringSimilarity(std::string s) {
    std::vector<int> z{static_cast<int>(s.size())};
    while (z.size() < s.size()) {
        int i{static_cast<int>(z.size())};
        z.push_back(common_len(s, 0, i));
        while (z.size() < i + z[i]) {
            int j{static_cast<int>(z.size())};
            if (j + z[j - i] < i + z[i]) {
                z.push_back(z[j - i]);
            } else {
                int known_common{i + z[i] - j};
                z.push_back(common_len(s, known_common, j + known_common) +
                            known_common);
                i = j;
            }
        }
    }
    std::vector<int> z2 = zArrayHelper(s);

    for (int i{0}; i < z.size(); i++) {
        assert(z[i] == z2[i]);
    }

    assert(z.size() == z2.size());

    return std::accumulate(z.begin(), z.end(), 0, std::plus<int>());
}

int stringSimilarityB(std::string s) {
    int r{0};
    for (int i{0}; i < s.size(); i++) {
        int j{0};
        while (j + i < s.size() && s[j] == s[i + j]) {
            j++;
        }
        r += j;
    }
    return r;
}

int main() {
    assert(stringSimilarity("aaaaaaaaaaab") ==
           stringSimilarityB("aaaaaaaaaaab"));
    assert(stringSimilarity("abababababab") ==
           stringSimilarityB("abababababab"));
    assert(stringSimilarity("ababcabababab") ==
           stringSimilarityB("ababcabababab"));
    assert(stringSimilarity("ababcaauestnahoesthakbabababab") ==
           stringSimilarityB("ababcaauestnahoesthakbabababab"));
}
