# Json parser

A tool to parse json files
  
|Linux|Windows  |
|--|--|
|./json-parser | json-parser.exe |

## 1. Usage

    Json file parser

    USAGE:
        json-parser --filename <FILE> --keys <TEXT>

    OPTIONS:
        -f, --filename <FILE>    Json file to parse
        -h, --help               Print help information
        -k, --keys <TEXT>        Property to search. Use '.' for nested properties and/or array indexes
        -V, --version            Print version information


## 2. Example

[sample.json](./sample.json)
```json

{
    "title": "the lord of the rings",
    "characters": [
        {
            "name": "frodo",
            "race": "hobbit"
        },
        {
            "name": "aragorn",
            "race": "human"
        }
    ]
}

```

```sh
./json-parser -f sample.json -k title
```

```json
"the lord of the rings"
```

```sh
./json-parser-f sample.json -k characters.0.race
```

```json
"hobbit"
```

## 3. More json example files
[bitcoin.json](./bitcon.json)

[bitcoin-uncorfirmed.json](./bitcoin-uncorfirmed.json)

[us-representatives.json](./us-representatives.json)
