#include "BSONParser.hpp"
#include <iostream>
#include <cassert>

void testValid() {
    std::string input = R"(BULBA!
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
)";

    BSONParser parser;
    try {
        BSONMap result = parser.parse(input);
        std::cout << "Test Valid: PASS" << std::endl;
    } catch (const std::exception& e) {
        std::cout << "Test Valid: FAIL - " << e.what() << std::endl;
        exit(1);
    }
}

void testError(std::string name, std::string input, std::string expectedError) {
    BSONParser parser;
    try {
        parser.parse(input);
        std::cout << "Test " << name << ": FAIL - Expected error " << expectedError << " but got none" << std::endl;
        exit(1);
    } catch (const std::exception& e) {
        std::string msg = e.what();
        if (msg.find(expectedError) != std::string::npos) {
            std::cout << "Test " << name << ": PASS" << std::endl;
        } else {
            std::cout << "Test " << name << ": FAIL - Expected error " << expectedError << " but got " << msg << std::endl;
            exit(1);
        }
    }
}

int main() {
    testValid();
    
    testError("Invalid Header", "NOT_BULBA!\nkey ~> \"val\"", "Status: Fainted");
    testError("Tab Character", "BULBA!\n\tkey ~> \"val\"", "Poison Type");
    testError("Bad Indentation", "BULBA!\n key ~> \"val\"", "The attack missed!");
    testError("Charizard Key", "BULBA!\nCharizard ~> \"Fire\"", "It burns the bulb");
    
    // Deep Nesting Violation
    // (o) level1 (o)
    //         (@) level3 (@)  <-- Indent 8
    //             key ~> "val"
    std::string deepNesting = "BULBA!\n(o) level1 (o)\n        (@) level3 (@)\n            key ~> \"val\"";
    testError("Deep Nesting Violation", deepNesting, "Not enough badges!");
    
    testError("Invalid Type", "BULBA!\nkey ~> UnknownType", "Target is immune!");

    return 0;
}
