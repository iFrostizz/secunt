use crate::build_visitor;
use crate::utils::is_power_of_2;

build_visitor! {
    BTreeMap::from([
        (
            0,
            FindingKey {
                summary: "Bitwise shifting operator can be used".to_string(),
                description: "When multiplying or dividing by a constant that is a multiple of 2, it might be worth to use bit shifts. These opcodes are cheaper because they are more straightforward for computers.".to_string(),
                severity: Severity::Gas
            }
        )
    ]),

    fn visit_binary_operation(&mut self, bo: &mut BinaryOperation) {
        let op = &bo.operator;
        if op == &BinaryOperator::Mul || op == &BinaryOperator::Div  {
            let rhs = &bo.rhs;
            if let Expression::Literal(lit) = rhs {
                if let Some(ts) = &lit.type_descriptions.type_string {
                    let spl = ts.split("int_const ").collect::<Vec<_>>();
                    if let Some(num_str) = spl.get(1) {
                        if let Ok(num) = num_str.parse::<u128>() {
                            if is_power_of_2(num) {
                                self.push_finding(0, Some(bo.src.clone()));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[test]
fn bitwise_shifts() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity 0.8.0;

contract Mul {
    function calculateStuff(uint256 a) public {
        uint256 val ;
        val = a * 2;
        val = a * 3;
        val = a * 4;
        val = a / 4;
        val = a / 6;
    }
}",
    ));

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "maths", 0),
        vec![6, 8, 9]
    );
}
