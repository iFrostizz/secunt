// Check for non-compliant code (e.g:
// Using safeApprove instead of ... : https://code4rena.com/reports/2022-06-badger/#n-01-safeapprove-is-deprecated

use crate::build_visitor;

build_visitor! {
    BTreeMap::from([
        (
            0,
            FindingKey {
                summary: "Unsafe ERC20 operation".to_string(),
                description: "Unsafe ERC20 operation(s), use `safeTransfer` instead" .to_string(),
                severity: Severity::Medium,
            }
        ),
        (
            1,
            FindingKey {
                summary: "Unsafe ERC20 operation".to_string(),
                description: "Unsafe ERC20 operation(s), use `safeIncreaseAllowance` instead" .to_string(),
                severity: Severity::Medium,
            }
        ),
        (
            2,
            FindingKey {
                summary: "Unsafe ERC20 operation".to_string(),
                description: "Unsafe ERC20 operation(s), use `safeTransfer` instead" .to_string(),
                severity: Severity::Low,
            }
        ),
        (
            3,
            FindingKey {
                summary: "Return values of transfer()/transferFrom() not checked".to_string(),
                description: "Not all IERC20 implementations revert() when there's a failure in transfer()/transferFrom(). The function signature has a boolean return value and they indicate errors that way instead. By not checking the return value, operations that should have marked as failed, may potentially go through without actually making a payment".to_string(),
                severity: Severity::Low
            }
        )
    ]),

    fn visit_variable_declaration_statement(&mut self, var_dec: &mut VariableDeclarationStatement) {
        let init_value = &var_dec.initial_value;
        if let Some(Expression::FunctionCall(func_call)) = init_value {
            if func_call.type_descriptions.type_string != Some(String::from("bool")) {
                if let Expression::MemberAccess(mem_acc) = &func_call.expression {
                    let findings = check_member(*mem_acc.clone());
                    if findings.iter().any(|f| f.0 == 0) {
                        self.push_finding(3, Some(var_dec.src.clone()));
                    }
                }
            }
        }

        var_dec.visit(self)
    },

    fn visit_member_access(&mut self, member_access: &mut MemberAccess) {
        check_member(member_access.clone()).iter().for_each(|f| {
            self.push_finding(f.0, f.1.clone());
        });

        member_access.visit(self)
    }
}

fn check_member(member_access: MemberAccess) -> Vec<(usize, Option<SourceLocation>)> {
    let mut findings = Vec::new();

    let unsafe_ops = vec![
        "transfer".to_owned(),
        "transferFrom".to_owned(),
        "approve".to_owned(),
    ];
    let mem_name = &member_access.member_name;
    let type_d = &member_access.type_descriptions;

    if mem_name == "transfer" {
        if let Some(type_string) = &type_d.type_string {
            if type_string.starts_with("function (address,uint256)") {
                findings.push((0, Some(member_access.src.clone())));
            }
        }
    }

    if (unsafe_ops).contains(mem_name) {
    } else if mem_name == "safeApprove" {
        findings.push((1, Some(member_access.src.clone())));
    }

    findings
}

#[test]
fn usage_of_transfer() {
    let findings = compile_and_get_findings(vec![ProjectFile::Contract(
        String::from("SafeTransfer"),
        String::from(
            "pragma solidity 0.8.0;

interface IERC20 {
  function transfer(address, uint256) external view returns (bool);
}

contract SafeTransfer {
    address immutable owner = msg.sender;

    function pull(IERC20 token) public view returns (int256) {
        token.transfer(owner, 100);
    }
}",
        ),
    )]);

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "erc20", 0),
        vec![11]
    );
    // TODO: detect when no variable declaration before
    // assert_eq!(
    //     lines_for_findings_with_code_module(&findings, "erc20", 3),
    //     vec![11]
    // );
}

// https://github.com/Picodes/4naly3er/blob/main/src/issues/L/deprecatedFunctions.ts
#[test]
fn deprecated_safe_approve() {
    let findings = compile_and_get_findings(vec![ProjectFile::Contract(
        String::from("SafeApprove"),
        String::from(
            "pragma solidity ^0.8.0;

interface IERC20 {
    function safeApprove(address, uint256) external;
}

contract SafeApprove {
    function approve(IERC20 token) public {
        token.safeApprove(address(0), 123456); 
    }
}",
        ),
    )]);

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "erc20", 1),
        vec![9]
    );
}

#[test]
fn eth_transfer() {
    let findings = compile_and_get_findings(vec![ProjectFile::Contract(
        String::from("EthTransfer"),
        String::from(
            "pragma solidity ^0.8.0;

contract EthTransfer {
    function ethTransfer() public {
        payable(msg.sender).transfer(address(this).balance);
    }
}",
        ),
    )]);

    assert!(!has_with_code(&findings, "erc20", 0));
    assert!(!has_with_code(&findings, "erc20", 3));
}

#[test]
fn safe_transfer() {
    let findings = compile_and_get_findings(vec![ProjectFile::Contract(
        String::from("SafeTransfer"),
        String::from(
            "pragma solidity 0.8.0;

interface IERC20 {
  function safeTransfer(address, uint256) external view returns (bool);
}

contract SafeTransfer {
    address immutable owner = msg.sender;

    function pull(IERC20 token) public view returns (int256) {
        token.safeTransfer(owner, 100);
    }
}",
        ),
    )]);

    assert!(!has_with_code(&findings, "erc20", 0));
}

#[test]
fn transfer_ret_checked() {
    let findings = compile_and_get_findings(vec![ProjectFile::Contract(
        String::from("SafeTransfer"),
        String::from(
            "pragma solidity 0.8.0;

interface IERC20 {
  function transfer(address, uint256) external view returns (bool);
}

contract SafeTransfer {
    address immutable owner = msg.sender;

    function pull(IERC20 token) public view returns (int256) {
        require(token.transfer(owner, 100));
        bool ret = token.transfer(owner, 100);
        require(ret);
    }
}",
        ),
    )]);

    assert!(!has_with_code(&findings, "erc20", 3));
}
