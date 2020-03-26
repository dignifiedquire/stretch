#[test]
fn align_flex_start_with_shrinking_children_with_stretch() {
    let mut stretch = stretch::Stretch::new();
    let node000: stretch::node::StretchNode = stretch
        .new_node(stretch::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, vec![])
        .unwrap();
    let node00: stretch::node::StretchNode = stretch
        .new_node(stretch::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, vec![node000])
        .unwrap();
    let node0: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style { align_items: stretch::style::AlignItems::FlexStart, ..Default::default() },
            vec![node00],
        )
        .unwrap();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(500f32),
                    height: stretch::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(&node).unwrap().size.width, 500f32);
    assert_eq!(stretch.layout(&node).unwrap().size.height, 500f32);
    assert_eq!(stretch.layout(&node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.width, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.height, 500f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().size.width, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().size.height, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node000).unwrap().size.width, 0f32);
    assert_eq!(stretch.layout(&node000).unwrap().size.height, 0f32);
    assert_eq!(stretch.layout(&node000).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node000).unwrap().location.y, 0f32);
}
