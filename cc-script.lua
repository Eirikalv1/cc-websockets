function strIndex(tbl, val)
    for k,v in ipairs(tbl) do
        if v == val then return k end
    end
end

function linearize(x, y, z, r, w)
    return (z + r)*(w * w) + (y + r)*w + (x + r)
end

function serialize(data, r)
    local w = (2*r + 1)

    local k = {}
    local v = {}
    for i=1, w*w*w do v[i] = 0 end

    for i=1, #data do
        local b = data[i]

        if not strIndex(k, b["name"]) then
            k[#k+1] = b["name"] end

        v[linearize(b["x"], b["y"], b["z"], r, w) + 1] = strIndex(k, b["name"])
    end

    return k, v
end

local ws = http.websocket("wss://IP")
local geo = peripheral.wrap("left")

local r = 3

local data = geo.scan(r)

local blocks, pos = serialize(data, r)
ws.send(textutils.serializeJSON(blocks) .. textutils.serializeJSON(pos))

while true do
    local packet = ws.receive()
    if not packet then break end

    _, err = pcall(loadstring(packet))
    ws.send(err)
end

ws.close()