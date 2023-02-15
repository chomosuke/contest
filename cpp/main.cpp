#include <algorithm>
#include <array>
#include <bitset>
#include <cassert>
#include <climits>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <limits>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

struct Limit {
    double maxValue;
    int maxVolume10Seconds;
    double maxValue1Second;
};

struct Volume {
    int volume;
    uint64_t timestamp;
};

struct Value {
    double value;
    uint64_t timestamp;
};

struct History {
    int volume10Seconds;
    double value1Second;
    queue<Volume> volumes;
    queue<Value> values;
};

// This is the class you need to implement! Feel free to add members, private
// methods etc, but don't change the public method signatures.
class RiskLimitProcessor {
  private:
    unordered_map<string, Limit> limits;
    unordered_map<string, History> histories;

  public:
    void AddLimit(const std::string &instrument, double maxValue,
                  int maxVolume10Seconds, double maxValue1Second) {
        limits[instrument] = {
            .maxValue = maxValue,
            .maxVolume10Seconds = maxVolume10Seconds,
            .maxValue1Second = maxValue1Second,
        };
    }

    void ProcessOrder(const std::string &instrument, uint64_t timestamp,
                      int volume, double price) {
        // correct value1Second & volume10Seconds
        while (!histories[instrument].values.empty() &&
               histories[instrument].values.front().timestamp <=
                   timestamp - 1000) {
            Value value = histories[instrument].values.front();
            histories[instrument].values.pop();
            histories[instrument].value1Second -= value.value;
        }
        while (!histories[instrument].volumes.empty() &&
               histories[instrument].volumes.front().timestamp <=
                   timestamp - 10000) {
            Volume volume = histories[instrument].volumes.front();
            histories[instrument].volumes.pop();
            histories[instrument].volume10Seconds -= volume.volume;
        }

        if (limits.find(instrument) == limits.end()) {
            cout << "NO_LIMITS " << instrument << endl;
        } else if (limits[instrument].maxValue < volume * price) {
            cout << "MAX_VAL_LIMIT " << instrument << endl;
        } else if (limits[instrument].maxVolume10Seconds <
                   volume + histories[instrument].volume10Seconds) {
            cout << "MAX_VOL_10S_LIMIT " << instrument << endl;
        } else if (limits[instrument].maxValue1Second <
                   volume * price + histories[instrument].value1Second) {
            cout << "MAX_VAL_1S_LIMIT " << instrument << endl;
        }
        // record history
        histories[instrument].volumes.push({
            .volume = volume,
            .timestamp = timestamp,
        });
        histories[instrument].volume10Seconds += volume;
        histories[instrument].values.push({
            .value = volume * price,
            .timestamp = timestamp,
        });
        histories[instrument].value1Second += volume * price;
    }
};

int main() {
    RiskLimitProcessor riskLimitProcessor;
    while (!std::cin.eof()) {
        std::string action, instrument;
        std::cin >> action >> instrument;
        if (action.empty())
            break; // handle whitespace at end of input
        if (action == "LIMIT") {
            double maxValue;
            int maxVolume10Seconds;
            double maxValue1Second;
            std::cin >> maxValue >> maxVolume10Seconds >> maxValue1Second;
            riskLimitProcessor.AddLimit(instrument, maxValue,
                                        maxVolume10Seconds, maxValue1Second);
        } else if (action == "ORDER") {
            uint64_t timestamp;
            int volume;
            double price;
            std::cin >> timestamp >> volume >> price;
            riskLimitProcessor.ProcessOrder(instrument, timestamp, volume,
                                            price);
        } else {
            std::cerr << "Malformed input!\n";
            return -1;
        }
    }
    return 0;
}
