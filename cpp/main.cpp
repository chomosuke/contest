#include <bits/stdc++.h>

using namespace std;

string ltrim(const string &);
string rtrim(const string &);

/*
 * Complete the 'findLowestStartingStair' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY jumps as parameter.
 */

int findLowestStartingStair(vector<int> jumps) {
    int case_{1};
    int min_case{1};
    for (auto jump : jumps) {
        case_ += jump;
        min_case = min(min_case, case_);
    }
    if (min_case < 1) {
        return 1 - min_case + 1;
    } else {
        return 1;
    }
}

int main() {
    ofstream fout(getenv("OUTPUT_PATH"));

    string jumps_count_temp;
    getline(cin, jumps_count_temp);

    int jumps_count = stoi(ltrim(rtrim(jumps_count_temp)));

    vector<int> jumps(jumps_count);

    for (int i = 0; i < jumps_count; i++) {
        string jumps_item_temp;
        getline(cin, jumps_item_temp);

        int jumps_item = stoi(ltrim(rtrim(jumps_item_temp)));

        jumps[i] = jumps_item;
    }

    int result = findLowestStartingStair(jumps);

    fout << result << "\n";

    fout.close();

    return 0;
}

string ltrim(const string &str) {
    string s(str);

    s.erase(s.begin(),
            find_if(s.begin(), s.end(), not1(ptr_fun<int, int>(isspace))));

    return s;
}

string rtrim(const string &str) {
    string s(str);

    s.erase(
        find_if(s.rbegin(), s.rend(), not1(ptr_fun<int, int>(isspace))).base(),
        s.end());

    return s;
}
