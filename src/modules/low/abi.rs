use crate::build_visitor;

build_visitor! {
    BTreeMap::from([
        (
            0,
            FindingKey {
                summary: "Use `abi.encodeCall() instead of `abi.encodeWithSignature()` or `abi.encodeWithSelector()`".to_string(),
                description: "Since 0.8.11, abi.encodeCall provide type-safe encode utility comparing with `abi.encodeWithSelector`. `abi.encodeWithSelector` can use with interface.<function>.selector to prevent typo error, but it doesn't provide type checking.".to_string(),
                severity: Severity::Low,
            }
        )
    ]),

    fn visit_member_access(&mut self, ma: &mut MemberAccess) {
        if let Expression::Identifier(expr) = &ma.expression {
            let td = expr.type_descriptions.clone();
            let mn = &ma.member_name;
            if td.type_string == Some(String::from("abi")) && (mn == "encodeWithSignature" || mn == "encodeWithSelector") {
                self.push_finding(0, Some(ma.src.clone()));
            }
        }

        ma.visit(self)
    }
}

#[test]
fn use_encode_call() {
    let findings = compile_contract_and_get_findings(String::from(
        r#"pragma solidity 0.8.17;

interface IERC20 {
    function transfer(address, uint256) external;
}

contract EncodeCall {
    function doThings(address to, uint256 amount) external {
        bytes memory a = abi.encodeCall(IERC20.transfer, (to, amount)); 
        bytes memory b = abi.encodeWithSelector(IERC20.transfer.selector, to, amount); 
        bytes memory c = abi.encodeWithSignature("transfer(address,uint256)", to, amount); 
    }
}"#,
    ));

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "abi", 0),
        [10, 11]
    );
}
