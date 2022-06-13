include("logger.jl")
const path = "./class_basket/intentional_invalid_magic.class"
function intentionally_invalid()
    bytes = read(path)
    bytes[1:4] = zeros(4)
    write(path, bytes)

    ScriptLog("made class with invalid magic (Should be 0xCAFEBABE, now contains zeros)")
end
