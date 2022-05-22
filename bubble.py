#!/usr/bin/env python3
import sys
import os

# A bare bones replication of the dbg! macro in Python
# Just made for easier development


def dbg(elem):
    print(f"DBG ~> {elem}")
    return elem

RED: str = "\033[91m"
GREEN: str = "\033[92m"
RESET: str = "\033[0m"

infobox = {
    "bytemods": f"""{GREEN}Info ~> \n\nBytemods -> Bytemods is a process which intentionally *breaks* a suite of class files for testing{RESET}"""
}
argv: list[str] = sys.argv
WELCOME_MESSAGE: str = """
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
- info [topic] ~ Gives information regarding the JVM about the given topic.

Note -> Set the "BUBBLE_RELEASE" environment variable to "true" to build release builds.
"""

def buildjvm():
    if os.environ.get("BUBBLE_RELEASE") == "true":
        print(f"\n{GREEN}Building production JVM... {RESET}\n")
        if os.system("cargo build --verbose --release") != 0:
            print(f"\n{RED}Build FAILED. :({RESET}\n")
        else:
            print(f"\n{GREEN}Build successful!{RESET}\n")
    else:
        print(
            f"\n{GREEN}Building development JVM... {RED}[DO NOT USE THIS BUILD IN PRODUCTION]{RESET}\n")
        if os.system("cargo build --verbose") != 0:
            print(f"\n{RED}Build FAILED. :({RESET}\n")
        else:
            print(f"\n{GREEN}Build successful!{RESET}\n")


def buildjdk():
    print(f"\n{GREEN}Not so fast cowboy, we ain't there.... yet ;){RESET}\n")


def classbasket():
    print(f"\n{GREEN}Building random small Java snippets for JVM testing...{RESET}")
    if os.system("javac ./class_basket/*.java") != 0:
        print(f"\n{RED}Build FAILED. {RESET}\n")
        exit()
    else:
        print(f"\n{GREEN}Build successful!{RESET}\n")

    os.chdir("scripts/byte_mods/src/")
    print(f"{GREEN}Building byte mods (Run `bubble info bytemods` to know more)\n{RESET}")
    if os.system("cargo run") != 0:
        print(f"\n{RED}Byte mods build FAILED. {RESET}\n")
    else:
        print(f"\n{GREEN}Byte mods build successful!{RESET}\n")


def test():
    classbasket()
    print(f"\n{GREEN}Testing...{RESET}\n")
    if os.system("cargo test") != 0:
        print(f"\n{RED}Tests FAILED. :({RESET}\n")
    else:
        print(f"\n{GREEN}Tests successful!{RESET}\n")


def fmt():
    print(f"\n{GREEN}Formatting...{RESET}")
    if os.system("cargo fmt") != 0:
        print(f"\n{RED}Formatting FAILED. :({RESET}\n")
    else:
        print(f"\n{GREEN}Formatting successful!{RESET}\n")

def info():
    try:
        info: str = argv[2]
    except IndexError:
        print(f"\n{RED}Please provide an info ID. Available options ->{GREEN}")
        for key in infobox.keys():
            print(f"~ {key}")
        print()
        exit(0)

    if infobox.get(info) is not None:
        print(infobox[info])
    else:
        print(f"\n{RED}Please provide a valid info ID. Available options ->{GREEN}")
        for key in infobox.keys():
            print(f"~ {key}")
        print()

def main():
    if len(argv) == 1:
        print(WELCOME_MESSAGE)
        exit()

    command: str = argv[1].lower()

    match command:
        case "buildjvm":
            buildjvm()
        case "buildjdk":
            buildjdk()
        case "test":
            test()
        case "advancedtest":
            advancedtest()
        case "classbasket":
            classbasket()
        case "fmt":
            fmt()
        case "info":
            info()

# --------------------------------------------------------------------------------------------------

main()
