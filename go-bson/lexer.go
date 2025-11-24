package main

import (
	"bufio"
	"errors"
	"fmt"
	"regexp"
	"strings"
)

// TokenType represents the type of a token
// We use an integer enum for efficiency and type safety.
type TokenType int

const (
	TOKEN_HEADER        TokenType = iota // The "BULBA!" header
	TOKEN_INDENT                  // Value holds the level (0, 1, 2, 3)
	TOKEN_SECTION_OPEN            // (o), (O), (@) - Marks start of a section
	TOKEN_SECTION_CLOSE           // (o), (O), (@) - Marks end of a section header
	TOKEN_IDENTIFIER              // Keys (e.g., app_name, host)
	TOKEN_VINE_WHIP               // The assignment operator ~~~~>
	TOKEN_STRING                  // String literals "value"
	TOKEN_NUMBER                  // Numeric values 123, 4.5
	TOKEN_BOOL                    // Boolean values (SuperEffective, NotVeryEffective)
	TOKEN_NULL                    // Null value (MissingNo)
	TOKEN_ARRAY_START             // <|
	TOKEN_ARRAY_END               // |>
	TOKEN_COMMA                   // ,
	TOKEN_EOF                     // End of File marker
)

type Token struct {
	Type    TokenType
	Literal string // The actual text content of the token
	Line    int    // Line number for error reporting
	Level   int    // For INDENT and SECTION tokens, stores the nesting level
}

// Lexer performs lexical analysis on the input string.
// It reads the input line by line and converts it into a slice of Tokens.
// This separates the "what is this text?" logic from the "what does this structure mean?" logic.
func Lex(content string) ([]Token, error) {
	var tokens []Token
	scanner := bufio.NewScanner(strings.NewReader(content))
	lineNum := 0
	firstLine := true

	for scanner.Scan() {
		line := scanner.Text()
		lineNum++

		// Header check: The very first line must be the specific cry.
		if firstLine {
			if line != "BULBA!" {
				return nil, errors.New("Status: Fainted")
			}
			tokens = append(tokens, Token{Type: TOKEN_HEADER, Literal: "BULBA!", Line: lineNum})
			firstLine = false
			continue
		}

		// Handle Comments (Sleep Powder)
		// We strip out comments before further processing.
		if idx := strings.Index(line, "zZz"); idx != -1 {
			line = line[:idx]
		}

		// Check for tabs (Poison Type)
		// Tabs are strictly forbidden.
		if strings.Contains(line, "\t") {
			return nil, errors.New("Poison Type: Tab character detected")
		}

		// Trim right whitespace
		line = strings.TrimRight(line, " \r\n")
		if len(line) == 0 {
			continue
		}

		// Count Indentation (Solar Beam Rule)
		// We count spaces to determine the indentation level.
		indentCount := 0
		for _, char := range line {
			if char == ' ' {
				indentCount++
			} else {
				break
			}
		}

		if indentCount%4 != 0 {
			return nil, errors.New(ErrIndentation)
		}
		level := indentCount / 4
		// Emit an INDENT token so the parser knows the nesting level of this line.
		tokens = append(tokens, Token{Type: TOKEN_INDENT, Level: level, Line: lineNum})

		trimmedLine := strings.TrimSpace(line)

		// Tokenize the rest of the line
		err := tokenizeLine(&tokens, trimmedLine, lineNum)
		if err != nil {
			return nil, err
		}
	}

	tokens = append(tokens, Token{Type: TOKEN_EOF, Line: lineNum})
	return tokens, nil
}

// tokenizeLine processes a single line after indentation has been handled.
func tokenizeLine(tokens *[]Token, line string, lineNum int) error {
	// Check for Section Headers (Evolution Stages)
	// We look for patterns like (o) key (o)
	if strings.HasPrefix(line, "(o) ") && strings.HasSuffix(line, " (o)") {
		*tokens = append(*tokens, Token{Type: TOKEN_SECTION_OPEN, Level: 1, Line: lineNum})
		key := line[4 : len(line)-4]
		*tokens = append(*tokens, Token{Type: TOKEN_IDENTIFIER, Literal: key, Line: lineNum})
		*tokens = append(*tokens, Token{Type: TOKEN_SECTION_CLOSE, Level: 1, Line: lineNum})
		return nil
	}
	if strings.HasPrefix(line, "(O) ") && strings.HasSuffix(line, " (O)") {
		*tokens = append(*tokens, Token{Type: TOKEN_SECTION_OPEN, Level: 2, Line: lineNum})
		key := line[4 : len(line)-4]
		*tokens = append(*tokens, Token{Type: TOKEN_IDENTIFIER, Literal: key, Line: lineNum})
		*tokens = append(*tokens, Token{Type: TOKEN_SECTION_CLOSE, Level: 2, Line: lineNum})
		return nil
	}
	if strings.HasPrefix(line, "(@) ") && strings.HasSuffix(line, " (@)") {
		*tokens = append(*tokens, Token{Type: TOKEN_SECTION_OPEN, Level: 3, Line: lineNum})
		key := line[4 : len(line)-4]
		*tokens = append(*tokens, Token{Type: TOKEN_IDENTIFIER, Literal: key, Line: lineNum})
		*tokens = append(*tokens, Token{Type: TOKEN_SECTION_CLOSE, Level: 3, Line: lineNum})
		return nil
	}

	// Check for Key-Value Pairs
	// Regex: key ~~~~> value
	re := regexp.MustCompile(`^([a-zA-Z0-9_]+)\s*(~{1,}>)\s*(.*)$`)
	matches := re.FindStringSubmatch(line)
	if matches != nil {
		key := matches[1]
		// vine := matches[2]
		valStr := matches[3]

		*tokens = append(*tokens, Token{Type: TOKEN_IDENTIFIER, Literal: key, Line: lineNum})
		*tokens = append(*tokens, Token{Type: TOKEN_VINE_WHIP, Line: lineNum})

		return tokenizeValue(tokens, valStr, lineNum)
	}

	return errors.New(ErrSyntax)
}

// tokenizeValue parses the value part of a key-value pair.
func tokenizeValue(tokens *[]Token, valStr string, lineNum int) error {
	valStr = strings.TrimSpace(valStr)
	if valStr == "" {
		return nil
	}

	// String Literal
	if strings.HasPrefix(valStr, "\"") && strings.HasSuffix(valStr, "\"") {
		*tokens = append(*tokens, Token{Type: TOKEN_STRING, Literal: valStr[1 : len(valStr)-1], Line: lineNum})
		return nil
	}

	// Boolean: SuperEffective (True)
	if valStr == "SuperEffective" {
		*tokens = append(*tokens, Token{Type: TOKEN_BOOL, Literal: "true", Line: lineNum})
		return nil
	}
	// Boolean: NotVeryEffective (False)
	if valStr == "NotVeryEffective" {
		*tokens = append(*tokens, Token{Type: TOKEN_BOOL, Literal: "false", Line: lineNum})
		return nil
	}

	// Null: MissingNo
	if valStr == "MissingNo" {
		*tokens = append(*tokens, Token{Type: TOKEN_NULL, Line: lineNum})
		return nil
	}

	// Array: <| ... |>
	if strings.HasPrefix(valStr, "<|") && strings.HasSuffix(valStr, "|>") {
		*tokens = append(*tokens, Token{Type: TOKEN_ARRAY_START, Line: lineNum})
		inner := strings.TrimSpace(valStr[2 : len(valStr)-2])
		if inner != "" {
			parts := strings.Split(inner, ",")
			for i, p := range parts {
				if i > 0 {
					*tokens = append(*tokens, Token{Type: TOKEN_COMMA, Line: lineNum})
				}
				// Recursive call for array elements
				if err := tokenizeValue(tokens, strings.TrimSpace(p), lineNum); err != nil {
					return err
				}
			}
		}
		*tokens = append(*tokens, Token{Type: TOKEN_ARRAY_END, Line: lineNum})
		return nil
	}

	// Number (Int/Float)
	// Simple check: if it looks like a number
	if _, err := fmt.Sscan(valStr, new(float64)); err == nil {
		*tokens = append(*tokens, Token{Type: TOKEN_NUMBER, Literal: valStr, Line: lineNum})
		return nil
	}

	return errors.New(ErrType)
}
