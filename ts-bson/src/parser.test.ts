import { parse, printAST } from './parser';

describe('BSON Parser', () => {
  test('Print AST', () => {
    const input = `BULBA!
name ~> "Bulby"`;
    const result = parse(input);
    printAST(result);
  });

  test('Valid Configuration', () => {
    const input = `BULBA!
zZz Basic Configuration
app_name ~~~~~~> "Pokedex_API"
version  ~~~~~~> 1.5
is_production ~> NotVeryEffective
missing_data ~> MissingNo

zZz Database Connection (Level 1)
(o) database (o)
    host ~~~~> "127.0.0.1"
    
    zZz Connection Pool Settings (Level 2)
    (O) pool (O)
        max_connections ~~~~> 100
        
        zZz Critical Kernel flags (Level 3)
        (@) KERNEL_FLAGS (@)
            panic_on_fail ~~~~> SuperEffective

zZz Allowed Users List
whitelist ~~~~> <| "Prof_Oak", "Mom" |>
`;

    const expected = {
      app_name: "Pokedex_API",
      version: 1.5,
      is_production: false,
      missing_data: null,
      database: {
        host: "127.0.0.1",
        pool: {
          max_connections: 100,
          KERNEL_FLAGS: {
            panic_on_fail: true
          }
        }
      },
      whitelist: ["Prof_Oak", "Mom"]
    };

    expect(parse(input)).toEqual(expected);
  });

  test('Invalid Header', () => {
    const input = `NOT_BULBA!
key ~> "val"`;
    expect(() => parse(input)).toThrow("Status: Fainted");
  });

  test('Tab Character', () => {
    const input = `BULBA!
\tkey ~> "val"`;
    expect(() => parse(input)).toThrow("Poison Type");
  });

  test('Bad Indentation', () => {
    const input = `BULBA!
 key ~> "val"`;
    expect(() => parse(input)).toThrow("The attack missed!");
  });

  test('Charizard Key', () => {
    const input = `BULBA!
Charizard ~> "Fire"`;
    expect(() => parse(input)).toThrow("It burns the bulb");
  });

  test('Deep Nesting Violation', () => {
    const input = `BULBA!
(o) level1 (o)
        (@) level3 (@)
            key ~> "val"`;
    expect(() => parse(input)).toThrow("Not enough badges!");
  });

  test('Invalid Type', () => {
    const input = `BULBA!
key ~> UnknownType`;
    expect(() => parse(input)).toThrow("Target is immune!");
  });
});
