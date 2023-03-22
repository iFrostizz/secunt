use crate::build_visitor;

build_visitor! {
    BTreeMap::from([
        (
            0,
            FindingKey {
                summary: "Local variable assignment".to_string(),
                description: "Catch frequently used storage variables in memory/stack, converting multiple SLOAD into 1 SLOAD.".to_string(),
                severity: Severity::Gas
            }
        )
    ]),

    fn visit_variable_declaration(&mut self, var: &mut VariableDeclaration) {
        self.id_var.insert(var.id, var.clone());

        Ok(())
    },

    fn visit_for_statement(&mut self, fors: &mut ForStatement) {
        if let Some(Expression::BinaryOperation(bo)) = &fors.condition {
            if let Expression::Identifier(rhs) = &bo.rhs {
                if let Some(id) = rhs.referenced_declaration {
                    if let Some(var) = self.id_var.get(&(id as usize)) {
                        if var.mutability() == &Mutability::Mutable && var.state_variable {
                            self.push_finding(0, Some(fors.src.clone()));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[test]
fn storage_len_loop() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity ^0.8.0;

contract StorLoop {
    uint256 length = 10;

    function loop() public {
        for (uint256 i = 0; i < length; i++) {
            // do something here
        }
    }
}",
    ));

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "loops", 0),
        vec![7]
    );
}

#[test]
fn stack_len_loop() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity ^0.8.0;

contract StorLoop {
    uint256 length = 10;

    function loop() public {
        uint256 l = length;

        for (uint256 i = 0; i < l; i++) {
            // do something here
        }
    }
}",
    ));

    assert!(!has_with_code(&findings, "loops", 0),);
}
