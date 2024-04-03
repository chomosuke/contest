#include <bits/stdc++.h>

using namespace std;

string ltrim(const string &);
string rtrim(const string &);

/*
 * Complete the 'romanizer' function below.
 *
 * The function is expected to return a STRING_ARRAY.
 * The function accepts INTEGER_ARRAY numbers as parameter.
 */

vector<string> romanizer(vector<int> numbers) {
    vector<string> result;
    for (auto number : numbers) {
        const int max_digit = 4;
        int digits[max_digit] = {
            number % 10,
            number % 100 / 10,
            number % 1000 / 100,
            number / 1000,
        };
        char presentation[max_digit][2] = {
            {'I', 'V'},
            {'X', 'L'},
            {'C', 'D'},
            {'M', 'M'},
        };
        stringstream roman;
        for (int i = max_digit - 1; i >= 0; i--) {
            if (digits[i] <= 3) {
                for (int j = 0; j < digits[i]; j++) {
                    roman << presentation[i][0];
                }
            } else if (digits[i] == 4) {
                roman << presentation[i][0];
                roman << presentation[i][1];
            } else if (digits[i] < 9) {
                roman << presentation[i][1];
                for (int j = 0; j < digits[i] - 5; j++) {
                    roman << presentation[i][0];
                }
            } else {
                roman << presentation[i][0];
                roman << presentation[i + 1][0];
            }
        }
        result.push_back(roman.str());
    }
    return result;
}

int main() {
    ofstream fout(getenv("OUTPUT_PATH"));

    string numbers_count_temp;
    getline(cin, numbers_count_temp);

    int numbers_count = stoi(ltrim(rtrim(numbers_count_temp)));

    vector<int> numbers(numbers_count);

    for (int i = 0; i < numbers_count; i++) {
        string numbers_item_temp;
        getline(cin, numbers_item_temp);

        int numbers_item = stoi(ltrim(rtrim(numbers_item_temp)));

        numbers[i] = numbers_item;
    }

    vector<string> result = romanizer(numbers);

    for (size_t i = 0; i < result.size(); i++) {
        fout << result[i];

        if (i != result.size() - 1) {
            fout << "\n";
        }
    }

    fout << "\n";

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
