use crate::build_visitor;

build_visitor! {
    BTreeMap::from([
        (0,
        FindingKey{
            summary: "Events are missing sender information".to_string(),
            description: "It's generally a good idea to include the public address of the sender of the call in which the event emitted because it may become hard to track off chain without any other information.".to_string(),
            severity: Severity::Informal,
        }),
        (1,
        FindingKey{
            summary: "Event is missing indexed parameters".to_string(),
            description: "It is usually a good idea to use indexed events since they can be fetched much easier using filtering off chain at a slightly higher gas cost.".to_string(),
            severity: Severity::Informal,
        })
    ]),

    fn visit_event_definition(&mut self, ed: &mut EventDefinition) {
        if ed.parameters.parameters.iter().any(|p| !p.indexed) {
            self.push_finding(1, Some(ed.src.clone()));
        };
        Ok(())
    },

    fn visit_emit_statement(&mut self, es: &mut EmitStatement) {
        let args = &es.event_call.arguments;
        let emits_sender = args.iter().any(|arg| {
            if let Expression::MemberAccess(ma) = arg {
                if let Expression::Identifier(id) = &ma.expression {
                    let td = id.type_descriptions.clone();
                    if td.type_string == Some(String::from("msg")) && ma.member_name == "sender" {
                        return true
                    }
                }
            }

            false
        });

        if !emits_sender {
            self.push_finding(0, Some(es.src.clone()));
        }

        Ok(())
    }
}

#[test]
fn event_no_sender() {
    let findings = compile_contract_and_get_findings(String::from(
        "pragma solidity 0.8.0;

contract Event {
    event SomeThing(uint256);
    event Other(address);
    event Other_Ind(address indexed);

    function emit_things() public {
        emit SomeThing(0);
        emit Other(msg.sender);
    }
}
",
    ));

    assert_eq!(
        lines_for_findings_with_code_module(&findings, "events", 0),
        [9]
    );
    assert_eq!(
        lines_for_findings_with_code_module(&findings, "events", 1),
        [4, 5]
    );
}
