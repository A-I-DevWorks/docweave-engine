--- Lua module documentation.

--- Adds two values.
function api.add(left, right)
    local function nested(value)
        return value
    end
    return left + right
end

function api:label(value)
    return tostring(value)
end

function undocumented(value)
    return value
end
