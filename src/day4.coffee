{ readFileSync } = require "fs"

data = readFileSync "../resources/day2.dat", "utf8"
data = data.split "\n\n"

console.log data