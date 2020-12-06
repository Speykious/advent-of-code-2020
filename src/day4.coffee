{ readFileSync } = require "fs"

data = readFileSync "../resources/day4.dat", "utf8"
data = data.split "\n\n"
       .map (s) ->
          new Map (s.split /\s+/
          .map (s) -> s.split ":")

console.log data.map (m) -> [m.keys()...]

digits = (s, n, min, max) ->
  unless (new RegExp "^(\\d{#{n}})$").test s then return false
  return +s >= min and +s <= max

validate = (p) ->
  unless digits (p.get "byr"), 4, 1920, 2002 then return false
  unless digits (p.get "iyr"), 4, 2010, 2020 then return false
  unless digits (p.get "eyr"), 4, 2020, 2030 then return false
  
  hgt = p.get "hgt"
  if hgt is undefined then return false
  nhgt = +(hgt.match /^(\d+)/)[1]
  if nhgt is NaN then return false
  if /cm$/.test hgt
    unless nhgt >= 150 and nhgt <= 193 then return false
  else if /in$/.test hgt
    unless nhgt >= 59 and nhgt <= 76 then return false
  else return false
  unless /^#[0-9a-fA-F]{6}$/.test p.get "hcl" then return false
  unless ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].includes p.get "ecl" then return false
  unless /^\d{9}$/.test p.get "pid" then return false
  return true

valids = 0
for p in data
  valids += if validate p then 1 else 0
  console.log (validate p), p

console.log "Number of valid passports:", valids