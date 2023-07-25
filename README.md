# Simpler Tree-Sitter CLI to just print the CST

Just a very simple way of parsing and printing the code CST with tree-sitter. 

You may use the compiled binary from the releases section of this repository or compile it yourself.

# Build from source

Clone this repository and run `make`. The cli binary is then located in the generated `build` repository.

# Example

For example, in this repository after running `make`:

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
  (use_declaration [1, 0] - [1, 13]
    argument: (scoped_identifier [1, 4] - [1, 12]
      path: (identifier [1, 4] - [1, 7])
      name: (identifier [1, 9] - [1, 12])))
  (use_declaration [2, 0] - [2, 20]
    argument: (scoped_identifier [2, 4] - [2, 19]
      path: (scoped_identifier [2, 4] - [2, 12]
        path: (identifier [2, 4] - [2, 7])
        name: (identifier [2, 9] - [2, 12]))
      name: (identifier [2, 14] - [2, 19])))
  (use_declaration [3, 0] - [3, 12]
    argument: (scoped_identifier [3, 4] - [3, 11]
      path: (identifier [3, 4] - [3, 7])
      name: (identifier [3, 9] - [3, 11])))
  (use_declaration [4, 0] - [4, 12]
    argument: (scoped_identifier [4, 4] - [4, 11]
      path: (identifier [4, 4] - [4, 7])
      name: (identifier [4, 9] - [4, 11])))
  (use_declaration [5, 0] - [5, 19]
    argument: (scoped_identifier [5, 4] - [5, 18]
      path: (scoped_identifier [5, 4] - [5, 11]
        path: (identifier [5, 4] - [5, 7])
        name: (identifier [5, 9] - [5, 11]))
      name: (identifier [5, 13] - [5, 18])))
  (use_declaration [6, 0] - [6, 20]
    argument: (scoped_identifier [6, 4] - [6, 19]
      path: (scoped_identifier [6, 4] - [6, 13]
        path: (identifier [6, 4] - [6, 7])
        name: (identifier [6, 9] - [6, 13]))
      name: (identifier [6, 15] - [6, 19])))
  (use_declaration [7, 0] - [7, 17]
    argument: (scoped_identifier [7, 4] - [7, 16]
      path: (identifier [7, 4] - [7, 7])
      name: (identifier [7, 9] - [7, 16])))
  (use_declaration [8, 0] - [8, 35]
    argument: (scoped_identifier [8, 4] - [8, 34]
      path: (scoped_identifier [8, 4] - [8, 21]
        path: (scoped_identifier [8, 4] - [8, 13]
          path: (identifier [8, 4] - [8, 7])
          name: (identifier [8, 9] - [8, 13]))
        name: (identifier [8, 15] - [8, 21]))
      name: (identifier [8, 23] - [8, 34])))
  (use_declaration [9, 0] - [9, 23]
    argument: (scoped_identifier [9, 4] - [9, 22]
      path: (scoped_identifier [9, 4] - [9, 13]
        path: (identifier [9, 4] - [9, 7])
        name: (identifier [9, 9] - [9, 13]))
      name: (identifier [9, 15] - [9, 22])))
  (use_declaration [10, 0] - [10, 22]
    argument: (scoped_use_list [10, 4] - [10, 21]
      path: (identifier [10, 4] - [10, 7])
      list: (use_list [10, 9] - [10, 21]
        (identifier [10, 10] - [10, 13])
        (identifier [10, 15] - [10, 20]))))
  (use_declaration [11, 0] - [11, 69]
    argument: (scoped_use_list [11, 4] - [11, 68]
      path: (identifier [11, 4] - [11, 15])
      list: (use_list [11, 17] - [11, 68]
        (identifier [11, 18] - [11, 27])
        (identifier [11, 29] - [11, 37])
        (identifier [11, 39] - [11, 46])
        (identifier [11, 48] - [11, 54])
        (identifier [11, 56] - [11, 61])
        (identifier [11, 63] - [11, 67]))))
  (line_comment [13, 0] - [13, 83])
  (function_item [15, 0] - [18, 1]
    name: (identifier [15, 3] - [15, 12])
    parameters: (parameters [15, 12] - [15, 24]
      (parameter [15, 13] - [15, 23]
        pattern: (identifier [15, 13] - [15, 17])
        type: (reference_type [15, 19] - [15, 23]
          type: (primitive_type [15, 20] - [15, 23]))))
    return_type: (generic_type [15, 28] - [15, 46]
      type: (scoped_type_identifier [15, 28] - [15, 38]
        path: (identifier [15, 28] - [15, 30])
        name: (type_identifier [15, 32] - [15, 38]))
      type_arguments: (type_arguments [15, 38] - [15, 46]
        (type_identifier [15, 39] - [15, 45])))
    body: (block [15, 47] - [18, 1]
      (let_declaration [16, 4] - [16, 45]
        pattern: (identifier [16, 8] - [16, 16])
        value: (try_expression [16, 19] - [16, 44]
          (call_expression [16, 19] - [16, 43]
            function: (scoped_identifier [16, 19] - [16, 37]
              path: (identifier [16, 19] - [16, 21])
              name: (identifier [16, 23] - [16, 37]))
            arguments: (arguments [16, 37] - [16, 43]
              (identifier [16, 38] - [16, 42])))))
      (call_expression [17, 4] - [17, 16]
        function: (identifier [17, 4] - [17, 6])
        arguments: (arguments [17, 6] - [17, 16]
          (identifier [17, 7] - [17, 15])))))
  (macro_definition [20, 0] - [25, 1]
    name: (identifier [20, 13] - [20, 23])
    (macro_rule [21, 4] - [24, 6]
      left: (token_tree_pattern [21, 4] - [21, 30]
        (token_binding_pattern [21, 5] - [21, 14]
          name: (metavariable [21, 5] - [21, 9])
          type: (fragment_specifier [21, 10] - [21, 14]))
        (token_repetition_pattern [21, 15] - [21, 29]
          (token_binding_pattern [21, 18] - [21, 27]
            name: (metavariable [21, 18] - [21, 22])
            type: (fragment_specifier [21, 23] - [21, 27]))))
      right: (token_tree [21, 34] - [24, 6]
        (token_tree [21, 35] - [24, 5]
          (identifier [22, 8] - [22, 16])
          (token_tree [22, 17] - [22, 33]
            (metavariable [22, 18] - [22, 22])
            (token_repetition [22, 23] - [22, 32]
              (metavariable [22, 26] - [22, 30])))
          (identifier [23, 8] - [23, 15])
          (identifier [23, 17] - [23, 21])
          (token_tree [23, 21] - [23, 24]
            (integer_literal [23, 22] - [23, 23]))))))
  (function_item [27, 0] - [208, 1]
    name: (identifier [27, 3] - [27, 7])
    parameters: (parameters [27, 7] - [27, 9])
    body: (block [27, 10] - [208, 1]
      (line_comment [28, 4] - [28, 64])
      (let_declaration [29, 4] - [29, 50]
        pattern: (identifier [29, 8] - [29, 12])
        type: (generic_type [29, 14] - [29, 25]
          type: (type_identifier [29, 14] - [29, 17])
          type_arguments: (type_arguments [29, 17] - [29, 25]
            (type_identifier [29, 18] - [29, 24])))
        value: (call_expression [29, 28] - [29, 49]
          function: (field_expression [29, 28] - [29, 47]
            value: (call_expression [29, 28] - [29, 39]
              function: (scoped_identifier [29, 28] - [29, 37]
                path: (identifier [29, 28] - [29, 31])
                name: (identifier [29, 33] - [29, 37]))
              arguments: (arguments [29, 37] - [29, 39]))
            field: (field_identifier [29, 40] - [29, 47]))
          arguments: (arguments [29, 47] - [29, 49])))
      (expression_statement [31, 4] - [34, 5]
        (if_expression [31, 4] - [34, 5]
          condition: (binary_expression [31, 7] - [31, 22]
            left: (call_expression [31, 7] - [31, 17]
              function: (field_expression [31, 7] - [31, 15]
                value: (identifier [31, 7] - [31, 11])
                field: (field_identifier [31, 12] - [31, 15]))
              arguments: (arguments [31, 15] - [31, 17]))
            right: (integer_literal [31, 21] - [31, 22]))
          consequence: (block [31, 23] - [34, 5]
            (expression_statement [32, 8] - [32, 87]
              (macro_invocation [32, 8] - [32, 86]
                macro: (identifier [32, 8] - [32, 15])
                (token_tree [32, 16] - [32, 86]
                  (string_literal [32, 17] - [32, 85]))))
            (expression_statement [33, 8] - [33, 15]
              (return_expression [33, 8] - [33, 14])))))
      (line_comment [36, 4] - [36, 29])
      (let_declaration [37, 4] - [37, 32]
        pattern: (identifier [37, 8] - [37, 20])
        value: (reference_expression [37, 23] - [37, 31]
          value: (index_expression [37, 24] - [37, 31]
            (identifier [37, 24] - [37, 28])
            (integer_literal [37, 29] - [37, 30]))))
      (let_declaration [38, 4] - [38, 34]
        pattern: (identifier [38, 8] - [38, 22])
        value: (reference_expression [38, 25] - [38, 33]
          value: (index_expression [38, 26] - [38, 33]
            (identifier [38, 26] - [38, 30])
            (integer_literal [38, 31] - [38, 32]))))
      (let_declaration [40, 4] - [40, 35]
        (mutable_specifier [40, 8] - [40, 11])
        pattern: (identifier [40, 12] - [40, 18])
        value: (call_expression [40, 21] - [40, 34]
          function: (scoped_identifier [40, 21] - [40, 32]
            path: (identifier [40, 21] - [40, 27])
            name: (identifier [40, 29] - [40, 32]))
          arguments: (arguments [40, 32] - [40, 34])))
      (expression_statement [42, 4] - [68, 5]
        (match_expression [42, 4] - [68, 5]
          value: (call_expression [42, 10] - [42, 31]
            function: (field_expression [42, 10] - [42, 29]
              value: (identifier [42, 10] - [42, 22])
              field: (field_identifier [42, 23] - [42, 29]))
            arguments: (arguments [42, 29] - [42, 31]))
          body: (match_block [42, 32] - [68, 5]
            (match_arm [43, 8] - [45, 50]
              pattern: (match_pattern [43, 8] - [43, 14]
                (string_literal [43, 8] - [43, 14]))
              value: (call_expression [43, 18] - [45, 49]
                function: (field_expression [43, 18] - [45, 19]
                  value: (call_expression [43, 18] - [44, 55]
                    function: (field_expression [43, 18] - [44, 25]
                      value: (identifier [43, 18] - [43, 24])
                      field: (field_identifier [44, 13] - [44, 25]))
                    arguments: (arguments [44, 25] - [44, 55]
                      (call_expression [44, 26] - [44, 54]
                        function: (scoped_identifier [44, 26] - [44, 52]
                          path: (identifier [44, 26] - [44, 42])
                          name: (identifier [44, 44] - [44, 52]))
                        arguments: (arguments [44, 52] - [44, 54]))))
                  field: (field_identifier [45, 13] - [45, 19]))
                arguments: (arguments [45, 19] - [45, 49]
                  (string_literal [45, 20] - [45, 48]))))
            (match_arm [46, 8] - [48, 50]
              pattern: (match_pattern [46, 8] - [46, 12]
                (string_literal [46, 8] - [46, 12]))
              value: (call_expression [46, 16] - [48, 49]
                function: (field_expression [46, 16] - [48, 19]
                  value: (call_expression [46, 16] - [47, 53]
                    function: (field_expression [46, 16] - [47, 25]
                      value: (identifier [46, 16] - [46, 22])
                      field: (field_identifier [47, 13] - [47, 25]))
                    arguments: (arguments [47, 25] - [47, 53]
                      (call_expression [47, 26] - [47, 52]
                        function: (scoped_identifier [47, 26] - [47, 50]
                          path: (identifier [47, 26] - [47, 40])
                          name: (identifier [47, 42] - [47, 50]))
                        arguments: (arguments [47, 50] - [47, 52]))))
                  field: (field_identifier [48, 13] - [48, 19]))
                arguments: (arguments [48, 19] - [48, 49]
                  (string_literal [48, 20] - [48, 48]))))
            (match_arm [49, 8] - [51, 50]
              pattern: (match_pattern [49, 8] - [49, 20]
                (string_literal [49, 8] - [49, 20]))
              value: (call_expression [49, 24] - [51, 49]
                function: (field_expression [49, 24] - [51, 19]
                  value: (call_expression [49, 24] - [50, 61]
                    function: (field_expression [49, 24] - [50, 25]
                      value: (identifier [49, 24] - [49, 30])
                      field: (field_identifier [50, 13] - [50, 25]))
                    arguments: (arguments [50, 25] - [50, 61]
                      (call_expression [50, 26] - [50, 60]
                        function: (scoped_identifier [50, 26] - [50, 58]
                          path: (identifier [50, 26] - [50, 48])
                          name: (identifier [50, 50] - [50, 58]))
                        arguments: (arguments [50, 58] - [50, 60]))))
                  field: (field_identifier [51, 13] - [51, 19]))
                arguments: (arguments [51, 19] - [51, 49]
                  (string_literal [51, 20] - [51, 48]))))
            (match_arm [52, 8] - [54, 50]
              pattern: (match_pattern [52, 8] - [52, 16]
                (string_literal [52, 8] - [52, 16]))
              value: (call_expression [52, 20] - [54, 49]
                function: (field_expression [52, 20] - [54, 19]
                  value: (call_expression [52, 20] - [53, 57]
                    function: (field_expression [52, 20] - [53, 25]
                      value: (identifier [52, 20] - [52, 26])
                      field: (field_identifier [53, 13] - [53, 25]))
                    arguments: (arguments [53, 25] - [53, 57]
                      (call_expression [53, 26] - [53, 56]
                        function: (scoped_identifier [53, 26] - [53, 54]
                          path: (identifier [53, 26] - [53, 44])
                          name: (identifier [53, 46] - [53, 54]))
                        arguments: (arguments [53, 54] - [53, 56]))))
                  field: (field_identifier [54, 13] - [54, 19]))
                arguments: (arguments [54, 19] - [54, 49]
                  (string_literal [54, 20] - [54, 48]))))
            (match_arm [55, 8] - [57, 50]
              pattern: (match_pattern [55, 8] - [55, 11]
                (string_literal [55, 8] - [55, 11]))
              value: (call_expression [55, 15] - [57, 49]
                function: (field_expression [55, 15] - [57, 19]
                  value: (call_expression [55, 15] - [56, 52]
                    function: (field_expression [55, 15] - [56, 25]
                      value: (identifier [55, 15] - [55, 21])
                      field: (field_identifier [56, 13] - [56, 25]))
                    arguments: (arguments [56, 25] - [56, 52]
                      (call_expression [56, 26] - [56, 51]
                        function: (scoped_identifier [56, 26] - [56, 49]
                          path: (identifier [56, 26] - [56, 39])
                          name: (identifier [56, 41] - [56, 49]))
                        arguments: (arguments [56, 49] - [56, 51]))))
                  field: (field_identifier [57, 13] - [57, 19]))
                arguments: (arguments [57, 19] - [57, 49]
                  (string_literal [57, 20] - [57, 48]))))
            (match_arm [58, 8] - [60, 50]
              pattern: (match_pattern [58, 8] - [58, 13]
                (string_literal [58, 8] - [58, 13]))
              value: (call_expression [58, 17] - [60, 49]
                function: (field_expression [58, 17] - [60, 19]
                  value: (call_expression [58, 17] - [59, 54]
                    function: (field_expression [58, 17] - [59, 25]
                      value: (identifier [58, 17] - [58, 23])
                      field: (field_identifier [59, 13] - [59, 25]))
                    arguments: (arguments [59, 25] - [59, 54]
                      (call_expression [59, 26] - [59, 53]
                        function: (scoped_identifier [59, 26] - [59, 51]
                          path: (identifier [59, 26] - [59, 41])
                          name: (identifier [59, 43] - [59, 51]))
                        arguments: (arguments [59, 51] - [59, 53]))))
                  field: (field_identifier [60, 13] - [60, 19]))
                arguments: (arguments [60, 19] - [60, 49]
                  (string_literal [60, 20] - [60, 48]))))
            (match_arm [61, 8] - [63, 50]
              pattern: (match_pattern [61, 8] - [61, 15]
                (string_literal [61, 8] - [61, 15]))
              value: (call_expression [61, 19] - [63, 49]
                function: (field_expression [61, 19] - [63, 19]
                  value: (call_expression [61, 19] - [62, 56]
                    function: (field_expression [61, 19] - [62, 25]
                      value: (identifier [61, 19] - [61, 25])
                      field: (field_identifier [62, 13] - [62, 25]))
                    arguments: (arguments [62, 25] - [62, 56]
                      (call_expression [62, 26] - [62, 55]
                        function: (scoped_identifier [62, 26] - [62, 53]
                          path: (identifier [62, 26] - [62, 43])
                          name: (identifier [62, 45] - [62, 53]))
                        arguments: (arguments [62, 53] - [62, 55]))))
                  field: (field_identifier [63, 13] - [63, 19]))
                arguments: (arguments [63, 19] - [63, 49]
                  (string_literal [63, 20] - [63, 48]))))
            (match_arm [64, 8] - [66, 50]
              pattern: (match_pattern [64, 8] - [64, 14]
                (string_literal [64, 8] - [64, 14]))
              value: (call_expression [64, 18] - [66, 49]
                function: (field_expression [64, 18] - [66, 19]
                  value: (call_expression [64, 18] - [65, 55]
                    function: (field_expression [64, 18] - [65, 25]
                      value: (identifier [64, 18] - [64, 24])
                      field: (field_identifier [65, 13] - [65, 25]))
                    arguments: (arguments [65, 25] - [65, 55]
                      (call_expression [65, 26] - [65, 54]
                        function: (scoped_identifier [65, 26] - [65, 52]
                          path: (identifier [65, 26] - [65, 42])
                          name: (identifier [65, 44] - [65, 52]))
                        arguments: (arguments [65, 52] - [65, 54]))))
                  field: (field_identifier [66, 13] - [66, 19]))
                arguments: (arguments [66, 19] - [66, 49]
                  (string_literal [66, 20] - [66, 48]))))
            (match_arm [67, 8] - [67, 55]
              pattern: (match_pattern [67, 8] - [67, 10]
                (reference_pattern [67, 8] - [67, 10]))
              value: (macro_invocation [67, 14] - [67, 54]
                macro: (identifier [67, 14] - [67, 24])
                (token_tree [67, 25] - [67, 54]
                  (string_literal [67, 26] - [67, 53])))))))
      (let_declaration [70, 4] - [73, 6]
        pattern: (identifier [70, 8] - [70, 19])
        value: (match_expression [70, 22] - [73, 5]
          value: (call_expression [70, 28] - [70, 53]
            function: (identifier [70, 28] - [70, 37])
            arguments: (arguments [70, 37] - [70, 53]
              (identifier [70, 38] - [70, 52])))
          body: (match_block [70, 54] - [73, 5]
            (match_arm [71, 8] - [71, 33]
              pattern: (match_pattern [71, 8] - [71, 20]
                (tuple_struct_pattern [71, 8] - [71, 20]
                  type: (identifier [71, 8] - [71, 10])
                  (identifier [71, 11] - [71, 19])))
              value: (identifier [71, 24] - [71, 32]))
            (match_arm [72, 8] - [72, 63]
              pattern: (match_pattern [72, 8] - [72, 16]
                (tuple_struct_pattern [72, 8] - [72, 16]
                  type: (identifier [72, 8] - [72, 11])
                  (identifier [72, 12] - [72, 15])))
              value: (macro_invocation [72, 20] - [72, 62]
                macro: (identifier [72, 20] - [72, 30])
                (token_tree [72, 31] - [72, 62]
                  (string_literal [72, 32] - [72, 56])
                  (identifier [72, 58] - [72, 61])))))))
      (let_declaration [75, 4] - [77, 7]
        pattern: (identifier [75, 8] - [75, 12])
        value: (call_expression [75, 15] - [77, 6]
          function: (field_expression [75, 15] - [75, 61]
            value: (call_expression [75, 15] - [75, 46]
              function: (field_expression [75, 15] - [75, 27]
                value: (identifier [75, 15] - [75, 21])
                field: (field_identifier [75, 22] - [75, 27]))
              arguments: (arguments [75, 27] - [75, 46]
                (identifier [75, 28] - [75, 39])
                (identifier [75, 41] - [75, 45])))
            field: (field_identifier [75, 47] - [75, 61]))
          arguments: (arguments [75, 61] - [77, 6]
            (closure_expression [75, 62] - [77, 5]
              parameters: (closure_parameters [75, 62] - [75, 64])
              body: (block [75, 65] - [77, 5]
                (expression_statement [76, 8] - [76, 53]
                  (macro_invocation [76, 8] - [76, 52]
                    macro: (identifier [76, 8] - [76, 18])
                    (token_tree [76, 19] - [76, 52]
                      (string_literal [76, 20] - [76, 51])))))))))
      (line_comment [78, 4] - [78, 40])
      (let_declaration [80, 4] - [80, 30]
        pattern: (identifier [80, 8] - [80, 14])
        value: (call_expression [80, 17] - [80, 29]
          function: (scoped_identifier [80, 17] - [80, 27]
            path: (identifier [80, 17] - [80, 19])
            name: (identifier [80, 21] - [80, 27]))
          arguments: (arguments [80, 27] - [80, 29])))
      (let_declaration [81, 4] - [81, 35]
        (mutable_specifier [81, 8] - [81, 11])
        pattern: (identifier [81, 12] - [81, 18])
        value: (call_expression [81, 21] - [81, 34]
          function: (field_expression [81, 21] - [81, 32]
            value: (identifier [81, 21] - [81, 27])
            field: (field_identifier [81, 28] - [81, 32]))
          arguments: (arguments [81, 32] - [81, 34])))
      (let_declaration [83, 4] - [83, 33]
        (mutable_specifier [83, 8] - [83, 11])
        pattern: (identifier [83, 12] - [83, 18])
        value: (call_expression [83, 21] - [83, 32]
          function: (field_expression [83, 21] - [83, 30]
            value: (identifier [83, 21] - [83, 25])
            field: (field_identifier [83, 26] - [83, 30]))
          arguments: (arguments [83, 30] - [83, 32])))
      (let_declaration [85, 4] - [85, 34]
        (mutable_specifier [85, 8] - [85, 11])
        pattern: (identifier [85, 12] - [85, 25])
        value: (boolean_literal [85, 28] - [85, 33]))
      (let_declaration [86, 4] - [86, 29]
        (mutable_specifier [86, 8] - [86, 11])
        pattern: (identifier [86, 12] - [86, 24])
        value: (integer_literal [86, 27] - [86, 28]))
      (let_declaration [87, 4] - [87, 39]
        (mutable_specifier [87, 8] - [87, 11])
        pattern: (identifier [87, 12] - [87, 30])
        value: (boolean_literal [87, 33] - [87, 38]))
      (expression_statement [88, 4] - [146, 5]
        (loop_expression [88, 4] - [146, 5]
          body: (block [88, 9] - [146, 5]
            (let_declaration [89, 8] - [89, 33]
              pattern: (identifier [89, 12] - [89, 16])
              value: (call_expression [89, 19] - [89, 32]
                function: (field_expression [89, 19] - [89, 30]
                  value: (identifier [89, 19] - [89, 25])
                  field: (field_identifier [89, 26] - [89, 30]))
                arguments: (arguments [89, 30] - [89, 32])))
            (let_declaration [90, 8] - [90, 39]
              pattern: (identifier [90, 12] - [90, 20])
              value: (call_expression [90, 23] - [90, 38]
                function: (field_expression [90, 23] - [90, 36]
                  value: (identifier [90, 23] - [90, 27])
                  field: (field_identifier [90, 28] - [90, 36]))
                arguments: (arguments [90, 36] - [90, 38])))
            (expression_statement [91, 8] - [145, 9]
              (if_expression [91, 8] - [145, 9]
                condition: (identifier [91, 11] - [91, 29])
                consequence: (block [91, 30] - [106, 9]
                  (expression_statement [92, 12] - [97, 13]
                    (if_expression [92, 12] - [97, 13]
                      condition: (identifier [92, 15] - [92, 23])
                      consequence: (block [92, 24] - [97, 13]
                        (expression_statement [93, 16] - [95, 19]
                          (call_expression [93, 16] - [95, 18]
                            function: (field_expression [93, 16] - [93, 49]
                              value: (call_expression [93, 16] - [93, 34]
                                function: (field_expression [93, 16] - [93, 28]
                                  value: (identifier [93, 16] - [93, 22])
                                  field: (field_identifier [93, 23] - [93, 28]))
                                arguments: (arguments [93, 28] - [93, 34]
                                  (string_literal [93, 29] - [93, 33])))
                              field: (field_identifier [93, 35] - [93, 49]))
                            arguments: (arguments [93, 49] - [95, 18]
                              (closure_expression [93, 50] - [95, 17]
                                parameters: (closure_parameters [93, 50] - [93, 55]
                                  (identifier [93, 51] - [93, 54]))
                                body: (block [93, 56] - [95, 17]
                                  (expression_statement [94, 20] - [94, 59]
                                    (macro_invocation [94, 20] - [94, 58]
                                      macro: (identifier [94, 20] - [94, 30])
                                      (token_tree [94, 31] - [94, 58]
                                        (string_literal [94, 32] - [94, 52])
                                        (identifier [94, 54] - [94, 57])))))))))
                        (expression_statement [96, 16] - [96, 37]
                          (assignment_expression [96, 16] - [96, 36]
                            left: (identifier [96, 16] - [96, 29])
                            right: (boolean_literal [96, 32] - [96, 36]))))))
                  (expression_statement [98, 12] - [105, 13]
                    (if_expression [98, 12] - [105, 13]
                      condition: (call_expression [98, 15] - [98, 41]
                        function: (field_expression [98, 15] - [98, 39]
                          value: (identifier [98, 15] - [98, 21])
                          field: (field_identifier [98, 22] - [98, 39]))
                        arguments: (arguments [98, 39] - [98, 41]))
                      consequence: (block [98, 42] - [100, 13]
                        (expression_statement [99, 16] - [99, 43]
                          (assignment_expression [99, 16] - [99, 42]
                            left: (identifier [99, 16] - [99, 34])
                            right: (boolean_literal [99, 37] - [99, 42]))))
                      alternative: (else_clause [100, 14] - [105, 13]
                        (if_expression [100, 19] - [105, 13]
                          condition: (call_expression [100, 22] - [100, 42]
                            function: (field_expression [100, 22] - [100, 40]
                              value: (identifier [100, 22] - [100, 28])
                              field: (field_identifier [100, 29] - [100, 40]))
                            arguments: (arguments [100, 40] - [100, 42]))
                          consequence: (block [100, 43] - [103, 13]
                            (expression_statement [101, 16] - [101, 42]
                              (assignment_expression [101, 16] - [101, 41]
                                left: (identifier [101, 16] - [101, 34])
                                right: (boolean_literal [101, 37] - [101, 41])))
                            (expression_statement [102, 16] - [102, 34]
                              (compound_assignment_expr [102, 16] - [102, 33]
                                left: (identifier [102, 16] - [102, 28])
                                right: (integer_literal [102, 32] - [102, 33]))))
                          alternative: (else_clause [103, 14] - [105, 13]
                            (block [103, 19] - [105, 13]
                              (expression_statement [104, 16] - [104, 22]
                                (break_expression [104, 16] - [104, 21])))))))))
                alternative: (else_clause [106, 10] - [145, 9]
                  (block [106, 15] - [145, 9]
                    (expression_statement [107, 12] - [138, 13]
                      (if_expression [107, 12] - [138, 13]
                        condition: (identifier [107, 15] - [107, 23])
                        consequence: (block [107, 24] - [138, 13]
                          (expression_statement [108, 16] - [112, 17]
                            (if_expression [108, 16] - [112, 17]
                              condition: (identifier [108, 19] - [108, 32])
                              consequence: (block [108, 33] - [112, 17]
                                (expression_statement [109, 20] - [111, 23]
                                  (call_expression [109, 20] - [111, 22]
                                    function: (field_expression [109, 20] - [109, 54]
                                      value: (call_expression [109, 20] - [109, 39]
                                        function: (field_expression [109, 20] - [109, 32]
                                          value: (identifier [109, 20] - [109, 26])
                                          field: (field_identifier [109, 27] - [109, 32]))
                                        arguments: (arguments [109, 32] - [109, 39]
                                          (string_literal [109, 33] - [109, 38]
                                            (escape_sequence [109, 35] - [109, 37]))))
                                      field: (field_identifier [109, 40] - [109, 54]))
                                    arguments: (arguments [109, 54] - [111, 22]
                                      (closure_expression [109, 55] - [111, 21]
                                        parameters: (closure_parameters [109, 55] - [109, 60]
                                          (identifier [109, 56] - [109, 59]))
                                        body: (block [109, 61] - [111, 21]
                                          (expression_statement [110, 24] - [110, 63]
                                            (macro_invocation [110, 24] - [110, 62]
                                              macro: (identifier [110, 24] - [110, 34])
                                              (token_tree [110, 35] - [110, 62]
                                                (string_literal [110, 36] - [110, 56])
                                                (identifier [110, 58] - [110, 61]))))))))))))
                          (expression_statement [113, 16] - [117, 17]
                            (for_expression [113, 16] - [117, 17]
                              value: (range_expression [113, 25] - [113, 40]
                                (integer_literal [113, 25] - [113, 26])
                                (identifier [113, 28] - [113, 40]))
                              body: (block [113, 41] - [117, 17]
                                (expression_statement [114, 20] - [116, 23]
                                  (call_expression [114, 20] - [116, 22]
                                    function: (field_expression [114, 20] - [114, 54]
                                      value: (call_expression [114, 20] - [114, 39]
                                        function: (field_expression [114, 20] - [114, 32]
                                          value: (identifier [114, 20] - [114, 26])
                                          field: (field_identifier [114, 27] - [114, 32]))
                                        arguments: (arguments [114, 32] - [114, 39]
                                          (string_literal [114, 33] - [114, 38])))
                                      field: (field_identifier [114, 40] - [114, 54]))
                                    arguments: (arguments [114, 54] - [116, 22]
                                      (closure_expression [114, 55] - [116, 21]
                                        parameters: (closure_parameters [114, 55] - [114, 60]
                                          (identifier [114, 56] - [114, 59]))
                                        body: (block [114, 61] - [116, 21]
                                          (expression_statement [115, 24] - [115, 63]
                                            (macro_invocation [115, 24] - [115, 62]
                                              macro: (identifier [115, 24] - [115, 34])
                                              (token_tree [115, 35] - [115, 62]
                                                (string_literal [115, 36] - [115, 56])
                                                (identifier [115, 58] - [115, 61]))))))))))))
                          (let_declaration [118, 16] - [118, 57]
                            pattern: (identifier [118, 20] - [118, 25])
                            type: (type_identifier [118, 27] - [118, 32])
                            value: (call_expression [118, 35] - [118, 56]
                              function: (field_expression [118, 35] - [118, 54]
                                value: (identifier [118, 35] - [118, 39])
                                field: (field_identifier [118, 40] - [118, 54]))
                              arguments: (arguments [118, 54] - [118, 56])))
                          (let_declaration [119, 16] - [119, 53]
                            pattern: (identifier [119, 20] - [119, 23])
                            type: (type_identifier [119, 25] - [119, 30])
                            value: (call_expression [119, 33] - [119, 52]
                              function: (field_expression [119, 33] - [119, 50]
                                value: (identifier [119, 33] - [119, 37])
                                field: (field_identifier [119, 38] - [119, 50]))
                              arguments: (arguments [119, 50] - [119, 52])))
                          (expression_statement [120, 16] - [124, 17]
                            (if_expression [120, 16] - [124, 17]
                              condition: (let_condition [120, 19] - [120, 61]
                                pattern: (tuple_struct_pattern [120, 23] - [120, 39]
                                  type: (identifier [120, 23] - [120, 27])
                                  (identifier [120, 28] - [120, 38]))
                                value: (call_expression [120, 42] - [120, 61]
                                  function: (field_expression [120, 42] - [120, 59]
                                    value: (identifier [120, 42] - [120, 48])
                                    field: (field_identifier [120, 49] - [120, 59]))
                                  arguments: (arguments [120, 59] - [120, 61])))
                              consequence: (block [120, 62] - [124, 17]
                                (expression_statement [121, 20] - [123, 23]
                                  (call_expression [121, 20] - [123, 22]
                                    function: (field_expression [121, 20] - [121, 74]
                                      value: (macro_invocation [121, 20] - [121, 59]
                                        macro: (identifier [121, 20] - [121, 25])
                                        (token_tree [121, 26] - [121, 59]
                                          (mutable_specifier [121, 28] - [121, 31])
                                          (identifier [121, 32] - [121, 38])
                                          (string_literal [121, 40] - [121, 46])
                                          (identifier [121, 48] - [121, 58])))
                                      field: (field_identifier [121, 60] - [121, 74]))
                                    arguments: (arguments [121, 74] - [123, 22]
                                      (closure_expression [121, 75] - [123, 21]
                                        parameters: (closure_parameters [121, 75] - [121, 80]
                                          (identifier [121, 76] - [121, 79]))
                                        body: (block [121, 81] - [123, 21]
                                          (expression_statement [122, 24] - [122, 63]
                                            (macro_invocation [122, 24] - [122, 62]
                                              macro: (identifier [122, 24] - [122, 34])
                                              (token_tree [122, 35] - [122, 62]
                                                (string_literal [122, 36] - [122, 56])
                                                (identifier [122, 58] - [122, 61]))))))))))))
                          (expression_statement [125, 16] - [136, 19]
                            (call_expression [125, 16] - [136, 18]
                              function: (field_expression [125, 16] - [134, 31]
                                value: (macro_invocation [125, 16] - [133, 17]
                                  macro: (identifier [125, 16] - [125, 21])
                                  (token_tree [125, 22] - [133, 17]
                                    (mutable_specifier [126, 21] - [126, 24])
                                    (identifier [126, 25] - [126, 31])
                                    (string_literal [127, 20] - [127, 45])
                                    (identifier [128, 20] - [128, 24])
                                    (identifier [128, 25] - [128, 29])
                                    (token_tree [128, 29] - [128, 31])
                                    (identifier [129, 20] - [129, 25])
                                    (identifier [129, 26] - [129, 29])
                                    (identifier [130, 20] - [130, 25])
                                    (identifier [130, 26] - [130, 32])
                                    (identifier [131, 20] - [131, 23])
                                    (identifier [131, 24] - [131, 27])
                                    (identifier [132, 20] - [132, 23])
                                    (identifier [132, 24] - [132, 30])))
                                field: (field_identifier [134, 17] - [134, 31]))
                              arguments: (arguments [134, 31] - [136, 18]
                                (closure_expression [134, 32] - [136, 17]
                                  parameters: (closure_parameters [134, 32] - [134, 37]
                                    (identifier [134, 33] - [134, 36]))
                                  body: (block [134, 38] - [136, 17]
                                    (expression_statement [135, 20] - [135, 59]
                                      (macro_invocation [135, 20] - [135, 58]
                                        macro: (identifier [135, 20] - [135, 30])
                                        (token_tree [135, 31] - [135, 58]
                                          (string_literal [135, 32] - [135, 52])
                                          (identifier [135, 54] - [135, 57])))))))))
                          (expression_statement [137, 16] - [137, 37]
                            (assignment_expression [137, 16] - [137, 36]
                              left: (identifier [137, 16] - [137, 29])
                              right: (boolean_literal [137, 32] - [137, 36]))))))
                    (expression_statement [139, 12] - [144, 13]
                      (if_expression [139, 12] - [144, 13]
                        condition: (call_expression [139, 15] - [139, 40]
                          function: (field_expression [139, 15] - [139, 38]
                            value: (identifier [139, 15] - [139, 21])
                            field: (field_identifier [139, 22] - [139, 38]))
                          arguments: (arguments [139, 38] - [139, 40]))
                        consequence: (block [139, 41] - [142, 13]
                          (expression_statement [140, 16] - [140, 43]
                            (assignment_expression [140, 16] - [140, 42]
                              left: (identifier [140, 16] - [140, 34])
                              right: (boolean_literal [140, 37] - [140, 42])))
                          (expression_statement [141, 16] - [141, 34]
                            (compound_assignment_expr [141, 16] - [141, 33]
                              left: (identifier [141, 16] - [141, 28])
                              right: (integer_literal [141, 32] - [141, 33]))))
                        alternative: (else_clause [142, 14] - [144, 13]
                          (block [142, 19] - [144, 13]
                            (expression_statement [143, 16] - [143, 42]
                              (assignment_expression [143, 16] - [143, 41]
                                left: (identifier [143, 16] - [143, 34])
                                right: (boolean_literal [143, 37] - [143, 41]))))))))))))))
      (expression_statement [147, 4] - [147, 35]
        (call_expression [147, 4] - [147, 34]
          function: (field_expression [147, 4] - [147, 16]
            value: (identifier [147, 4] - [147, 10])
            field: (field_identifier [147, 11] - [147, 16]))
          arguments: (arguments [147, 16] - [147, 34]
            (call_expression [147, 17] - [147, 33]
              function: (field_expression [147, 17] - [147, 31]
                value: (identifier [147, 17] - [147, 21])
                field: (field_identifier [147, 22] - [147, 31]))
              arguments: (arguments [147, 31] - [147, 33])))))
      (expression_statement [148, 4] - [148, 17]
        (macro_invocation [148, 4] - [148, 16]
          macro: (identifier [148, 4] - [148, 11])
          (token_tree [148, 12] - [148, 16]
            (string_literal [148, 13] - [148, 15]))))
      (let_declaration [150, 4] - [150, 31]
        (mutable_specifier [150, 8] - [150, 11])
        pattern: (identifier [150, 12] - [150, 23])
        value: (identifier [150, 26] - [150, 30]))
      (expression_statement [151, 4] - [165, 5]
        (loop_expression [151, 4] - [165, 5]
          body: (block [151, 9] - [165, 5]
            (let_declaration [152, 8] - [152, 33]
              pattern: (identifier [152, 12] - [152, 16])
              value: (call_expression [152, 19] - [152, 32]
                function: (field_expression [152, 19] - [152, 30]
                  value: (identifier [152, 19] - [152, 25])
                  field: (field_identifier [152, 26] - [152, 30]))
                arguments: (arguments [152, 30] - [152, 32])))
            (expression_statement [153, 8] - [164, 9]
              (if_expression [153, 8] - [164, 9]
                condition: (call_expression [153, 11] - [153, 27]
                  function: (field_expression [153, 11] - [153, 25]
                    value: (identifier [153, 11] - [153, 15])
                    field: (field_identifier [153, 16] - [153, 25]))
                  arguments: (arguments [153, 25] - [153, 27]))
                consequence: (block [153, 28] - [162, 9]
                  (expression_statement [154, 12] - [161, 13]
                    (if_expression [154, 12] - [161, 13]
                      condition: (binary_expression [154, 15] - [154, 51]
                        left: (call_expression [154, 15] - [154, 30]
                          function: (field_expression [154, 15] - [154, 28]
                            value: (identifier [154, 15] - [154, 19])
                            field: (field_identifier [154, 20] - [154, 28]))
                          arguments: (arguments [154, 28] - [154, 30]))
                        right: (call_expression [154, 34] - [154, 51]
                          function: (field_expression [154, 34] - [154, 49]
                            value: (identifier [154, 34] - [154, 38])
                            field: (field_identifier [154, 39] - [154, 49]))
                          arguments: (arguments [154, 49] - [154, 51])))
                      consequence: (block [154, 52] - [157, 13]
                        (expression_statement [155, 16] - [155, 41]
                          (assignment_expression [155, 16] - [155, 40]
                            left: (identifier [155, 16] - [155, 27])
                            right: (call_expression [155, 30] - [155, 40]
                              function: (identifier [155, 30] - [155, 34])
                              arguments: (arguments [155, 34] - [155, 40]
                                (identifier [155, 35] - [155, 39])))))
                        (expression_statement [156, 16] - [156, 22]
                          (break_expression [156, 16] - [156, 21])))
                      alternative: (else_clause [157, 14] - [161, 13]
                        (block [157, 19] - [161, 13]
                          (expression_statement [158, 16] - [160, 17]
                            (if_expression [158, 16] - [160, 17]
                              condition: (unary_expression [158, 19] - [158, 45]
                                (call_expression [158, 20] - [158, 45]
                                  function: (field_expression [158, 20] - [158, 43]
                                    value: (identifier [158, 20] - [158, 26])
                                    field: (field_identifier [158, 27] - [158, 43]))
                                  arguments: (arguments [158, 43] - [158, 45])))
                              consequence: (block [158, 46] - [160, 17]
                                (expression_statement [159, 20] - [159, 26]
                                  (break_expression [159, 20] - [159, 25]))))))))))
                alternative: (else_clause [162, 10] - [164, 9]
                  (if_expression [162, 15] - [164, 9]
                    condition: (unary_expression [162, 18] - [162, 45]
                      (call_expression [162, 19] - [162, 45]
                        function: (field_expression [162, 19] - [162, 43]
                          value: (identifier [162, 19] - [162, 25])
                          field: (field_identifier [162, 26] - [162, 43]))
                        arguments: (arguments [162, 43] - [162, 45])))
                    consequence: (block [162, 46] - [164, 9]
                      (expression_statement [163, 12] - [163, 18]
                        (break_expression [163, 12] - [163, 17]))))))))))
      (expression_statement [167, 4] - [207, 5]
        (if_expression [167, 4] - [207, 5]
          condition: (call_expression [167, 7] - [167, 28]
            function: (field_expression [167, 7] - [167, 26]
              value: (identifier [167, 7] - [167, 18])
              field: (field_identifier [167, 19] - [167, 26]))
            arguments: (arguments [167, 26] - [167, 28]))
          consequence: (block [167, 29] - [207, 5]
            (expression_statement [168, 8] - [206, 9]
              (if_expression [168, 8] - [206, 9]
                condition: (let_condition [168, 11] - [168, 39]
                  pattern: (tuple_struct_pattern [168, 15] - [168, 25]
                    type: (identifier [168, 15] - [168, 19])
                    (identifier [168, 20] - [168, 24]))
                  value: (identifier [168, 28] - [168, 39]))
                consequence: (block [168, 40] - [206, 9]
                  (let_declaration [169, 12] - [169, 46]
                    pattern: (identifier [169, 16] - [169, 21])
                    value: (call_expression [169, 24] - [169, 45]
                      function: (field_expression [169, 24] - [169, 43]
                        value: (identifier [169, 24] - [169, 28])
                        field: (field_identifier [169, 29] - [169, 43]))
                      arguments: (arguments [169, 43] - [169, 45])))
                  (let_declaration [170, 12] - [170, 42]
                    pattern: (identifier [170, 16] - [170, 19])
                    value: (call_expression [170, 22] - [170, 41]
                      function: (field_expression [170, 22] - [170, 39]
                        value: (identifier [170, 22] - [170, 26])
                        field: (field_identifier [170, 27] - [170, 39]))
                      arguments: (arguments [170, 39] - [170, 41])))
                  (expression_statement [171, 12] - [173, 15]
                    (call_expression [171, 12] - [173, 14]
                      function: (field_expression [171, 12] - [171, 53]
                        value: (macro_invocation [171, 12] - [171, 38]
                          macro: (identifier [171, 12] - [171, 17])
                          (token_tree [171, 18] - [171, 38]
                            (mutable_specifier [171, 20] - [171, 23])
                            (identifier [171, 24] - [171, 30])
                            (string_literal [171, 32] - [171, 37]
                              (escape_sequence [171, 33] - [171, 35]))))
                        field: (field_identifier [171, 39] - [171, 53]))
                      arguments: (arguments [171, 53] - [173, 14]
                        (closure_expression [171, 54] - [173, 13]
                          parameters: (closure_parameters [171, 54] - [171, 59]
                            (identifier [171, 55] - [171, 58]))
                          body: (block [171, 60] - [173, 13]
                            (expression_statement [172, 16] - [172, 55]
                              (macro_invocation [172, 16] - [172, 54]
                                macro: (identifier [172, 16] - [172, 26])
                                (token_tree [172, 27] - [172, 54]
                                  (string_literal [172, 28] - [172, 48])
                                  (identifier [172, 50] - [172, 53])))))))))
                  (expression_statement [174, 12] - [193, 13]
                    (if_expression [174, 12] - [193, 13]
                      condition: (call_expression [174, 15] - [174, 32]
                        function: (field_expression [174, 15] - [174, 30]
                          value: (identifier [174, 15] - [174, 19])
                          field: (field_identifier [174, 20] - [174, 30]))
                        arguments: (arguments [174, 30] - [174, 32]))
                      consequence: (block [174, 33] - [189, 13]
                        (expression_statement [175, 16] - [188, 17]
                          (if_expression [175, 16] - [188, 17]
                            condition: (call_expression [175, 19] - [175, 34]
                              function: (field_expression [175, 19] - [175, 32]
                                value: (identifier [175, 19] - [175, 23])
                                field: (field_identifier [175, 24] - [175, 32]))
                              arguments: (arguments [175, 32] - [175, 34]))
                            consequence: (block [175, 35] - [179, 17]
                              (expression_statement [176, 20] - [178, 23]
                                (call_expression [176, 20] - [178, 22]
                                  function: (field_expression [176, 20] - [176, 81]
                                    value: (macro_invocation [176, 20] - [176, 66]
                                      macro: (identifier [176, 20] - [176, 25])
                                      (token_tree [176, 26] - [176, 66]
                                        (mutable_specifier [176, 28] - [176, 31])
                                        (identifier [176, 32] - [176, 38])
                                        (string_literal [176, 40] - [176, 52])
                                        (identifier [176, 54] - [176, 58])
                                        (identifier [176, 59] - [176, 63])
                                        (token_tree [176, 63] - [176, 65])))
                                    field: (field_identifier [176, 67] - [176, 81]))
                                  arguments: (arguments [176, 81] - [178, 22]
                                    (closure_expression [176, 82] - [178, 21]
                                      parameters: (closure_parameters [176, 82] - [176, 87]
                                        (identifier [176, 83] - [176, 86]))
                                      body: (block [176, 88] - [178, 21]
                                        (expression_statement [177, 24] - [177, 63]
                                          (macro_invocation [177, 24] - [177, 62]
                                            macro: (identifier [177, 24] - [177, 34])
                                            (token_tree [177, 35] - [177, 62]
                                              (string_literal [177, 36] - [177, 56])
                                              (identifier [177, 58] - [177, 61]))))))))))
                            alternative: (else_clause [179, 18] - [188, 17]
                              (block [179, 23] - [188, 17]
                                (expression_statement [180, 20] - [187, 23]
                                  (call_expression [180, 20] - [187, 22]
                                    function: (field_expression [180, 20] - [185, 35]
                                      value: (macro_invocation [180, 20] - [184, 21]
                                        macro: (identifier [180, 20] - [180, 25])
                                        (token_tree [180, 26] - [184, 21]
                                          (mutable_specifier [181, 25] - [181, 28])
                                          (identifier [181, 29] - [181, 35])
                                          (string_literal [182, 24] - [182, 40]
                                            (escape_sequence [182, 33] - [182, 35])
                                            (escape_sequence [182, 37] - [182, 39]))
                                          (identifier [183, 24] - [183, 28])
                                          (identifier [183, 29] - [183, 33])
                                          (token_tree [183, 33] - [183, 35])
                                          (identifier [183, 36] - [183, 43])
                                          (token_tree [183, 43] - [183, 56]
                                            (string_literal [183, 44] - [183, 48]
                                              (escape_sequence [183, 45] - [183, 47]))
                                            (string_literal [183, 50] - [183, 55]
                                              (escape_sequence [183, 51] - [183, 53])))))
                                      field: (field_identifier [185, 21] - [185, 35]))
                                    arguments: (arguments [185, 35] - [187, 22]
                                      (closure_expression [185, 36] - [187, 21]
                                        parameters: (closure_parameters [185, 36] - [185, 41]
                                          (identifier [185, 37] - [185, 40]))
                                        body: (block [185, 42] - [187, 21]
                                          (expression_statement [186, 24] - [186, 63]
                                            (macro_invocation [186, 24] - [186, 62]
                                              macro: (identifier [186, 24] - [186, 34])
                                              (token_tree [186, 35] - [186, 62]
                                                (string_literal [186, 36] - [186, 56])
                                                (identifier [186, 58] - [186, 61]))))))))))))))
                      alternative: (else_clause [189, 14] - [193, 13]
                        (block [189, 19] - [193, 13]
                          (expression_statement [190, 16] - [192, 19]
                            (call_expression [190, 16] - [192, 18]
                              function: (field_expression [190, 16] - [190, 69]
                                value: (macro_invocation [190, 16] - [190, 54]
                                  macro: (identifier [190, 16] - [190, 21])
                                  (token_tree [190, 22] - [190, 54]
                                    (mutable_specifier [190, 24] - [190, 27])
                                    (identifier [190, 28] - [190, 34])
                                    (string_literal [190, 36] - [190, 40])
                                    (identifier [190, 42] - [190, 46])
                                    (identifier [190, 47] - [190, 51])
                                    (token_tree [190, 51] - [190, 53])))
                                field: (field_identifier [190, 55] - [190, 69]))
                              arguments: (arguments [190, 69] - [192, 18]
                                (closure_expression [190, 70] - [192, 17]
                                  parameters: (closure_parameters [190, 70] - [190, 75]
                                    (identifier [190, 71] - [190, 74]))
                                  body: (block [190, 76] - [192, 17]
                                    (expression_statement [191, 20] - [191, 59]
                                      (macro_invocation [191, 20] - [191, 58]
                                        macro: (identifier [191, 20] - [191, 30])
                                        (token_tree [191, 31] - [191, 58]
                                          (string_literal [191, 32] - [191, 52])
                                          (identifier [191, 54] - [191, 57])))))))))))))
                  (expression_statement [194, 12] - [201, 15]
                    (call_expression [194, 12] - [201, 14]
                      function: (field_expression [194, 12] - [199, 27]
                        value: (macro_invocation [194, 12] - [198, 13]
                          macro: (identifier [194, 12] - [194, 17])
                          (token_tree [194, 18] - [198, 13]
                            (mutable_specifier [195, 17] - [195, 20])
                            (identifier [195, 21] - [195, 27])
                            (string_literal [196, 16] - [196, 39])
                            (identifier [197, 16] - [197, 21])
                            (identifier [197, 22] - [197, 25])
                            (identifier [197, 27] - [197, 32])
                            (identifier [197, 33] - [197, 39])
                            (identifier [197, 41] - [197, 44])
                            (identifier [197, 45] - [197, 48])
                            (identifier [197, 50] - [197, 53])
                            (identifier [197, 54] - [197, 60])))
                        field: (field_identifier [199, 13] - [199, 27]))
                      arguments: (arguments [199, 27] - [201, 14]
                        (closure_expression [199, 28] - [201, 13]
                          parameters: (closure_parameters [199, 28] - [199, 33]
                            (identifier [199, 29] - [199, 32]))
                          body: (block [199, 34] - [201, 13]
                            (expression_statement [200, 16] - [200, 55]
                              (macro_invocation [200, 16] - [200, 54]
                                macro: (identifier [200, 16] - [200, 26])
                                (token_tree [200, 27] - [200, 54]
                                  (string_literal [200, 28] - [200, 48])
                                  (identifier [200, 50] - [200, 53])))))))))
                  (expression_statement [202, 12] - [204, 15]
                    (call_expression [202, 12] - [204, 14]
                      function: (field_expression [202, 12] - [202, 52]
                        value: (macro_invocation [202, 12] - [202, 37]
                          macro: (identifier [202, 12] - [202, 17])
                          (token_tree [202, 18] - [202, 37]
                            (mutable_specifier [202, 20] - [202, 23])
                            (identifier [202, 24] - [202, 30])
                            (string_literal [202, 32] - [202, 36]
                              (escape_sequence [202, 33] - [202, 35]))))
                        field: (field_identifier [202, 38] - [202, 52]))
                      arguments: (arguments [202, 52] - [204, 14]
                        (closure_expression [202, 53] - [204, 13]
                          parameters: (closure_parameters [202, 53] - [202, 58]
                            (identifier [202, 54] - [202, 57]))
                          body: (block [202, 59] - [204, 13]
                            (expression_statement [203, 16] - [203, 55]
                              (macro_invocation [203, 16] - [203, 54]
                                macro: (identifier [203, 16] - [203, 26])
                                (token_tree [203, 27] - [203, 54]
                                  (string_literal [203, 28] - [203, 48])
                                  (identifier [203, 50] - [203, 53])))))))))
                  (expression_statement [205, 12] - [205, 42]
                    (macro_invocation [205, 12] - [205, 41]
                      macro: (identifier [205, 12] - [205, 22])
                      (token_tree [205, 23] - [205, 41]
                        (string_literal [205, 24] - [205, 40])))))))))))))
```