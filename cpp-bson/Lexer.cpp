#include "Lexer.hpp"
#include <sstream>
#include <regex>
#include <iostream>
#include <algorithm>

Lexer::Lexer(const std::string& content) : content(content) {}

// tokenize
// The main loop that processes the input line by line.
// It handles high-level structure like headers, comments, and indentation.
std::vector<Token> Lexer::tokenize() {
    std::stringstream ss(content);
    std::string line;
    int lineNum = 0;
    bool firstLine = true;

    while (std::getline(ss, line)) {
        lineNum++;
        // Handle Windows-style line endings
        if (!line.empty() && line.back() == '\r') line.pop_back();

        // Header Check: The Cry
        if (firstLine) {
            if (line != "BULBA!") {
                throw std::runtime_error("Status: Fainted");
            }
            tokens.push_back({TOKEN_HEADER, "BULBA!", lineNum, 0});
            firstLine = false;
            continue;
        }

        // Handle comments (Sleep Powder)
        // We strip out comments before further processing.
        size_t commentPos = line.find("zZz");
        if (commentPos != std::string::npos) {
            line = line.substr(0, commentPos);
        }

        // Trim right whitespace
        line.erase(std::find_if(line.rbegin(), line.rend(), [](unsigned char ch) {
            return !std::isspace(ch);
        }).base(), line.end());

        if (line.empty()) continue;

        // Check indentation (Solar Beam Rule)
        // We count spaces to determine the indentation level.
        int indentCount = 0;
        bool hasTab = false;
        for (char c : line) {
            if (c == ' ') indentCount++;
            else if (c == '\t') { hasTab = true; break; }
            else break;
        }

        if (hasTab) throw std::runtime_error("Poison Type: Tab character detected");
        if (indentCount % 4 != 0) throw std::runtime_error("The attack missed!");

        int level = indentCount / 4;
        // Emit an INDENT token so the parser knows the nesting level of this line.
        tokens.push_back({TOKEN_INDENT, "", lineNum, level});

        std::string trimmedLine = trim(line);
        tokenizeLine(trimmedLine, lineNum);
    }
    
    tokens.push_back({TOKEN_EOF, "", lineNum, 0});
    return tokens;
}

// tokenizeLine
// Processes a single line after indentation has been handled.
// Identifies Section Headers or Key-Value pairs.
void Lexer::tokenizeLine(const std::string& line, int lineNum) {
    // Section Headers (Evolution Stages)
    // We check for specific patterns like (o) ... (o)
    if (startsWith(line, "(o) ") && endsWith(line, " (o)")) {
        tokens.push_back({TOKEN_SECTION_OPEN, "", lineNum, 1});
        std::string key = line.substr(4, line.length() - 8);
        tokens.push_back({TOKEN_IDENTIFIER, key, lineNum, 0});
        tokens.push_back({TOKEN_SECTION_CLOSE, "", lineNum, 1});
        return;
    }
    if (startsWith(line, "(O) ") && endsWith(line, " (O)")) {
        tokens.push_back({TOKEN_SECTION_OPEN, "", lineNum, 2});
        std::string key = line.substr(4, line.length() - 8);
        tokens.push_back({TOKEN_IDENTIFIER, key, lineNum, 0});
        tokens.push_back({TOKEN_SECTION_CLOSE, "", lineNum, 2});
        return;
    }
    if (startsWith(line, "(@) ") && endsWith(line, " (@)")) {
        tokens.push_back({TOKEN_SECTION_OPEN, "", lineNum, 3});
        std::string key = line.substr(4, line.length() - 8);
        tokens.push_back({TOKEN_IDENTIFIER, key, lineNum, 0});
        tokens.push_back({TOKEN_SECTION_CLOSE, "", lineNum, 3});
        return;
    }

    // Key-Value Pairs
    // We use Regex to capture the key, the vine whip, and the value.
    std::regex re("^([a-zA-Z0-9_]+)\\s*(~{1,}>)\\s*(.*)$");
    std::smatch matches;
    if (std::regex_match(line, matches, re)) {
        std::string key = matches[1];
        std::string valStr = matches[3];
        
        tokens.push_back({TOKEN_IDENTIFIER, key, lineNum, 0});
        tokens.push_back({TOKEN_VINE_WHIP, "", lineNum, 0});
        tokenizeValue(valStr, lineNum);
        return;
    }

    throw std::runtime_error("It hurt itself in its confusion!");
}

// tokenizeValue
// Parses the value part of a key-value pair.
void Lexer::tokenizeValue(const std::string& valStr, int lineNum) {
    std::string s = trim(valStr);
    if (s.empty()) return;

    // String Literal
    if (startsWith(s, "\"") && endsWith(s, "\"")) {
        tokens.push_back({TOKEN_STRING, s.substr(1, s.length() - 2), lineNum, 0});
        return;
    }
    // Boolean: SuperEffective (True)
    if (s == "SuperEffective") {
        tokens.push_back({TOKEN_BOOL, "true", lineNum, 0});
        return;
    }
    // Boolean: NotVeryEffective (False)
    if (s == "NotVeryEffective") {
        tokens.push_back({TOKEN_BOOL, "false", lineNum, 0});
        return;
    }
    // Null: MissingNo
    if (s == "MissingNo") {
        tokens.push_back({TOKEN_NULL, "", lineNum, 0});
        return;
    }
    // Array: <| ... |>
    if (startsWith(s, "<|") && endsWith(s, "|>")) {
        tokens.push_back({TOKEN_ARRAY_START, "", lineNum, 0});
        std::string inner = s.substr(2, s.length() - 4);
        std::stringstream ss(inner);
        std::string segment;
        bool first = true;
        while(std::getline(ss, segment, ',')) {
            if (!first) tokens.push_back({TOKEN_COMMA, "", lineNum, 0});
            tokenizeValue(segment, lineNum); // Recursive call for array elements
            first = false;
        }
        tokens.push_back({TOKEN_ARRAY_END, "", lineNum, 0});
        return;
    }

    // Number (Int/Float)
    try {
        size_t pos;
        std::stoi(s, &pos);
        if (pos == s.length()) {
            tokens.push_back({TOKEN_NUMBER, s, lineNum, 0});
            return;
        }
    } catch (...) {}

    try {
        size_t pos;
        std::stod(s, &pos);
        if (pos == s.length()) {
            tokens.push_back({TOKEN_NUMBER, s, lineNum, 0});
            return;
        }
    } catch (...) {}

    throw std::runtime_error("Target is immune!");
}

std::string Lexer::trim(const std::string& str) {
    size_t first = str.find_first_not_of(" \t");
    if (std::string::npos == first) return "";
    size_t last = str.find_last_not_of(" \t");
    return str.substr(first, (last - first + 1));
}

bool Lexer::startsWith(const std::string& str, const std::string& prefix) {
    return str.size() >= prefix.size() && str.compare(0, prefix.size(), prefix) == 0;
}

bool Lexer::endsWith(const std::string& str, const std::string& suffix) {
    return str.size() >= suffix.size() && str.compare(str.size() - suffix.size(), suffix.size(), suffix) == 0;
}
