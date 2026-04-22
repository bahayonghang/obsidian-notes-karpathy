mod common;

use common::repo_root;
use onkb::dev::doc_fragments::{
    build_installation_boundary_fragment, build_skill_inventory_fragment,
};

#[test]
fn skill_inventory_fragment_snapshot() {
    let fragment = build_skill_inventory_fragment(&repo_root()).expect("skill inventory");
    insta::assert_snapshot!("skill_inventory_fragment", fragment);
}

#[test]
fn installation_boundary_fragment_snapshot() {
    let fragment =
        build_installation_boundary_fragment(&repo_root()).expect("installation boundary");
    insta::assert_snapshot!("installation_boundary_fragment", fragment);
}
