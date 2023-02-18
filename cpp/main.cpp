#include <bits/stdc++.h>

using namespace std;

string ltrim(const string &);
string rtrim(const string &);

/*
 * Complete the 'closestStraightCity' function below.
 *
 * The function is expected to return a STRING_ARRAY.
 * The function accepts following parameters:
 *  1. STRING_ARRAY c
 *  2. INTEGER_ARRAY x
 *  3. INTEGER_ARRAY y
 *  4. STRING_ARRAY q
 */

struct City {
    int x;
    int y;
    string name;
};

struct Neighbour {
    int distance;
    string name;
};

vector<string> closestStraightCity(vector<string> c, vector<int> x,
                                   vector<int> y, vector<string> q) {
    vector<City> cities;

    for (size_t i = 0; i < c.size(); i++) {
        cities.push_back({.x = x[i], .y = y[i], .name = c[i]});
    }

    unordered_map<int, vector<City>> citiesWithY;
    unordered_map<int, vector<City>> citiesWithX;
    for (size_t i = 0; i < cities.size(); i++) {
        citiesWithX[cities[i].x].push_back(cities[i]);
        citiesWithY[cities[i].y].push_back(cities[i]);
    }
    for (auto it = citiesWithX.begin(); it != citiesWithX.end(); it++) {
        sort(it->second.begin(), it->second.end(),
             [](City a, City b) { return a.x < b.x; });
    }
    for (auto it = citiesWithY.begin(); it != citiesWithY.end(); it++) {
        sort(it->second.begin(), it->second.end(),
             [](City a, City b) { return a.y < b.y; });
    }

    unordered_map<string, City> cityWithName;
    for (size_t i = 0; i < cities.size(); i++) {
        cityWithName[cities[i].name] = cities[i];
    }

    vector<string> anss;
    for (size_t i = 0; i < q.size(); i++) {
        City city = cityWithName[q[i]];
        vector<Neighbour> neighbours;
        if (citiesWithY[city.y].size() > 1) {
            vector<City> cities = citiesWithY[city.y];
            size_t index =
                distance(cities.begin(),
                         lower_bound(cities.begin(), cities.end(), city));
            if (index - 1 >= 0) {
                neighbours.push_back(
                    {.distance = abs(cities[index - 1].x - cities[index].x),
                     .name = cities[index - 1].name});
            }
            if (index + 1 < cities.size()) {
                neighbours.push_back(
                    {.distance = abs(cities[index + 1].x - cities[index].x),
                     .name = cities[index + 1].name});
            }
        }

        if (citiesWithX[city.x].size() > 1) {
            vector<City> cities = citiesWithX[city.x];
            size_t index =
                distance(cities.begin(),
                         lower_bound(cities.begin(), cities.end(), city));
            if (index - 1 >= 0) {
                neighbours.push_back(
                    {.distance = abs(cities[index - 1].y - cities[index].y),
                     .name = cities[index - 1].name});
            }
            if (index + 1 < cities.size()) {
                neighbours.push_back(
                    {.distance = abs(cities[index + 1].y - cities[index].y),
                     .name = cities[index + 1].name});
            }
        }

        sort(neighbours.begin(), neighbours.end(),
             [](Neighbour a, Neighbour b) {
                 if (a.distance < b.distance) {
                     return true;
                 } else if (b.distance < a.distance) {
                     return false;
                 } else {
                     return a.name.compare(b.name) < 0;
                 }
             });

        if (neighbours.empty()) {
            anss.push_back("NONE");
        } else {
            anss.push_back(neighbours[0].name);
        }
    }

    return anss;
}
int main() {
    ofstream fout(getenv("OUTPUT_PATH"));

    string c_count_temp;
    getline(cin, c_count_temp);

    int c_count = stoi(ltrim(rtrim(c_count_temp)));

    vector<string> c(c_count);

    for (int i = 0; i < c_count; i++) {
        string c_item;
        getline(cin, c_item);

        c[i] = c_item;
    }

    string x_count_temp;
    getline(cin, x_count_temp);

    int x_count = stoi(ltrim(rtrim(x_count_temp)));

    vector<int> x(x_count);

    for (int i = 0; i < x_count; i++) {
        string x_item_temp;
        getline(cin, x_item_temp);

        int x_item = stoi(ltrim(rtrim(x_item_temp)));

        x[i] = x_item;
    }

    string y_count_temp;
    getline(cin, y_count_temp);

    int y_count = stoi(ltrim(rtrim(y_count_temp)));

    vector<int> y(y_count);

    for (int i = 0; i < y_count; i++) {
        string y_item_temp;
        getline(cin, y_item_temp);

        int y_item = stoi(ltrim(rtrim(y_item_temp)));

        y[i] = y_item;
    }

    string q_count_temp;
    getline(cin, q_count_temp);

    int q_count = stoi(ltrim(rtrim(q_count_temp)));

    vector<string> q(q_count);

    for (int i = 0; i < q_count; i++) {
        string q_item;
        getline(cin, q_item);

        q[i] = q_item;
    }

    vector<string> result = closestStraightCity(c, x, y, q);

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
