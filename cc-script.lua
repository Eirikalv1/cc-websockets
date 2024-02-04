local RADIUS = 8

function strIndex(tbl, val)
 for k,v in ipairs(tbl) do
  if v == val then return k end
 end
end

function linearize(x, y, z)
 local r = RADIUS
 local w = 2 * r + 1
 return (z + r)*(w * w) + (y + r)*w + (x + r)
end

function serialize(data)
 local r = RADIUS
 local w = 2 * r + 1
 
 local k = {}
 local v = {}
 for i=1, w*w*w do v[i] = 0 end
 
 for i=1,#data do
  local b = data[i]
  
  if not strIndex(k, b["name"]) then 
   k[#k+1] = b["name"] 
  end
  
  v[linearize(b["x"], b["y"], b["z"], r, w )+1] = strIndex(k, b["name"])
 end
 
 return k, v 
end

local ws = http.websocketAsync("wss://332d-178-232-110-172.ngrok-free.app")  
if not ws then print("Could not create websocket") end
local _,_,ws = os.pullEvent("websocket_success")

local geo = peripheral.wrap("left")

os.startTimer(0.5)

while true do
 local event, _, msg, _ = os.pullEvent()
 if event == "websocket_message" then
  if msg then 
   success, response = pcall(loadstring(msg))
   if success then
    ws.send("0")
   else
    ws.send("1" .. response)
   end
  end
 end
 
 if event == "timer" then 
  local data = geo.scan(RADIUS)
  local blocks, pos = serialize(data)
  ws.send("2" .. textutils.serializeJSON(blocks) .. textutils.serializeJSON(pos))
  
  os.startTimer(2)
 end
end
