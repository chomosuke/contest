#include <bits/stdc++.h>

using namespace std;

string ltrim(const string &);
string rtrim(const string &);



/*
 * Complete the 'minMoves' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER startRow
 *  3. INTEGER startCol
 *  4. INTEGER endRow
 *  5. INTEGER endCol
 */

int minMoves(int n, int startRow, int startCol, int endRow, int endCol) {
    vector<vector<int>> dist(n, vector<int>(n, -1));
    queue<tuple<int, int>> toVisit;
    dist[startRow][startCol] = 0;
    toVisit.push(make_tuple(startRow, startCol));
    while (!toVisit.empty() && dist[endRow][endCol] == -1) {
        int row = get<0>(toVisit.front());
        int col = get<1>(toVisit.front());
        int d = dist[row][col] + 1;
        tuple<int, int> nexts[8] = {
            {row + 2, col + 1},
            {row - 2, col + 1},
            {row + 2, col - 1},
            {row - 2, col - 1},
            {row + 1, col + 2},
            {row - 1, col + 2},
            {row + 1, col - 2},
            {row - 1, col - 2},
        };
        for (auto& next : nexts) {
            int row = get<0>(next);
            int col = get<1>(next);
            if (row >= 0 && row < n && col >= 0 && col < n && dist[row][col] == -1) {
                dist[row][col] = d;
                toVisit.push(next);
            }
        }
        toVisit.pop();
    }
    return dist[endRow][endCol];
}

int main()
{
    ofstream fout(getenv("OUTPUT_PATH"));

    string n_temp;
    getline(cin, n_temp);

    int n = stoi(ltrim(rtrim(n_temp)));

    string startRow_temp;
    getline(cin, startRow_temp);

    int startRow = stoi(ltrim(rtrim(startRow_temp)));

    string startCol_temp;
    getline(cin, startCol_temp);

    int startCol = stoi(ltrim(rtrim(startCol_temp)));

    string endRow_temp;
    getline(cin, endRow_temp);

    int endRow = stoi(ltrim(rtrim(endRow_temp)));

    string endCol_temp;
    getline(cin, endCol_temp);

    int endCol = stoi(ltrim(rtrim(endCol_temp)));

    int result = minMoves(n, startRow, startCol, endRow, endCol);

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
