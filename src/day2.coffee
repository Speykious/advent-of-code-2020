{ readFileSync } = require "fs"

data = readFileSync "../resources/day2.dat", "utf8"
data = data.split "\n"
match2pwrule = ([input, min, max, letter, password]) -> ({ min, max, letter, password })
data = data.map (s) -> match2pwrule s.match /^(\d+)-(\d+) (\w): (\w+)/

testpwrule1 = (r) ->
  counter = 0
  for c in r.password
    if c is r.letter then counter++
  return counter >= r.min and counter <= r.max

testpwrule2 = ({ password, min, max, letter }) ->
  a = password[min-1] is letter
  b = password[max-1] is letter
  return (a and not b) or (not a and b)

valids1 = 0
for r in data
  if testpwrule1 r then valids1++
console.log "\x1b[1mNumber of valids (1) :", valids1

valids2 = 0
for r in data
  if testpwrule2 r then valids2++
console.log "\x1b[1mNumber of valids (2) :", valids2