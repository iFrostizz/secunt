use crate::build_visitor;
use crate::utils::type_as_bytes;

build_visitor! {
    BTreeMap::from([
        (
            0,
            FindingKey {
                summary: "Unsafe downcast".to_string(),
                description: "When downcasting to a smaller type, Solidity doesn't write any runtime check which may lead to bugs.".to_string(),
                severity: Severity::Low,
            }
        )
    ]),

    fn visit_function_call(&mut self, fcall: &mut FunctionCall) {
        if fcall.kind == FunctionCallKind::TypeConversion {
            let arguments = &fcall.arguments;
            if arguments.len() == 1 {
                let to_cast = fcall.type_descriptions.type_string.clone().unwrap_or_default();
                let to_bytes = type_as_bytes(&to_cast) ;

                let from_cast = if let Expression::Identifier(id) = &arguments[0] {
                    id.type_descriptions.type_string.clone().unwrap_or_default()
                } else if let Expression::MemberAccess(ma) = &arguments[0] {
                    ma.type_descriptions.type_string.clone().unwrap_or_default()
                } else {
                    return Ok(());
                };

                let from_bytes = type_as_bytes(&from_cast);
                if from_bytes > to_bytes {
                    self.push_finding(0, Some(fcall.src.clone()));
                }
            }
        }
        Ok(())
    }
}

#[test]
fn unsafe_downcast() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity 0.8.0;

contract Down {
    function down() public {
        uint256 x = 2**234;
        uint32 time = uint32(block.timestamp);
        uint32 number = uint32(block.number);
        uint48 num = uint48(x);
        uint256 big = uint48(num);
    }
}",
    ));

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "downcast", 0),
        [6, 7, 8]
    );
}
