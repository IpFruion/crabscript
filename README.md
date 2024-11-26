# Crabscript

A _crustacean_ script library, cli, and runtime that is cross system compatible.

## CLI Example

```
echo "echo hello ${0}" > hello_script
crabscript hello_script "world"
```

The `crabscript` command will load some common sdk commands by default and parse and run the script as is with provided arguments after the script file location.
