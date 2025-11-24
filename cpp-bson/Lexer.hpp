#pragma once
#include <string>
#include <vector>

// TokenType Enum
// Defines all possible tokens in the BSON language.
// Using an enum ensures type safety and readability throughout the lexer and parser.
enum TokenType {
    TOKEN_HEADER,         // The "BULBA!" header
    TOKEN_INDENT,         // Indentation (4 spaces)
    TOKEN_SECTION_OPEN,   // (o), (O), (@)
    TOKEN_SECTION_CLOSE,  // (o), (O), (@)
    TOKEN_IDENTIFIER,     // Keys
    TOKEN_VINE_WHIP,      // Assignment operator ~~~~>
    TOKEN_STRING,         // "value"
    TOKEN_NUMBER,         // 123, 4.5
    TOKEN_BOOL,           // SuperEffective, NotVeryEffective
    TOKEN_NULL,           // MissingNo
    TOKEN_ARRAY_START,    // <|
    TOKEN_ARRAY_END,      // |>
    TOKEN_COMMA,          // ,
    TOKEN_EOF             // End of File
};

// Token Structure
// Represents a single unit of meaning in the source code.
struct Token {
    TokenType type;
    std::string literal; // The actual text content
    int line;            // Line number for error reporting
    int level;           // For INDENT and SECTION tokens, stores the nesting level
};

// Lexer Class
// Responsible for converting raw source code into a stream of tokens.
// Encapsulates the lexical analysis logic, hiding the complexity of string parsing.
class Lexer {
public:
    Lexer(const std::string& content);
    
    // Main method to generate tokens
    std::vector<Token> tokenize();

private:
    std::string content;
    std::vector<Token> tokens;
    
    // Helper methods for internal logic
    void tokenizeLine(const std::string& line, int lineNum);
    void tokenizeValue(const std::string& valStr, int lineNum);
    std::string trim(const std::string& str);
    bool startsWith(const std::string& str, const std::string& prefix);
    bool endsWith(const std::string& str, const std::string& suffix);
};
