// https://github.com/code-423n4/2022-12-tigris-findings/blob/main/data/JC-G.md#g02-state-variables-that-never-change-should-be-declared-immutable-or-constant

use crate::build_visitor;

build_visitor! {
    BTreeMap::from([
        (
            0,
            FindingKey {
                summary: "State variables that never change should be directly inlined in the bytecode".to_string(),
                description: "When state variables are guaranteed to never change, they should be inlined in the bytecode of the contract by declaring them as immutables or constants to avoid paying the upfront cost of SLOAD which are expensive, mainly when the slot is cold.".to_string(),
                severity: Severity::Gas,
            }
        )
    ]),

    fn visit_source_unit(&mut self, source_unit: &mut SourceUnit) {
        // get state variables and assignment
        source_unit.visit(self)?;
        // check if assignments are done on state variables out of constructor

        Ok(())
    },

    fn visit_variable_declaration(&mut self, variable_declaration: &mut VariableDeclaration) {
        if variable_declaration.state_variable {
            self.state_variables.push(variable_declaration.name.clone());
        }

        variable_declaration.visit(self)
    },

    fn visit_function_definition(&mut self, function_definition: &mut FunctionDefinition) {
        if function_definition.kind == Some(FunctionKind::Constructor) {
            self.inside.constructor = true;
        }

        function_definition.visit(self)?;

        self.inside.constructor = true;

        Ok(())
    },

    fn visit_assignment(&mut self, assignment: &mut Assignment) {
        if let Expression::Identifier(identifier) = &assignment.lhs {
            if self.state_variables.contains(&identifier.name) && !self.inside.constructor {
                self.push_finding(0, Some(assignment.src.clone()));
            }
        }

        assignment.visit(self)
    }
}

#[test]
fn not_changing() {
    let findings = compile_and_get_findings(vec![ProjectFile::Contract(
        String::from("NotChange"),
        String::from(
            "pragma solidity 0.8.0;

contract NotChange {
    string public baseURI;

    constructor(string memory _baseURI) {
        baseURI = _baseURI;
    }
}",
        ),
    )]);

    assert_eq!(
        lines_for_findings_with_code(&findings, "immutable", 0),
        vec![4]
    );
}

#[test]
fn changing() {
    let findings = compile_and_get_findings(vec![ProjectFile::Contract(
        String::from("Changes"),
        String::from(
            "pragma solidity 0.8.0;

contract Changes {
    string public baseURI;

    constructor(string memory _baseURI) {
        baseURI = _baseURI;
    }

    function setURI(string memory _baseURI) public {
        baseURI = _baseURI;
    }
}",
        ),
    )]);

    assert!(!has_with_code(&findings, "immutable", 0));
}
