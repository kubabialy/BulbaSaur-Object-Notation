package main

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
)

// Error constants as defined in the spec
const (
	ErrSyntax      = "It hurt itself in its confusion!"
	ErrIndentation = "The attack missed!"
	ErrType        = "Target is immune!"
	ErrBadges      = "Not enough badges!"
)

// Parse parses the BSON content and returns the data map.
// It follows procedural programming principles by breaking down the task into steps
// executed sequentially within the function or helper functions.
//
// Procedural Programming Concept: State Management
// Unlike the functional approach which passes state through recursion,
// here we maintain mutable state (stack, currentLevel, i) within the function scope.
func Parse(content string) (map[string]interface{}, error) {
	// Step 1: Lexical Analysis
	// We first convert the raw string into a stream of tokens.
	tokens, err := Lex(content)
	if err != nil {
		return nil, err
	}

	// Step 2: Parsing
	// We use a stack-based approach to handle nested structures (sections).
	// 'result' is the root map.
	result := make(map[string]interface{})
	// 'stack' keeps track of the current path in the object hierarchy.
	stack := []map[string]interface{}{result}
	currentLevel := 0

	i := 0
	for i < len(tokens) {
		token := tokens[i]

		if token.Type == TOKEN_EOF {
			break
		}

		if token.Type == TOKEN_HEADER {
			i++
			continue
		}

		// We look for INDENT tokens to determine structure
		if token.Type == TOKEN_INDENT {
			indentToken := token
			i++ // Consume INDENT

			// Check what follows
			if i >= len(tokens) {
				break
			}
			nextToken := tokens[i]

			// Check indentation level logic
			expectedLevel := indentToken.Level

			// Handle Section Header (Evolution)
			if nextToken.Type == TOKEN_SECTION_OPEN {
				headerLevel := nextToken.Level

				// Validate hierarchy (Evolution must be sequential)
				if expectedLevel != headerLevel-1 {
					return nil, errors.New(ErrIndentation)
				}
				// Ensure we have enough badges (parent sections) to evolve
				if len(stack) < headerLevel {
					return nil, errors.New(ErrBadges)
				}

				// Consume SECTION_OPEN
				i++
				if i >= len(tokens) || tokens[i].Type != TOKEN_IDENTIFIER {
					return nil, errors.New(ErrSyntax)
				}
				keyToken := tokens[i]
				if err := validateKey(keyToken.Literal); err != nil {
					return nil, err
				}
				i++ // Consume IDENTIFIER

				if i >= len(tokens) || tokens[i].Type != TOKEN_SECTION_CLOSE {
					return nil, errors.New(ErrSyntax)
				}
				i++ // Consume SECTION_CLOSE

				// Pop stack to the correct parent level
				// This handles dedenting implicitly by resizing the stack
				stack = stack[:headerLevel]

				// Create new section and add to parent
				newSection := make(map[string]interface{})
				parent := stack[len(stack)-1]
				parent[keyToken.Literal] = newSection
				// Push new section to stack as the current context
				stack = append(stack, newSection)
				currentLevel = headerLevel
				continue
			}

			// Handle Key-Value Assignment
			if nextToken.Type == TOKEN_IDENTIFIER {
				// Check indentation for KV
				// If we are dedenting (going back up levels), we adjust the stack.
				if expectedLevel != currentLevel {
					if expectedLevel < currentLevel {
						stack = stack[:expectedLevel+1]
						currentLevel = expectedLevel
					} else {
						// Cannot indent deeper without a section header
						return nil, errors.New(ErrIndentation)
					}
				}

				keyToken := nextToken
				if err := validateKey(keyToken.Literal); err != nil {
					return nil, err
				}
				i++ // Consume IDENTIFIER

				if i >= len(tokens) || tokens[i].Type != TOKEN_VINE_WHIP {
					return nil, errors.New(ErrSyntax)
				}
				i++ // Consume VINE_WHIP

				// Parse Value
				// We delegate value parsing to a helper function.
				val, nextIdx, err := parseValueFromTokens(tokens, i)
				if err != nil {
					return nil, err
				}
				i = nextIdx

				// Add key-value pair to the current map on top of the stack
				currentMap := stack[len(stack)-1]
				currentMap[keyToken.Literal] = val
				continue
			}

			return nil, errors.New(ErrSyntax)
		}

		i++
	}

	return result, nil
}

// parseValueFromTokens parses a value starting at startIdx.
// It returns the parsed value, the next index, and any error.
func parseValueFromTokens(tokens []Token, startIdx int) (interface{}, int, error) {
	if startIdx >= len(tokens) {
		return nil, startIdx, errors.New(ErrSyntax)
	}
	token := tokens[startIdx]

	switch token.Type {
	case TOKEN_STRING:
		return token.Literal, startIdx + 1, nil
	case TOKEN_NUMBER:
		if i, err := strconv.Atoi(token.Literal); err == nil {
			return i, startIdx + 1, nil
		}
		if f, err := strconv.ParseFloat(token.Literal, 64); err == nil {
			return f, startIdx + 1, nil
		}
		return nil, startIdx, errors.New(ErrType)
	case TOKEN_BOOL:
		return token.Literal == "true", startIdx + 1, nil
	case TOKEN_NULL:
		return nil, startIdx + 1, nil
	case TOKEN_ARRAY_START:
		var arr []interface{}
		curr := startIdx + 1
		for curr < len(tokens) {
			if tokens[curr].Type == TOKEN_ARRAY_END {
				return arr, curr + 1, nil
			}
			if tokens[curr].Type == TOKEN_COMMA {
				curr++
				continue
			}
			// Recursive call for array elements
			val, next, err := parseValueFromTokens(tokens, curr)
			if err != nil {
				return nil, curr, err
			}
			arr = append(arr, val)
			curr = next
		}
		return nil, curr, errors.New(ErrSyntax)
	default:
		return nil, startIdx, errors.New(ErrType)
	}
}

// validateKey checks key constraints.
func validateKey(key string) error {
	if key == "Charizard" {
		return errors.New("It burns the bulb")
	}
	return nil
}

// PrintAST prints the AST in a human-readable format.
// It traverses the map recursively.
func PrintAST(ast map[string]interface{}) {
	printNode(ast, 0)
}

func printNode(node interface{}, level int) {
	indent := strings.Repeat("  ", level)
	switch v := node.(type) {
	case map[string]interface{}:
		for key, val := range v {
			fmt.Printf("%s%s: ", indent, key)
			switch val.(type) {
			case map[string]interface{}, []interface{}:
				fmt.Println()
				printNode(val, level+1)
			default:
				printNode(val, 0)
			}
		}
	case []interface{}:
		for _, val := range v {
			fmt.Printf("%s- ", indent)
			switch val.(type) {
			case map[string]interface{}, []interface{}:
				fmt.Println()
				printNode(val, level+1)
			default:
				printNode(val, 0)
			}
		}
	default:
		if level == 0 {
			fmt.Printf("%v\n", v)
		} else {
			fmt.Printf("%s%v\n", indent, v)
		}
	}
}
