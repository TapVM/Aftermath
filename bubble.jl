#!/usr/bin/env julia

include("./BubbleScripts/namespace.jl")

# ANSI Colors ✨
const RED = "\x1b[31m"
const GREEN = "\x1b[32m"
const BOLD = "\x1b[1m"
const CODE = "\x1b[100m\x1b[30m"
const RESET = "\x1b[0m"

# Messages
const BUILD_FAILURE = "
Note -> If you or any other program didn't edit the source code which might've
caused this build failure, we would greatly appreciate if you made a bug report.
"

function build()
    if length(ARGS) != 1
        if ARGS[2] != "--release"
            println("\n$(RED)Unknown flag$RESET -> $(ARGS[2])\n")
            exit(1)
        else
            try
                run(`cargo run --release`)
            catch e
                println("$(RED)Build failed -> $(e).$RESET")
                println(BUILD_FAILURE)
                exit(1)
            end
        end
    end

    try
        run(`cargo run`)
    catch e
        println("$(RED)Build failed -> $(e).$RESET")
        println(BUILD_FAILURE)
        exit(1)
    end
end

function test()
    try
        run(`cargo test`)
    catch e
        println("$(RED)Build failed -> $(e).$RESET")
        println(BUILD_FAILURE)
        exit(1)
    end
end

function fmt()
    try
        run(`cargo fmt`)
    catch e
        println("$(RED)Build failed -> $(e).$RESET")
        println(BUILD_FAILURE)
        exit(1)
    end
end

function buildjdk()
    println("$GREEN We're not there... yet. $RESET😉")
    exit(0)
end

function advancedtest()
    println("$GREEN We're not there... yet. $RESET😉")
    exit(0)
end

function classbasket()
    print("\n$(GREEN)Building classbasket")

    try
        for file in filter((z) -> endswith(z, ".java"), readdir("./class_basket"))
            run(`javac ./class_basket/$file`)
            print(".")
        end
        println(" Done! $(RESET)✨\n")
        println("Building $(RED)invalid$RESET classfiles...")
        intentionally_invalid()
    catch e
        println("$(RED)Build failed -> $e.$RESET")
        println(BUILD_FAILURE)
        exit(1)
    end

    println("$RESET")
end

if length(ARGS) == 0
    println("""

$GREEN Welcome to the TapVM Aftermath JVM build system. $RESET

$BOLD Commands $RESET
--------------------------------------------------------------------------------
⦁ build        ~ Builds the JVM.
⦁ build-jdk    ~ Builds an OpenJDK distribution using Aftermath.
⦁ test         ~ Tests the JVM.
⦁ advancedtest ~ Advanced testing for the JVM. (Requires an internet connection)
⦁ classbasket  ~ Builds small Java snippets for JVM development and testing.
⦁ fmt          ~ Formats the source code using Rustfmt.

$BOLD Flags $RESET
--------------------------------------------------------------------------------
The $CODE build $RESET and $CODE build-jdk $RESET commands can have an optional $CODE --release $RESET flag. This
will build an optimized and stripped build, however the build time will be more
than a normal build (i.e a$BOLD development$RESET build).
""")
    exit()
else
    command = ARGS[1]
    if command == "build"
        build()
    elseif command == "build-jdk"
        buildjdk()
    elseif command == "test"
        test()
    elseif command == "advancedtest"
        advancedtest()
    elseif command == "classbasket"
        classbasket()
    elseif command == "fmt"
        fmt()
    else
        println("$(RED)Unknown command$RESET -> $command. Please run bubble without any arguments
for help regarding its usage.
")
    end
end
