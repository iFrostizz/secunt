use crate::build_visitor;

build_visitor! {
    BTreeMap::from([
        (
            0,
            FindingKey {
                summary: "Don't use payable.transfer()/payable.send()".to_string(),
                description: "These functions should never be used because they may cause a DOS state".to_string(),
                severity: Severity::Low
            }
        )
    ]),

    fn visit_member_access(&mut self, mem: &mut MemberAccess) {
        let ts = mem.type_descriptions.type_identifier.clone().unwrap_or_default();
        let ts = ts.as_str();

        if ts == "t_function_send_nonpayable$_t_uint256_$returns$_t_bool_$" || ts == "t_function_transfer_nonpayable$_t_uint256_$returns$__$" {
            self.push_finding(0, Some(mem.src.clone()));
        }
        Ok(())
    }
}

#[test]
fn deprecated_send_transfer() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity 0.8.0;

contract Depreacted {
    function give() public {
        payable(msg.sender).transfer(1 ether);
        payable(msg.sender).send(1 ether);
    }
}",
    ));

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "deprecated", 0),
        [5, 6]
    );
}

#[test]
fn token_transfer() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity 0.8.0;

interface IERC20 {
    function transfer(address, uint256) external returns (bool);
}

contract Depreacted {
    function give(IERC20 tok) public {
        tok.transfer(msg.sender, 1 ether);
    }
}",
    ));

    assert!(!has_with_code(&findings, "deprecated", 0));
}
