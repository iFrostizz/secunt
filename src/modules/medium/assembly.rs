// Module that finds for external and dangerous calls

use crate::build_visitor;

build_visitor!(
    BTreeMap::from([
        (0,
        FindingKey {
            description: "using extcodesize. Can be an issue if determining if EOA.".to_string(),
            summary: "extcodesize for EOA test".to_string(),
            severity: Severity::Medium
        }),
        (1,
         FindingKey {
             summary: "No need to use assembly for available method".to_string(),
             description: "Some methods that are accessed using assembly can be accessed by high level methods. It is thus no need the risk of using assembly, which may flag some static analyzers or shoot yourself in the foot.".to_string(),
             severity: Severity::Informal,
         })
    ]),

    fn visit_yul_identifier(&mut self, yul_identifier: &mut YulIdentifier) {
        let n = yul_identifier.name.as_str();
        if ["extcodesize", "caller", "callvalue", "calldatasize", "extcodesize", "chainid", "gas", "extcodehash", "selfbalance", "address"].contains(&n) {
            self.push_finding(1, Some(yul_identifier.src.clone()));

            if n == "extcodesize" {
                self.push_finding(0, Some(yul_identifier.src.clone()));
            }
        }

        yul_identifier.visit(self)
    }
);

#[test]
fn with_extcodesize() {
    let findings = compile_and_get_findings(vec![ProjectFile::Contract(
        String::from("ExtCodeSize"),
        String::from(
            "pragma solidity ^0.8.0;

contract ExtCodeSize {
    function make(address to) public {
        uint256 size;
            
        assembly {
            size := extcodesize(to)
        }
    }
}",
        ),
    )]);

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "assembly", 0), // extcodesize
        vec![8]
    );
}

#[test]
fn without_extcodesize() {
    let findings = compile_and_get_findings(vec![ProjectFile::Contract(
        String::from("WithoutExtCodeSize"),
        String::from(
            "pragma solidity ^0.8.0;

contract WithoutExtCodeSize {
    function make(address to) public {
        uint256 bal;
            
        assembly {
            bal := balance(to)
        }
    }
}",
        ),
    )]);

    assert!(!has_with_code(&findings, "assembly", 0)); // extcodesize);
}

#[test]
fn nested_extcodesize() {
    let findings = compile_and_get_findings(vec![ProjectFile::Contract(
        String::from("NestedExtCodeSize"),
        String::from(
            "pragma solidity ^0.8.0;

contract NestedExtCodeSize {
    function make(address to) public {
        uint256 size;
            
        assembly {
            for { let i:= 0 } lt(i, 10) { i := add(i, 1) } {
                size := extcodesize(to)
            }
        }
    }
}",
        ),
    )]);

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "assembly", 0), // extcodesize
        vec![9]
    );
}

#[test]
fn assembly_meth() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity ^0.8.0;

contract Assembly {
    function make(address to) public {
        assembly {
            let id := chainid()
            let cds := calldatasize()
            // let h := keccak256()
            let g := gas()
            let addr := address()
            let size := extcodesize(addr)
            let hash := extcodehash(addr)
            let bal := selfbalance()
        }
    }
}",
    ));

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "assembly", 1), // extcodesize
        vec![6, 7, 9, 10, 11, 12, 13]
    );
}
