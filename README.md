# Simpler Tree-Sitter CLI to just print the CST

Just a very simple way of parsing and printing the code CST with tree-sitter. 

You may use the compiled binary from the releases section of this repository or compile it yourself.

# Build from source

Clone this repository and run `make`. The cli binary is then located in the generated `build` repository.

# Example

For example, in this repository after running `make`:

For help, run:

```bash
arthur@pop-os:$ ./build/treesittercli help

Usage: <program> <language: string> <source code filepath: string> <optional format: normal, xml, json>
```

For a full example of a normal output, see [./example_output/example.txt](./example_output/example.txt)
```bash
arthur@pop-os:$ ./build/treesittercli rust ./src/main.rs

(source_file [0, 0] - [209, 0]
  (inner_attribute_item [0, 0] - [0, 64]
    (attribute [0, 3] - [0, 63]
      (identifier [0, 3] - [0, 11])
      arguments: (token_tree [0, 11] - [0, 63]
        (identifier [0, 12] - [0, 28])
        (identifier [0, 30] - [0, 35])
        (token_tree [0, 35] - [0, 62]
          (identifier [0, 36] - [0, 45])
          (identifier [0, 47] - [0, 61])))))

          [...]
```

For a full example of an xml output, see [./example_output/example.xml](./example_output/example.xml)
```bash
arthur@pop-os:$ ./build/treesittercli rust ./src/main.rs xml

<source_file>
  <inner_attribute_item>#![
    <attribute>
      <identifier>cfg_attr</identifier>

      <token_tree type="arguments">(
        <identifier>debug_assertions</identifier>

        <identifier>allow</identifier>

        <token_tree>(
          <identifier>dead_code</identifier>

          [...]
```

For a full example of an xml output, see [./example_output/example.json](./example_output/example.json)
```bash
arthur@pop-os:$ ./build/treesittercli rust ./src/main.rs json

{
  "children": [
    {
      "children": [
        {
          "children": [],
          "end": {
            "column": 1,
            "row": 0
          },
          "kind": "#",
          "start": {
            "column": 0,
            "row": 0
          },
          "value": "#"
        },
        {
          "children": [],
          "end": {
            "column": 2,
            "row": 0
          },
          "kind": "!",
          "start": {
            "column": 1,
            "row": 0
          },
          "value": "!"

          [...]
```