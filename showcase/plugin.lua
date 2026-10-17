--- # Lua adapter example
--- Lua `---` comments immediately above a function are treated as docs.

--- Calculate the sum of two numbers.
function plugin.add(left, right)
    return left + right
end

-- Regular Lua comments are ignored by the adapter.
--- Format a label using method-style syntax.
function plugin:label(value)
    return tostring(value)
end
