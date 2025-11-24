import { tokenize, Token, TokenType } from "./lexer";

// Types
export type BSONValue =
  | string
  | number
  | boolean
  | null
  | BSONValue[]
  | BSONMap;
export interface BSONMap {
  [key: string]: BSONValue;
}

// Error constants
const ERR_SYNTAX = "It hurt itself in its confusion!";
const ERR_INDENTATION = "The attack missed!";
const ERR_TYPE = "Target is immune!";
const ERR_BADGES = "Not enough badges!";

interface Cursor {
  tokens: Token[];
  index: number;
}

// Main parse function
// This function orchestrates the parsing process.
// It follows functional principles by delegating state management to the recursive `parseBlock` function.
export const parse = (content: string): BSONMap => {
  // Step 1: Lexical Analysis
  const tokens = tokenize(content);
  const cursor: Cursor = { tokens, index: 0 };

  // Step 2: Check Header (The Cry)
  if (cursor.index < cursor.tokens.length) {
    const token = cursor.tokens[cursor.index];
    if (token.type !== TokenType.HEADER) {
      // Should be caught by lexer but just in case
      throw new Error("Status: Fainted");
    }
    cursor.index++;
  }

  // Step 3: Parse the content recursively
  const [result] = parseBlock(cursor, 0);
  return result;
};

// Recursive block parser
// This function parses a block of lines at a specific indentation level.
// It returns the map for the current block and the updated cursor.
// This avoids mutable global state by passing the cursor through the recursion.
//
// Functional Programming Concept: Recursion
// Instead of a loop with a mutable stack, we use recursion to handle nested structures.
// Each call to parseBlock handles one level of nesting.
const parseBlock = (
  cursor: Cursor,
  currentLevel: number,
): [BSONMap, Cursor] => {
  // Base case: End of tokens
  if (cursor.index >= cursor.tokens.length) {
    return [{}, cursor];
  }

  const token = cursor.tokens[cursor.index];

  // Base case: EOF
  if (token.type === TokenType.EOF) {
    return [{}, cursor];
  }

  // We expect an INDENT token to start a line
  if (token.type === TokenType.INDENT) {
    const expectedLevel = token.level;

    // Base case: Dedent (End of Block)
    // If the indentation level decreases, it means we've finished the current block
    // and should return to the parent caller.
    if (expectedLevel < currentLevel) {
      return [{}, cursor];
    }

    // Error case: Indent mismatch
    // If indentation is deeper than expected without a new section header, it's an error.
    if (expectedLevel > currentLevel) {
      // Check if it's a header trying to skip levels (Evolution violation)
      const nextIndex = cursor.index + 1;
      if (nextIndex < cursor.tokens.length) {
        const nextToken = cursor.tokens[nextIndex];
        if (nextToken.type === TokenType.SECTION_OPEN) {
          if (expectedLevel === nextToken.level - 1) {
            // Correct indentation for the header, but we are at wrong level
            throw new Error(ERR_BADGES);
          }
        }
      }
      throw new Error(ERR_INDENTATION);
    }

    // Indent matches currentLevel. We have a statement to parse.
    // Parse the current statement (KV or Section)
    const [key, value, nextCursor] = parseStatement(cursor, currentLevel);

    // Recursive step: Parse the rest of the block (siblings)
    const [siblings, finalCursor] = parseBlock(nextCursor, currentLevel);

    // Combine the current statement with the siblings
    return [{ [key]: value, ...siblings }, finalCursor];
  }

  // Should not happen if lexer works correctly (INDENT is always first on line)
  // Skip token to avoid infinite loop if lexer is broken
  return parseBlock({ ...cursor, index: cursor.index + 1 }, currentLevel);
};

// Helper to parse a single statement (Section or Key-Value)
// Returns [Key, Value, NextCursor]
const parseStatement = (cursor: Cursor, currentLevel: number): [string, BSONValue, Cursor] => {
  // We are at INDENT. Look ahead.
  const nextIndex = cursor.index + 1;
  if (nextIndex >= cursor.tokens.length) throw new Error(ERR_SYNTAX);
  
  const nextToken = cursor.tokens[nextIndex];

  // Case 1: Section Header (Evolution)
  if (nextToken.type === TokenType.SECTION_OPEN) {
    const headerLevel = nextToken.level;

    // Hierarchy Check: Evolution must be sequential (1 -> 2 -> 3)
    if (headerLevel !== currentLevel + 1) {
      throw new Error(ERR_BADGES);
    }

    // Consume INDENT, SECTION_OPEN
    let idx = cursor.index + 2;

    // Expect IDENTIFIER (The name of the section)
    if (idx >= cursor.tokens.length || cursor.tokens[idx].type !== TokenType.IDENTIFIER) {
      throw new Error(ERR_SYNTAX);
    }
    const keyToken = cursor.tokens[idx];
    validateKey(keyToken.literal);
    idx++;

    // Expect SECTION_CLOSE
    if (idx >= cursor.tokens.length || cursor.tokens[idx].type !== TokenType.SECTION_CLOSE) {
      throw new Error(ERR_SYNTAX);
    }
    idx++;

    // Recursion Step: Parse the child block
    const [childMap, afterChildCursor] = parseBlock({ ...cursor, index: idx }, headerLevel);
    
    return [keyToken.literal, childMap, afterChildCursor];
  }

  // Case 2: Key-Value Assignment
  if (nextToken.type === TokenType.IDENTIFIER) {
    // Consume INDENT
    let idx = cursor.index + 1;

    const keyToken = cursor.tokens[idx];
    validateKey(keyToken.literal);
    idx++;

    // Expect VINE_WHIP (Assignment Operator)
    if (idx >= cursor.tokens.length || cursor.tokens[idx].type !== TokenType.VINE_WHIP) {
      throw new Error(ERR_SYNTAX);
    }
    idx++;

    // Parse Value
    const [val, newIndex] = parseValueFromTokens(cursor.tokens, idx);
    
    return [keyToken.literal, val, { ...cursor, index: newIndex }];
  }

  throw new Error(ERR_SYNTAX);
};

// Helper to parse values from the token stream
// Returns the parsed value and the new index in the token stream.
const parseValueFromTokens = (
  tokens: Token[],
  index: number,
): [BSONValue, number] => {
  if (index >= tokens.length) throw new Error(ERR_SYNTAX);
  const token = tokens[index];

  switch (token.type) {
    case TokenType.STRING:
      return [token.literal, index + 1];
    case TokenType.NUMBER:
      return [Number(token.literal), index + 1];
    case TokenType.BOOL:
      return [token.literal === "true", index + 1];
    case TokenType.NULL:
      return [null, index + 1];
    case TokenType.ARRAY_START:
      const arr: BSONValue[] = [];
      let curr = index + 1;
      while (curr < tokens.length) {
        if (tokens[curr].type === TokenType.ARRAY_END) {
          return [arr, curr + 1];
        }
        if (tokens[curr].type === TokenType.COMMA) {
          curr++;
          continue;
        }
        // Recursive call for array elements
        const [val, next] = parseValueFromTokens(tokens, curr);
        arr.push(val);
        curr = next;
      }
      throw new Error(ERR_SYNTAX);
    default:
      throw new Error(ERR_TYPE);
  }
};

const validateKey = (key: string): void => {
  if (key === "Charizard") throw new Error("It burns the bulb");
};

// Print AST function
// Recursively prints the AST structure
export const printAST = (ast: BSONMap): void => {
  printNode(ast, 0);
};

const printNode = (node: BSONValue, level: number): void => {
  const indent = "  ".repeat(level);
  
  if (node === null) {
    console.log(level === 0 ? "null" : `${indent}null`);
    return;
  }

  if (typeof node === "object") {
    if (Array.isArray(node)) {
      node.forEach((val) => {
        process.stdout.write(`${indent}- `);
        if (typeof val === "object" && val !== null) {
          console.log();
          printNode(val, level + 1);
        } else {
          printNode(val, 0);
        }
      });
    } else {
      // Map
      Object.entries(node).forEach(([key, val]) => {
        process.stdout.write(`${indent}${key}: `);
        if (typeof val === "object" && val !== null) {
          console.log();
          printNode(val, level + 1);
        } else {
          printNode(val, 0);
        }
      });
    }
  } else {
    // Primitive
    console.log(level === 0 ? String(node) : `${indent}${node}`);
  }
};
