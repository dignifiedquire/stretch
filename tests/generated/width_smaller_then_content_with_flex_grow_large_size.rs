#[test]
fn width_smaller_then_content_with_flex_grow_large_size() {
    let mut stretch = stretch::Stretch::new();
    let node00: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(70f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node0: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1f32,
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(0f32), ..Default::default() },
                ..Default::default()
            },
            vec![node00],
        )
        .unwrap();
    let node10: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node1: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1f32,
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(0f32), ..Default::default() },
                ..Default::default()
            },
            vec![node10],
        )
        .unwrap();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0, node1],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(&node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(&node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().size.width, 70f32);
    assert_eq!(stretch.layout(&node00).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node1).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(&node1).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node1).unwrap().location.x, 50f32);
    assert_eq!(stretch.layout(&node1).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node10).unwrap().size.width, 20f32);
    assert_eq!(stretch.layout(&node10).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node10).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node10).unwrap().location.y, 0f32);
}
