#[test]
fn align_strech_should_size_based_on_parent() {
    let mut stretch = stretch::Stretch::new();
    let node000: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20f32),
                    height: stretch::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node00: stretch::node::StretchNode = stretch
        .new_node(stretch::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, vec![node000])
        .unwrap();
    let node0: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                justify_content: stretch::style::JustifyContent::Center,
                flex_grow: 0f32,
                flex_shrink: 1f32,
                ..Default::default()
            },
            vec![node00],
        )
        .unwrap();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(&node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(&node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.width, 20f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().size.width, 20f32);
    assert_eq!(stretch.layout(&node00).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node000).unwrap().size.width, 20f32);
    assert_eq!(stretch.layout(&node000).unwrap().size.height, 20f32);
    assert_eq!(stretch.layout(&node000).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node000).unwrap().location.y, 0f32);
}
