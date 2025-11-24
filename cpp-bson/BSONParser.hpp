#pragma once
#include <string>
#include <map>
#include <vector>
#include <variant>
#include <memory>
#include <stdexcept>
#include "Lexer.hpp"

// BSONValue structure to hold various types supported by BSON
struct BSONValue;

// Using shared_ptr for map to handle recursive structure safely
using BSONMap = std::map<std::string, BSONValue>;
using BSONArray = std::vector<BSONValue>;

// BSONValue
// A variant-like structure that can hold any of the supported data types.
// This is essential for a dynamically typed format like BSON.
struct BSONValue {
    enum Type { STRING, INT, FLOAT, BOOL, NULL_TYPE, ARRAY, OBJECT };
    Type type;
    
    // Using std::variant for type safety (C++17)
    // This allows us to store different types in a single field safely.
    std::variant<std::string, int, double, bool, std::monostate, BSONArray, std::shared_ptr<BSONMap>> value;

    BSONValue() : type(NULL_TYPE), value(std::monostate{}) {}
    BSONValue(std::string v) : type(STRING), value(v) {}
    BSONValue(const char* v) : type(STRING), value(std::string(v)) {}
    BSONValue(int v) : type(INT), value(v) {}
    BSONValue(double v) : type(FLOAT), value(v) {}
    BSONValue(bool v) : type(BOOL), value(v) {}
    BSONValue(BSONArray v) : type(ARRAY), value(v) {}
    BSONValue(std::shared_ptr<BSONMap> v) : type(OBJECT), value(v) {}

    // Print method to display the value recursively
    void print(int indent = 0) const;
};

// Function to print the entire AST
void printAST(const BSONMap& map);

// BSONParser Class
// Implements the parsing logic using Object-Oriented Principles.
// Encapsulates the state of the parsing process (stack, current level).
class BSONParser {
public:
    BSONParser();
    // Main parse method
    BSONMap parse(const std::string& content);

private:
    // Context for the stack to track nesting
    // OOP Concept: Encapsulation of state
    struct Context {
        std::shared_ptr<BSONMap> map;
        int level; // 0=Root, 1=Bulb, 2=Ivysaur, 3=Venusaur
    };

    std::vector<Context> stack;
    int currentLevel;

    // Helper methods
    BSONValue parseValueFromTokens(const std::vector<Token>& tokens, size_t& i);
    void validateKey(const std::string& key);
};
