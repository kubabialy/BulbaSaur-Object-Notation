// Token definitions
// We define an enum for all possible token types in our language.
// This helps in type safety and makes the code more readable.
export enum TokenType {
  HEADER,         // The "BULBA!" header
  INDENT,         // Indentation (4 spaces)
  SECTION_OPEN,   // (o), (O), (@)
  SECTION_CLOSE,  // (o), (O), (@)
  IDENTIFIER,     // Keys
  VINE_WHIP,      // Assignment operator ~~~~>
  STRING,         // "value"
  NUMBER,         // 123, 4.5
  BOOL,           // SuperEffective, NotVeryEffective
  NULL,           // MissingNo
  ARRAY_START,    // <|
  ARRAY_END,      // |>
  COMMA,          // ,
  EOF             // End of File
}

export interface Token {
  type: TokenType;
  literal: string; // The actual text content of the token
  line: number;    // Line number for error reporting
  level: number;   // For INDENT and SECTION tokens, stores the nesting level
}

// Error constants
const ERR_INDENTATION = "The attack missed!";

// Lexer Function
// The goal of the lexer is to convert a raw string of code into a stream of tokens.
// It handles low-level details like whitespace, comments, and character matching.
// This separation of concerns simplifies the parser, which can focus on structure.
//
// Functional Programming Principles:
// - Pure Functions: tokenize, tokenizeLine, tokenizeBody, tokenizeValue are pure.
// - Immutability: We use flatMap to create the token array instead of pushing to a mutable array.
// - No Side Effects: No global state or console logs.
export const tokenize = (content: string): Token[] => {
  const lines = content.split('\n');

  const tokens = lines.flatMap((line, index) => {
    const lineNum = index + 1;

    // Header check: The very first line must be the specific cry.
    if (index === 0) {
      if (line.trim() !== 'BULBA!') {
        throw new Error("Status: Fainted");
      }
      return [{ type: TokenType.HEADER, literal: 'BULBA!', line: lineNum, level: 0 }];
    }

    return tokenizeLine(line, lineNum);
  });

  return [...tokens, { type: TokenType.EOF, literal: '', line: lines.length + 1, level: 0 }];
};

// Helper to tokenize a single line
// Returns an array of tokens for that line (or empty array if line is empty/comment)
const tokenizeLine = (line: string, lineNum: number): Token[] => {
  // Handle comments (Sleep Powder)
  const commentIdx = line.indexOf('zZz');
  const lineWithoutComment = commentIdx !== -1 ? line.substring(0, commentIdx) : line;

  // Trim right whitespace
  const cleanLine = lineWithoutComment.replace(/\s+$/, '');

  if (cleanLine.length === 0) return [];

  // Check indentation (Solar Beam Rule)
  const indentCount = countIndent(cleanLine);
  if (indentCount % 4 !== 0) {
    throw new Error(ERR_INDENTATION);
  }

  const level = indentCount / 4;
  const indentToken: Token = { type: TokenType.INDENT, literal: '', line: lineNum, level };

  const trimmedBody = cleanLine.trim();
  const bodyTokens = tokenizeBody(trimmedBody, lineNum, level);

  return [indentToken, ...bodyTokens];
};

const countIndent = (line: string): number => {
  let count = 0;
  for (const char of line) {
    if (char === ' ') count++;
    else if (char === '\t') throw new Error("Poison Type: Tab character detected");
    else break;
  }
  return count;
};

const tokenizeBody = (line: string, lineNum: number, level: number): Token[] => {
  // Check for Section Headers
  if (line.startsWith('(o) ') && line.endsWith(' (o)')) {
    return [
      { type: TokenType.SECTION_OPEN, literal: '', line: lineNum, level: 1 },
      { type: TokenType.IDENTIFIER, literal: line.substring(4, line.length - 4), line: lineNum, level: 0 },
      { type: TokenType.SECTION_CLOSE, literal: '', line: lineNum, level: 1 }
    ];
  }
  if (line.startsWith('(O) ') && line.endsWith(' (O)')) {
    return [
      { type: TokenType.SECTION_OPEN, literal: '', line: lineNum, level: 2 },
      { type: TokenType.IDENTIFIER, literal: line.substring(4, line.length - 4), line: lineNum, level: 0 },
      { type: TokenType.SECTION_CLOSE, literal: '', line: lineNum, level: 2 }
    ];
  }
  if (line.startsWith('(@) ') && line.endsWith(' (@)')) {
    return [
      { type: TokenType.SECTION_OPEN, literal: '', line: lineNum, level: 3 },
      { type: TokenType.IDENTIFIER, literal: line.substring(4, line.length - 4), line: lineNum, level: 0 },
      { type: TokenType.SECTION_CLOSE, literal: '', line: lineNum, level: 3 }
    ];
  }

  // Check for Key-Value Pairs
  const match = line.match(/^([a-zA-Z0-9_]+)\s*(~{1,}>)\s*(.*)$/);
  if (match) {
    const keyToken: Token = { type: TokenType.IDENTIFIER, literal: match[1], line: lineNum, level: 0 };
    const vineToken: Token = { type: TokenType.VINE_WHIP, literal: '', line: lineNum, level: 0 };
    const valueTokens = tokenizeValue(match[3], lineNum);
    return [keyToken, vineToken, ...valueTokens];
  }

  throw new Error("It hurt itself in its confusion!");
};

// Helper to tokenize the value part
const tokenizeValue = (valStr: string, lineNum: number): Token[] => {
  valStr = valStr.trim();
  if (valStr === '') return [];

  if (valStr.startsWith('"') && valStr.endsWith('"')) {
    return [{ type: TokenType.STRING, literal: valStr.substring(1, valStr.length - 1), line: lineNum, level: 0 }];
  }
  if (valStr === 'SuperEffective') {
    return [{ type: TokenType.BOOL, literal: 'true', line: lineNum, level: 0 }];
  }
  if (valStr === 'NotVeryEffective') {
    return [{ type: TokenType.BOOL, literal: 'false', line: lineNum, level: 0 }];
  }
  if (valStr === 'MissingNo') {
    return [{ type: TokenType.NULL, literal: '', line: lineNum, level: 0 }];
  }
  if (valStr.startsWith('<|') && valStr.endsWith('|>')) {
    const startToken: Token = { type: TokenType.ARRAY_START, literal: '', line: lineNum, level: 0 };
    const endToken: Token = { type: TokenType.ARRAY_END, literal: '', line: lineNum, level: 0 };
    
    const inner = valStr.substring(2, valStr.length - 2).trim();
    if (inner === '') return [startToken, endToken];

    const parts = inner.split(',');
    // Map parts to tokens and intersperse commas
    const innerTokens = parts.flatMap((p, i) => {
      const valTokens = tokenizeValue(p, lineNum);
      if (i > 0) {
        return [{ type: TokenType.COMMA, literal: '', line: lineNum, level: 0 }, ...valTokens];
      }
      return valTokens;
    });

    return [startToken, ...innerTokens, endToken];
  }

  if (!isNaN(Number(valStr))) {
    return [{ type: TokenType.NUMBER, literal: valStr, line: lineNum, level: 0 }];
  }

  throw new Error("Target is immune!");
};
