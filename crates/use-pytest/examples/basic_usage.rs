use use_pytest::{PytestMarkerName, PytestNodeId, PytestOutcome};

fn main() -> Result<(), use_pytest::PytestNameError> {
    let marker = PytestMarkerName::new("slow")?;
    let node_id = PytestNodeId::new("tests/test_app.py::test_smoke")?;

    assert_eq!(marker.as_str(), "slow");
    assert!(node_id.has_scope_separator());
    assert_eq!(PytestOutcome::Passed.as_str(), "passed");
    Ok(())
}
