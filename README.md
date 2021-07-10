# Tok2me
Tok2me is a [Maximal munch](https://www.wikiwand.com/en/Maximal_munch) parser designed to fit into terminal pipelines and tokenize user input and simplify text processing pipelines. Now any program that can read a tsv can utilize Tok2me to tokenize user input. Tok2me reads a token definition yaml file, tokenizes an input file (or reads from stdin if no input file is provided), and writes the tokenized output to stdout in the format `TOKEN_NAME<TAB>token_value`.

e.g. Token deffinition:
```yaml
# tokens.yaml
# Example token deffinition document
# Each token has a name and a list of regular expressions that it can match
ignore: []
tokens:
  - 
    token_type: "COMMA"
    exprs: [","]
  - 
    token_type: "WS"
    exprs: ["[ \\t]+"]
  -
    token_type: "NL"
    exprs: ["\\n","\\r\\n"]
  -
    token_type: "STRING"
    exprs: ["\"([\\s\\S]*?)\""]

```
e.g. Running tok2me with the provided token file on standard input:
```shell
printf "\"This is a string\" ,\r\n" | tok2me.exe -t tokens.yaml

```
output:
```tsv
# This is the tokenized output from tok2me.
# Lines beginning with '#' are comments and may be skipped!
STRING	"This is a string"
WS	 
COMMA	,
NL	\r\n

```