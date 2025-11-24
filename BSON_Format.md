# BSON Specification (The "Green Paper")

**Version:** 0.0.1 (Seedling)
**Status:** Experimental / Organic
**MIME Type:** `application/x-vine-whip`
**File Extension:** `.001`

---

## 1. Introduction
Bulbasaur (BSON) is a configuration format designed to prioritize organic growth over machine efficiency. It rejects the industrial rigidity of JSON and YAML in favor of a syntax that mimics the lifecycle of a Grass-type Pokémon.

### 1.1 Encoding
All BSON files must be encoded in **UTF-8**.
Any other encoding is considered "Confused" (the parser will hurt itself in its confusion).

---

## 2. File Structure

### 2.1 The Cry (Header)
A valid BSON file is a living thing. It must announce itself.
The **very first line** of the file must be exactly:

```text
BULBA!
```

* **Constraint:** If the file begins with whitespace, comments, or any other character, the parser returns `Status: Fainted`.

### 2.2 Whitespace (The Solar Beam Rule)
Indentation represents sunlight absorption. Consistency is key for photosynthesis.

* **Unit:** Indentation must be composed of **Space characters (U+0020)**.
* **Quantity:** Each indentation level is exactly **4 spaces**.
* **Tab Prohibition:** The Tab character (`\t`) is **Poison Type**. If a parser encounters a tab, it must immediately terminate execution to prevent the poisoning from spreading to the CPU.
* **Line Endings:** Must be `\n` (LF). `\r\n` (CRLF) is considered "Industrial Waste" and should be stripped.

---

## 3. Comments (Sleep Powder)
Comments represent parts of the file that are currently napping. The parser tiptoes past them.

* **Syntax:** `zZz` (Case sensitive).
* **Behavior:** Anything following `zZz` until the end of the line is ignored.
* **Placement:** Can be on a new line or inline (after a value).

```text
zZz This is a comment
host ~~~~> "localhost" zZz Inline napping
```

---

## 4. Key-Value Assignment

### 4.1 Keys
Keys are case-sensitive identifiers.

* **Allowed:** Alphanumeric characters and underscores.
* **Restricted Keyword:** You may not use the word `Charizard` as a key. It burns the bulb.

### 4.2 The Vine Whip (Assignment Operator)
Values are assigned using a vine.

* **Syntax:** A tilde `~` repeated at least 2 times, followed by a greater-than sign `>`.
* **Regex:** `~{2,}>`
* **Semantics:** The length of the vine is visually significant but functionally identical.
    * `~>` : Short-range whip (High priority).
    * `~~~~~~~~~>` : Long-range whip (Lazy loaded, conceptually).

---

## 5. Data Types

### 5.1 Strings
Double-quoted Unicode text.

```text
name ~~~~> "Ash Ketchum"
```

### 5.2 Numbers (HP/Stats)
Standard Integers and Floats.

```text
level ~~~~> 5
win_rate ~~> 45.5
```

### 5.3 Booleans (Type Effectiveness)
Standard `true`/`false` logic is replaced by type matchups.

* **True:** `SuperEffective`
* **False:** `NotVeryEffective`

### 5.4 Null
If a value is missing or unknown, it is a glitch.

* **Value:** `MissingNo`

### 5.5 Arrays (Razor Leaf)
Lists are encapsulated in sharp leaves.

* **Delimiters:** `<|` and `|>`
* **Separators:** Commas `,`

```text
items ~~~~> <| "Potion", "Antidote", "Town Map" |>
```

---

## 6. Hierarchy (Evolution)

BSON does not use arbitrary nesting. It uses **Evolutionary Stages**. You cannot define a Stage 3 section without a Stage 2 parent.

### 6.1 Seed / Root
Top-level keys defined outside of any bulb are "Seed Level."

### 6.2 Level 1: The Bulbasaur Bulb `(o)`
The first level of nesting.

```text
(o) network (o)
    port ~~~~> 8080
```

### 6.3 Level 2: The Ivysaur Bulb `(O)`
Must be nested inside a Level 1 Bulb.

```text
(o) network (o)
    (O) security (O)
        ssl ~~~~> SuperEffective
```

### 6.4 Level 3: The Venusaur Bulb `(@)`
The final stage of evolution. Must be nested inside a Level 2 Bulb.

```text
(o) core (o)
    (O) kernel (O)
        (@) PROCESS_SCHEDULER (@)
            priority ~~~~> "High"
```

* **Constraint:** You cannot go deeper than Level 3. If you need Level 4 nesting, your code is too complex and you should refactor (or use a Mega Evolution Stone, which is not supported in v0.0.1).

---

## 7. Example Reference Document

```text
BULBA!

zZz Basic Configuration
app_name ~~~~~~> "Pokedex_API"
version  ~~~~~~> 1.5
is_production ~> NotVeryEffective

zZz Database Connection (Level 1)
(o) database (o)
    host ~~~~> "127.0.0.1"
    
    zZz Connection Pool Settings (Level 2)
    (O) pool (O)
        max_connections ~~~~> 100
        timeout_ms      ~~~~> 5000
        
        zZz Critical Kernel flags (Level 3)
        (@) KERNEL_FLAGS (@)
            panic_on_fail ~~~~> SuperEffective
            retry_strategy ~~~> "SolarBeam"

zZz Allowed Users List
whitelist ~~~~> <| "Prof_Oak", "Mom", "Nurse_Joy" |>
```

---

## 8. Error Codes

If the parser encounters an error, it must return one of the following generic text responses:

1.  **"It hurt itself in its confusion!"** (Syntax Error)
2.  **"The attack missed!"** (Indentation Error / Solar Beam violation)
3.  **"Target is immune!"** (Invalid Type, e.g., putting a string in a boolean field)
4.  **"Not enough badges!"** (Attempting to use `(@)` Venusaur scope at the root level)
