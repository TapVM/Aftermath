#!/usr/bin/env python3
import sys
import os

argv = sys.argv
WELCOME_MESSAGE = """
Welcome to the TapVM Aftermath JVM build system.
The following are the available commands.

# Building.
- buildjvm ~ Builds the JVM alone.
- buildjdk ~ Builds OpenJDK with the built JVM.

# Testing.
- test ~ Tests the JVM.
- advancedtest ~ Advanced testing for the JVM (Requires an internet connection).

# Development utilities.
- classbasket ~ Builds random small Java snippets for JVM testing.
- fmt ~ Formats the source code.

Note -> Set the "BUBBLE_RELEASE" environment variable to "true" to build release builds.
"""

RED = "\033[91m"
GREEN = "\033[92m"
RESET = "\033[0m"

def buildjvm():
    if os.environ.get("BUBBLE_RELEASE") == "true":
        print(f"\n{GREEN}Building production JVM... {RESET}\n")
        if os.system("cargo build --verbose --release") != 0:
            print(f"\n{RED}Build FAILED. :<{RESET}\n")
        else:
            print(f"\n{GREEN}Build successful!{RESET}\n")
    else:
        print(f"\n{GREEN}Building development JVM... {RED}[DO NOT USE THIS BUILD IN PRODUCTION]{RESET}\n")
        if os.system("cargo build --verbose") != 0:
            print(f"\n{RED}Build FAILED. :<{RESET}\n")
        else:
            print(f"\n{GREEN}Build successful!{RESET}\n")

def buildjdk():
    print(f"\n{GREEN}Not so fast cowboy, we ain't there.... yet ;){RESET}\n")

def classbasket():
    print(f"\n{GREEN}Building random small Java snippets for JVM testing...{RESET}\n")
    if os.system("javac ./class_basket/*.java") != 0:
        print(f"\n{RED}Build FAILED. :<{RESET}\n")
        exit()
    else:
        print(f"\n{GREEN}Build successful!{RESET}\n")

    os.chdir("scripts/byte_mods/src/");
    if os.system("cargo run") != 0:
        print(f"\n{RED}Byte mods build FAILED. :<{RESET}\n")
    else:
        print(f"\n{GREEN}Byte mods build successful!{RESET}\n")

def test():
    classbasket()
    print(f"\n{GREEN}Testing...{RESET}\n")
    if os.system("cargo test") != 0:
        print(f"\n{RED}Tests FAILED. :<{RESET}\n")
    else:
        print(f"\n{GREEN}Tests successful!{RESET}\n")

def fmt():
    print(f"\n{GREEN}Formatting...{RESET}\n")
    if os.system("cargo fmt") != 0:
        print(f"\n{RED}Formatting FAILED. :<{RESET}\n")
    else:
        print(f"\n{GREEN}Formatting successful!{RESET}\n")

def main():
    if len(argv) == 1:
        print(WELCOME_MESSAGE)
        exit()

    command = argv[1].lower()

    if command == "buildjvm":
        buildjvm()

    elif command == "buildjdk":
        buildjdk()

    elif command == "test":
        test()

    elif command == "advancedtest":
        advancedtest()

    elif command == "classbasket":
        classbasket()

    elif command == "fmt":
        fmt()

    else:
        print(WELCOME_MESSAGE)

# --------------------------------------------------------------------------------------------------

main()