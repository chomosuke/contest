#include <bits/stdc++.h>

using namespace std;

string ltrim(const string &);
string rtrim(const string &);



/*
 * Complete the 'getMaximumPower' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts INTEGER_ARRAY power as parameter.
 */

long getMaximumPower(vector<int> power) {
    map<int, int> ps{};
    for (auto p : power) {
        if (ps.find(p) == ps.end()) {
            ps.insert({p, 1});
        } else {
            ps[p]++;
        }
    }
    int prevPow = -2;
    vector<vector<long>> segments{};
    for (auto const &[p, freq] : ps) {
        if (p == prevPow + 1) {
            segments.back().push_back(p * freq);
        } else {
            segments.push_back(vector<long>{p * freq});
        }
        prevPow = p;
    }

    long totalPow{0};
    for (auto const &segment : segments) {
        long pow1 = 0;
        long pow2 = 0;
        for (int i = 0; i < segment.size(); i += 2) {
            pow1 += segment[i];
        }
        for (int i = 1; i < segment.size(); i += 2) {
            pow2 += segment[i];
        }
        totalPow += max(pow1, pow2);
    }

    return totalPow;
}

int main()
{
    ofstream fout(getenv("OUTPUT_PATH"));

    string power_count_temp;
    getline(cin, power_count_temp);

    int power_count = stoi(ltrim(rtrim(power_count_temp)));

    vector<int> power(power_count);

    for (int i = 0; i < power_count; i++) {
        string power_item_temp;
        getline(cin, power_item_temp);

        int power_item = stoi(ltrim(rtrim(power_item_temp)));

        power[i] = power_item;
    }

    long result = getMaximumPower(power);

    fout << result << "\n";

    fout.close();

    return 0;
}

string ltrim(const string &str) {
    string s(str);

    s.erase(
        s.begin(),
        find_if(s.begin(), s.end(), not1(ptr_fun<int, int>(isspace)))
    );

    return s;
}

string rtrim(const string &str) {
    string s(str);

    s.erase(
        find_if(s.rbegin(), s.rend(), not1(ptr_fun<int, int>(isspace))).base(),
        s.end()
    );

    return s;
}
