# navi-json
command line tool to navigate JSON files with basic SQL-like queries.

The name plays with the assonance with the word 'navigator', at least in the way I pronounce the word 'JSON', but I didn't focus a lot on the name.

## Purpose
The main purposes of this tool are two:
- to study and understand better the Rust language. I felt the need to try something outside the traditional exercises and I wanted to build something useful; I thought that this could be a good compromise.
- to have something that helps me navigate a 18MB JSON file with tens of thousands of objects that we use at work ("why aren't you using a database!?" yeah, I don't know)

I think that something like this already exists, and if not, there must be a reason. Anyway I wanted to challenge myself and try to develop my own. For my scope this tool does not need to be complicated, I won't add anything complex like `JOIN`s or `CASE`.
This is my first time writing something in Rust that is not a pure and incomplete exercise, so it surely won't be well designed, elegant or optimized, at least for now.

## Dependencies
- [clap](https://docs.rs/clap/latest/clap/index.html)
- [json](https://docs.rs/json/latest/json/)

navi-json is a command line tool, so it obviously needs to parse arguments; for that I decided to use 'clap' to study the usage of crates and to spend less time on parsing arguments and writing help commands. 
The second dependency is obviously 'json', which helps me a lot with JSON data type manipulation.

Among the first things I decided to do with this project was to eliminate the `unwrap()` method. I am aiming to build something for production usage, so the unwrap is only on the test functions. To do so I think I will probably use [thiserror](https://docs.rs/thiserror/latest/thiserror/) or something similar. Until now I just used a generic error but I will focus more on that later on.

## Feature
The main usage of the tool is to filter through json objects or arrays, the result will always be an array.

### Ready
The first features I introduced were the ones I thought simpler:
- skip, you can skip the first n results.
- limit, you can limit the final number of results
- select, you can select specific properties to view in the results. You can select nested properties with dot notation and if a property does not exists it will have as value the string "not found". I know this is not the best choice, I needed something different from `null`, so I put that but it will change shortly.

### To be made
- select for arrays elements, I would like to add the possibility to select only the nth element of an array property.
- where, the obvious goal of this tool is to search and filter by condition, so a 'where' clause is a must
- count, I would like to add a count clause to just get the number of the results instead of the complete list
