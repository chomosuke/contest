#include <bits/stdc++.h>
using namespace std;

struct Variable {
    int name;
    bool operator==(const Variable &oth) const { return name == oth.name; }
    bool operator!=(const Variable &oth) const { return !(operator==(oth)); }
};

struct Abstraction;
struct Application;

typedef variant<Variable, Abstraction, Application> Lambda;

struct Abstraction {
    Variable input;
    Lambda *term;
};

struct Application {
    Lambda *input;
    Lambda *output;
};

class LambdaCalculus {
  public:
    Lambda *Parse(const string &str) {}

    string ToString(Lambda *term) {
        if (holds_alternative<Variable>(*term)) {
            return to_string(get<Variable>(*term).name);
        } else if (holds_alternative<Application>(*term)) {
            const Application &app = get<Application>(*term);
            string left = ToString(app.input);
            string right = ToString(app.output);
            return "(" + left + ") (" + right + ")";
        } else {
            Abstraction &abst = get<Abstraction>(*term);
            return "lambda" + to_string(abst.input.name) + ": " +
                   ToString(abst.term);
        }
    }

    unordered_set<Variable> FreeVariables(const Lambda &term) {
        if (holds_alternative<Variable>(term)) {
            return {get<Variable>(term)};
        } else if (holds_alternative<Application>(term)) {
            const Application &app = get<Application>(term);
            auto left = FreeVariables(*app.input);
            auto right = FreeVariables(*app.output);
            left.merge(right);
            return left;
        } else {
            Abstraction &abst = get<Abstraction>(term);
            auto freeVars = FreeVariables(*abst.term);
            freeVars.erase(abst.input);
            return freeVars;
        }
    }

    void Substitute(Lambda *term, const Variable &variable,
                    const Lambda &replacement) {
        if (holds_alternative<Variable>(*term)) {
            if (get<Variable>(*term) == variable) {
                *term = replacement;
            }
        } else if (holds_alternative<Application>(*term)) {
            Application &app = get<Application>(*term);
            Substitute(app.input, variable, replacement);
            Substitute(app.output, variable, replacement);
        } else {
            Abstraction &abst = get<Abstraction>(*term);
            if (abst.input != variable) {
                const auto &freeVars = FreeVariables(replacement);
                if (freeVars.find(variable) == freeVars.end()) {
                    Substitute(abst.term, variable, replacement);
                }
            }
        }
    }

    void Reduce(Lambda *term) {
        if (holds_alternative<Variable>(*term))
            return;

        if (holds_alternative<Application>(*term)) {
            Application &app = get<Application>(*term);
            if (holds_alternative<Abstraction>(*app.input)) {
                Abstraction &abst = get<Abstraction>(*app.input);
                Lambda newTerm = {*abst.term};
                Substitute(&newTerm, abst.input, *app.output);
                if (*term != newTerm) {
                    /* This is not full cycle detection */
                    *term = newTerm;
                    Reduce(term);
                }
            }
        } else {
            Abstraction &abst = get<Abstraction>(*term);
            Reduce(abst.term);
        }
    }

  private:
    vector<unique_ptr<Lambda>> mTerms;
};

int main() {
    string input;
    cin >> input;
    LambdaCalculus lc;
    Lambda *lambda = lc.Parse(input);
    lc.Reduce(lambda);
    cout << lc.ToString(lambda);
}
