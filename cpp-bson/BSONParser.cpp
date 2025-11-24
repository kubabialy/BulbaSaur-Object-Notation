#include "BSONParser.hpp"
#include "Lexer.hpp"
#include <iostream>
#include <algorithm>

// Error messages
const std::string ERR_SYNTAX = "It hurt itself in its confusion!";
const std::string ERR_INDENTATION = "The attack missed!";
const std::string ERR_TYPE = "Target is immune!";
const std::string ERR_BADGES = "Not enough badges!";

BSONParser::BSONParser() : currentLevel(0) {}

// parse
// The core method that orchestrates the parsing process.
// It uses a stack to manage the hierarchical structure of the BSON document.
BSONMap BSONParser::parse(const std::string& content) {
    // Step 1: Lexical Analysis
    // Delegate the tokenization to the Lexer class.
    Lexer lexer(content);
    std::vector<Token> tokens = lexer.tokenize();

    // Step 2: Parsing
    // Initialize the root map and the stack.
    auto root = std::make_shared<BSONMap>();
    stack.clear();
    stack.push_back({root, 0});
    currentLevel = 0;

    size_t i = 0;
    while (i < tokens.size()) {
        Token token = tokens[i];

        if (token.type == TOKEN_EOF) break;

        if (token.type == TOKEN_HEADER) {
            i++;
            continue;
        }

        // We look for INDENT tokens to determine structure
        if (token.type == TOKEN_INDENT) {
            Token indentToken = token;
            i++; // Consume INDENT

            if (i >= tokens.size()) break;
            Token nextToken = tokens[i];

            int expectedLevel = indentToken.level;

            // Handle Section Header (Evolution)
            if (nextToken.type == TOKEN_SECTION_OPEN) {
                int headerLevel = nextToken.level;

                // Hierarchy Check: Evolution must be sequential (1 -> 2 -> 3)
                if (expectedLevel != headerLevel - 1) {
                    throw std::runtime_error(ERR_INDENTATION);
                }
                // Ensure we have enough badges (parent sections) to evolve
                if (stack.size() < headerLevel) {
                    throw std::runtime_error(ERR_BADGES);
                }

                i++; // Consume SECTION_OPEN
                if (i >= tokens.size() || tokens[i].type != TOKEN_IDENTIFIER) {
                    throw std::runtime_error(ERR_SYNTAX);
                }
                Token keyToken = tokens[i];
                validateKey(keyToken.literal);
                i++; // Consume IDENTIFIER

                if (i >= tokens.size() || tokens[i].type != TOKEN_SECTION_CLOSE) {
                    throw std::runtime_error(ERR_SYNTAX);
                }
                i++; // Consume SECTION_CLOSE

                // Pop stack to the correct parent level
                // This handles dedenting implicitly by resizing the stack
                while (stack.size() > headerLevel) {
                    stack.pop_back();
                }

                // Create new section and add to parent
                auto newMap = std::make_shared<BSONMap>();
                BSONValue val(newMap);
                auto parentMap = stack.back().map;
                (*parentMap)[keyToken.literal] = val;

                // Push new section to stack as the current context
                stack.push_back({newMap, headerLevel});
                currentLevel = headerLevel;
                continue;
            }

            // Handle Key-Value Assignment
            if (nextToken.type == TOKEN_IDENTIFIER) {
                // Check indentation for KV
                if (expectedLevel != currentLevel) {
                    if (expectedLevel < currentLevel) {
                        // Dedent: Pop stack until we reach the expected level
                        while (stack.size() > expectedLevel + 1) {
                            stack.pop_back();
                        }
                        currentLevel = expectedLevel;
                    } else {
                        // Cannot indent deeper without a section header
                        throw std::runtime_error(ERR_INDENTATION);
                    }
                }

                Token keyToken = nextToken;
                validateKey(keyToken.literal);
                i++; // Consume IDENTIFIER

                if (i >= tokens.size() || tokens[i].type != TOKEN_VINE_WHIP) {
                    throw std::runtime_error(ERR_SYNTAX);
                }
                i++; // Consume VINE_WHIP

                // Parse Value
                BSONValue val = parseValueFromTokens(tokens, i);
                auto currentMap = stack.back().map;
                (*currentMap)[keyToken.literal] = val;
                continue;
            }

            throw std::runtime_error(ERR_SYNTAX);
        }
        i++;
    }

    return *root;
}

// parseValueFromTokens
// Helper method to parse values from the token stream.
BSONValue BSONParser::parseValueFromTokens(const std::vector<Token>& tokens, size_t& i) {
    if (i >= tokens.size()) throw std::runtime_error(ERR_SYNTAX);
    Token token = tokens[i];
    i++;

    switch (token.type) {
        case TOKEN_STRING: return BSONValue(token.literal);
        case TOKEN_NUMBER: {
            try {
                size_t pos;
                int v = std::stoi(token.literal, &pos);
                if (pos == token.literal.length()) return BSONValue(v);
            } catch (...) {}
            try {
                return BSONValue(std::stod(token.literal));
            } catch (...) {}
            throw std::runtime_error(ERR_TYPE);
        }
        case TOKEN_BOOL: return BSONValue(token.literal == "true");
        case TOKEN_NULL: return BSONValue();
        case TOKEN_ARRAY_START: {
            BSONArray arr;
            while (i < tokens.size()) {
                if (tokens[i].type == TOKEN_ARRAY_END) {
                    i++;
                    return BSONValue(arr);
                }
                if (tokens[i].type == TOKEN_COMMA) {
                    i++;
                    continue;
                }
                // Recursive call for array elements
                arr.push_back(parseValueFromTokens(tokens, i));
            }
            throw std::runtime_error(ERR_SYNTAX);
        }
        default: throw std::runtime_error(ERR_TYPE);
    }
}

void BSONParser::validateKey(const std::string& key) {
    if (key == "Charizard") throw std::runtime_error("It burns the bulb");
}


// Implementation of print methods

void printAST(const BSONMap& map) {
    for (const auto& pair : map) {
        std::cout << pair.first << ": ";
        if (pair.second.type == BSONValue::OBJECT || pair.second.type == BSONValue::ARRAY) {
            std::cout << std::endl;
            pair.second.print(1);
        } else {
            pair.second.print(0);
        }
    }
}

void BSONValue::print(int indent) const {
    std::string indentation(indent * 2, ' ');
    
    std::visit([&](auto&& arg) {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, std::string>) {
            if (indent == 0) std::cout << arg << std::endl;
            else std::cout << indentation << arg << std::endl;
        } else if constexpr (std::is_same_v<T, int> || std::is_same_v<T, double>) {
            if (indent == 0) std::cout << arg << std::endl;
            else std::cout << indentation << arg << std::endl;
        } else if constexpr (std::is_same_v<T, bool>) {
            if (indent == 0) std::cout << (arg ? "true" : "false") << std::endl;
            else std::cout << indentation << (arg ? "true" : "false") << std::endl;
        } else if constexpr (std::is_same_v<T, std::monostate>) {
            if (indent == 0) std::cout << "null" << std::endl;
            else std::cout << indentation << "null" << std::endl;
        } else if constexpr (std::is_same_v<T, BSONArray>) {
            for (const auto& val : arg) {
                std::cout << indentation << "- ";
                if (val.type == OBJECT || val.type == ARRAY) {
                    std::cout << std::endl;
                    val.print(indent + 1);
                } else {
                    val.print(0);
                }
            }
        } else if constexpr (std::is_same_v<T, std::shared_ptr<BSONMap>>) {
            for (const auto& pair : *arg) {
                std::cout << indentation << pair.first << ": ";
                if (pair.second.type == OBJECT || pair.second.type == ARRAY) {
                    std::cout << std::endl;
                    pair.second.print(indent + 1);
                } else {
                    pair.second.print(0);
                }
            }
        }
    }, value);
}
