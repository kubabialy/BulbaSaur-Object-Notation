package main

import (
	"reflect"
	"testing"
)

func TestParse_Valid(t *testing.T) {
	input := `BULBA!
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
`

	expected := map[string]interface{}{
		"app_name":      "Pokedex_API",
		"version":       1.5,
		"is_production": false,
		"missing_data":  nil,
		"database": map[string]interface{}{
			"host": "127.0.0.1",
			"pool": map[string]interface{}{
				"max_connections": 100,
				"KERNEL_FLAGS": map[string]interface{}{
					"panic_on_fail": true,
				},
			},
		},
		"whitelist": []interface{}{"Prof_Oak", "Mom"},
	}

	result, err := Parse(input)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected:\n%v\nGot:\n%v", expected, result)
	}
}

func TestParse_Errors(t *testing.T) {
	tests := []struct {
		name      string
		input     string
		errSubstr string
	}{
		{
			name: "Invalid Header",
			input: `NOT_BULBA!
key ~> "value"`,
			errSubstr: "Status: Fainted",
		},
		{
			name: "Tab Character",
			input: `BULBA!
	key ~> "value"`,
			errSubstr: "Poison Type",
		},
		{
			name: "Bad Indentation",
			input: `BULBA!
 key ~> "value"`,
			errSubstr: ErrIndentation,
		},
		{
			name: "Charizard Key",
			input: `BULBA!
Charizard ~> "Fire"`,
			errSubstr: "It burns the bulb",
		},
		{
			name: "Deep Nesting Violation",
			input: `BULBA!
(o) level1 (o)
        (@) level3 (@)
            key ~> "val"`,
			errSubstr: ErrBadges,
		},
		{
			name: "Invalid Type",
			input: `BULBA!
key ~> UnknownType`,
			errSubstr: ErrType,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := Parse(tt.input)
			if err == nil {
				t.Errorf("Expected error containing %q, got nil", tt.errSubstr)
			} else if !contains(err.Error(), tt.errSubstr) {
				t.Errorf("Expected error containing %q, got %q", tt.errSubstr, err.Error())
			}
		})
	}
}

func contains(s, substr string) bool {
	return len(s) >= len(substr) && s[0:len(substr)] == substr || (len(s) > len(substr) && contains(s[1:], substr))
}

func TestPrintAST(t *testing.T) {
	input := `BULBA!
name ~> "Bulby"`
	result, err := Parse(input)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	// Just verify it doesn't panic
	PrintAST(result)
}
