// Make sure that the code style is good, e.g.
// No remaining TODOs: https://code4rena.com/reports/2022-06-badger/#n-02-open-todos
// hardhat's console.log

use crate::build_visitor;
use std::fs;

build_visitor!(
    BTreeMap::from([
        (
            0,
             FindingKey {
                 summary: "Consider using named mappings".to_string(),
                 description: "From 0.8.18, Solidity introduced named mappings. They make it much easier for pair of eyes to know what they are returning. Please consider using them if you can.".to_string(),
                 severity: Severity::Informal
             }
         ),
         (
             1,
             FindingKey {
                 summary: "Storage variables should start with `s_`".to_string(),
                 description: "Prepending all storage variables with a `s_` makes them easier to find them. Storage variables may be crucial for security and thus is a good code style recommendation.".to_string(),
                 severity: Severity::Informal
             }
         ),
         (
             2,
             FindingKey {
                 summary: "Non external/public variable should begin with an underscore".to_string(),
                 description: "Any variable name that is not visible to the world should begin with an underscore because it facilitates readability of the code.".to_string(),
                 severity: Severity::Informal
             }
         ),
         (
             3,
             FindingKey {
                 summary: "Non external/public function names should begin with an underscore".to_string(),
                 description: "Any function name that is not visible to the world should begin with an underscore because it facilitates readability of the code.".to_string(),
                 severity: Severity::Informal
             }
         ),
         (
             4,
            FindingKey {
                summary: "Consider write the constants on the left part of the conditionals".to_string(),
                description: "Usually, some errors or typos may be done by writing the constant on the right part of a contional operator. This is because the human is rather used to compare a base to a term rather than the other way around.".to_string(),
                severity: Severity::Informal
            }
         ),
         (
            5,
            FindingKey {
                summary: "Avoid creating error without help".to_string(),
                description: "It is considered better practice to include to parameters to custom errors because they are usually needed for clarity purposes.".to_string(),
                severity: Severity::Informal
            }
         ),
         (
             6,
             FindingKey {
                 summary: "Avoid using `uint`, `int`, ...".to_string(),
                 description: "`uint` is not a very clear variable name, because it implicitely means uint256. Prefer using `uint256` instead.".to_string(),
                 severity: Severity::Informal
             }
         ),
         (
             7,
             FindingKey {
                 summary: "Consider writing `private` explicitely".to_string(),
                 description: "It is considered a better code practice to explicitely declare private variables. In Solidity, when the visibility of state variables is not specified, it is compiled as private.".to_string(),
                 severity: Severity::Informal
             }
         ),
         (
             8,
             FindingKey {
                 summary: "Indentation level".to_string(),
                 description: "From the Solidity Style Guide, a 4 spaces indentation level should be [preferred](https://docs.soliditylang.org/en/v0.8.17/style-guide.html#indentation).".to_string(),
                 severity: Severity::Informal
             }
         )
    ]),

    fn visit_source_unit(&mut self, su: &mut SourceUnit) {
        if let Ok(content) = fs::read_to_string(&su.absolute_path) {
            let mut flagged_width = false;
            let mut last_tab: usize = 0;
            for line in content.lines() {
                if !flagged_width {
                    let trimmed = line.trim_start();
                    let tab_size = line.len() - trimmed.len();
                    let shift = usize::abs_diff(last_tab, tab_size);
                    if shift != 4 && shift != 0 {
                        // TODO: try to find a way to display the full line
                        self.push_finding(8, None);
                        flagged_width = true;
                    }
                    last_tab = tab_size;
                }
            }
        };

        su.visit(self)
    },

    fn visit_variable_declaration(&mut self, vd: &mut VariableDeclaration) {
        if vd.state_variable && !vd.name.starts_with("s_") {
            self.push_finding(1, Some(vd.src.clone()));
        }

        if (vd.visibility == Visibility::Private || vd.visibility == Visibility::Internal) && !vd.name.starts_with('_') {
            self.push_finding(2, Some(vd.src.clone()));
        }

        if let Some(TypeName::ElementaryTypeName(tn)) = &vd.type_name {
            let name = &tn.name;
            if name == "int" || name == "uint" {
                self.push_finding(6, Some(vd.src.clone()));
            }
        }


        Ok(())
    },

    fn visit_function_definition(&mut self, fd: &mut FunctionDefinition) {
        if (fd.visibility == Visibility::Private || fd.visibility == Visibility::Internal) && !fd.name.starts_with('_') {
            self.push_finding(3, Some(fd.src.clone()));
        }

        fd.visit(self)
    },

    fn visit_if_statement(&mut self, is: &mut IfStatement) {
        // dbg!(&is);
        is.visit(self)
    },

    fn visit_error_definition(&mut self, ed: &mut ErrorDefinition) {
        if ed.parameters.parameters.is_empty() {
            self.push_finding(5, Some(ed.src.clone()));
        }
        ed.visit(self)
    }
);

// TODO: named mappings not supported by the AST
// #[test]
// fn named_mappings() {
//     let findings = compile_contract_and_get_findings(String::from(
//         "pragma solidity 0.8.20;

// contract NamedMappings {
//     mapping(address contractAddress => uint numOfFunctions) public cst_is_goated;

//     mapping(address => uint) public anonymous_mapping;
// }",
//     ));

//     assert_eq!(
//         lines_for_findings_with_code_module(&findings, "style", 0),
//         [6]
//     );
// }

#[test]
fn storage_var() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity 0.8.0;

contract SStorage {
    uint256 some_storage;
    uint256 s_some_storage;
    address hello;
    uint256 s_amba;
}",
    ));

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "style", 1),
        [4, 6]
    );
}

// TODO: stupid AST don't tell if implicitely private
// #[test]
// fn inv_var_func() {
//     let findings = compile_contract_and_get_findings(String::from(
//         "pragma solidity 0.8.0;

// contract InvVar {
//     uint256 public _hello;
//     uint256 _hi;
//     uint256 hi;
//     uint256 private hello;

//     function pub() public {}
//     function _inv() private {}
//     function priv() private {}
//     function inter() internal {}
// }",
//     ));

//     assert_eq!(
//         lines_for_findings_with_code_module(&findings, "style", 2),
//         [6, 7]
//     );
//     assert_eq!(
//         lines_for_findings_with_code_module(&findings, "style", 3),
//         [11, 12]
//     );
//     assert_eq!(
//         lines_for_findings_with_code_module(&findings, "style", 7),
//         [5, 6]
//     );
// }

// TODO: this stupid AST doesn't resolve any constant value
// #[test]
// fn compare() {
//     let findings = compile_contract_and_get_findings(String::from(
//         "pragma solidity 0.8.0;

// contract Cst {
//     uint256 constant y = 7123981763813;

//     function comp(uint256 x) public {
//         if (x == y) {}
//         else if (x != y) {}
//         else if (y > x) {}
//         else if (y == x) {}
//         else if (x^2 == y) {}
//     }
// }",
//     ));

//     assert_eq!(
//         lines_for_findings_with_code_module(&findings, "style", 4),
//         [7, 8, 11]
//     );
// }

#[test]
fn empty_error() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity 0.8.4;

contract Hey{
    error Problem();
    error Good(address);
}",
    ));

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "style", 5),
        [4]
    );
}

#[test]
fn unclear_type() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity 0.8.0;

contract Yo {
    uint256 val;
    uint hi;
    uint8 nope;
    int a;
}",
    ));

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "style", 6),
        [5, 7]
    );
    assert!(!has_with_code(&findings, "style", 8));
}

#[test]
fn tab_size() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity 0.8.0;

contract Tabs {
        function long_tabs() public {
                // Something went wrong!
        }
}",
    ));

    assert!(has_with_code(&findings, "style", 8));
}
